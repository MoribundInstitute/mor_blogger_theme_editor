//! GitHub OAuth Device Flow client + Fork-and-Pull-Request submission workflow.
//!
//! Intended flow for a desktop user:
//!   1. Call `request_device_code` → show `user_code` + `verification_uri` in the UI.
//!   2. Call `poll_for_token` on each interval tick until `Ok(token)` is returned.
//!   3. Construct a `GitHubClient` with the resulting token.
//!   4. Call `submit_pull_request` — this forks the upstream repo to the user's account,
//!      commits the module XML to the fork, then opens a PR against the upstream `main`
//!      branch.  Direct commits to the upstream are intentionally not supported.

use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use serde::{Deserialize, Serialize};
use std::{fmt, time::Duration};

const API_ROOT: &str = "https://api.github.com";

// Maximum attempts and wait per poll cycle when waiting for a fork to become ready.
// GitHub's fork operation is async (202 Accepted); the repo typically appears within
// 10-20 s, so 12 × 3 s = 36 s covers almost all real-world cases.
const FORK_POLL_ATTEMPTS: u32 = 12;
const FORK_POLL_INTERVAL: Duration = Duration::from_secs(3);

// ─── Error type ──────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum GitHubError {
    Http(reqwest::Error),
    /// Device-code flow: user has not yet authorised; caller must wait `interval` seconds.
    AuthorizationPending,
    /// User explicitly denied the authorisation request.
    AccessDenied,
    /// Poll interval is too short; caller must back off before retrying.
    SlowDown,
    /// Device code expired; restart the flow from `request_device_code`.
    ExpiredToken,
    /// Fork creation returned an unexpected error status.
    ForkFailed { message: String },
    /// Fork was queued but did not become available within the polling window.
    ForkTimeout,
    /// A pull request from this fork+branch already exists upstream.
    PullRequestDuplicate,
    /// PR creation failed for a reason other than duplication.
    PullRequestFailed { message: String },
    /// Catch-all for unexpected GitHub API responses.
    Api { status: u16, message: String },
}

impl fmt::Display for GitHubError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(e) => write!(f, "HTTP error: {}", e),
            Self::AuthorizationPending => write!(f, "Authorization pending — retry after interval"),
            Self::AccessDenied => write!(f, "Access denied by user"),
            Self::SlowDown => write!(f, "Polling too fast — increase interval"),
            Self::ExpiredToken => write!(f, "Device code expired — restart flow"),
            Self::ForkFailed { message } => write!(f, "Fork creation failed: {}", message),
            Self::ForkTimeout => write!(f, "Fork did not become ready within the polling window"),
            Self::PullRequestDuplicate => write!(f, "A pull request from this fork already exists"),
            Self::PullRequestFailed { message } => write!(f, "PR creation failed: {}", message),
            Self::Api { status, message } => write!(f, "GitHub API {}: {}", status, message),
        }
    }
}

impl std::error::Error for GitHubError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Self::Http(e) = self { Some(e) } else { None }
    }
}

impl From<reqwest::Error> for GitHubError {
    fn from(e: reqwest::Error) -> Self {
        Self::Http(e)
    }
}

// ─── OAuth Device Flow DTOs ───────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    /// Short code the user types at `verification_uri`.
    pub user_code: String,
    /// URL to display, e.g. `https://github.com/login/device`.
    pub verification_uri: String,
    pub expires_in: u64,
    /// Minimum seconds between poll attempts.
    pub interval: u64,
}

#[derive(Deserialize)]
struct TokenPollResponse {
    access_token: Option<String>,
    error: Option<String>,
}

// ─── Internal API DTOs ────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct AuthUser {
    login: String,
}

#[derive(Deserialize)]
struct ForkInfo {
    full_name: String,
    default_branch: String,
}

#[derive(Serialize)]
struct PutFileRequest {
    message: String,
    /// Base64-encoded file content.
    content: String,
    /// Required when updating an existing file; omit on first create.
    #[serde(skip_serializing_if = "Option::is_none")]
    sha: Option<String>,
    /// Target branch on the fork.
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<String>,
}

#[derive(Deserialize)]
struct PutFileResponse {
    content: Option<FileMeta>,
}

#[derive(Deserialize)]
struct FileMeta {
    sha: Option<String>,
}

#[derive(Serialize)]
struct CreatePrRequest {
    title: String,
    body: String,
    /// Format: `"fork_user:branch"`.
    head: String,
    /// Target branch on the upstream repo.
    base: String,
}

/// Returned by `submit_pull_request` on success.
#[derive(Debug, Deserialize)]
pub struct PullRequestResponse {
    pub number: u32,
    pub html_url: String,
}

// ─── Authenticated client ─────────────────────────────────────────────────────

#[derive(Clone)]
pub struct GitHubClient {
    token: String,
    client: reqwest::Client,
}

impl GitHubClient {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            client: reqwest::Client::builder()
                .user_agent(concat!("MorBloggerThemeEditor/", env!("CARGO_PKG_VERSION")))
                .build()
                .unwrap_or_default(),
        }
    }

    // ── Public workflow ──────────────────────────────────────────────────────

    /// Zero-trust community submission workflow.
    ///
    /// Steps:
    ///   1. Resolve the authenticated user's GitHub login via `GET /user`.
    ///   2. Fork `{upstream_owner}/{repo}` to the user's account (idempotent — GitHub
    ///      returns 200 if the fork already exists).
    ///   3. Poll `GET /repos/{user}/{repo}` until the fork responds with 200 (the fork
    ///      API is asynchronous; GitHub returns 202 Accepted before the repo is usable).
    ///   4. Commit `content` to `community/{category}/{name}` on the fork's default branch.
    ///   5. Open a pull request from `{user}:{branch}` to `{upstream_owner}:{repo}:main`.
    ///
    /// Returns the created [`PullRequestResponse`] (PR number + URL) on success.
    pub async fn submit_pull_request(
        &self,
        upstream_owner: &str,
        repo: &str,
        category: &str,
        name: &str,
        content: &str,
    ) -> Result<PullRequestResponse, GitHubError> {
        let safe_name = sanitize_path_component(name);
        let file_path = format!("community/{}/{}", category, safe_name);

        let user_login = self.get_authenticated_user().await?;
        log::info!("[github] Authenticated as: {}", user_login);

        let fork = self.fork_repository(upstream_owner, repo).await?;
        log::info!("[github] Fork: {}", fork.full_name);

        self.await_fork_ready(&user_login, repo).await?;

        self.commit_to_fork(
            &user_login,
            repo,
            &fork.default_branch,
            &file_path,
            category,
            &safe_name,
            content,
        )
        .await?;

        let pr = self
            .open_pull_request(
                upstream_owner,
                repo,
                &user_login,
                &fork.default_branch,
                category,
                &safe_name,
            )
            .await?;

        log::info!("[github] PR #{} opened: {}", pr.number, pr.html_url);
        Ok(pr)
    }

    // ── Private steps ────────────────────────────────────────────────────────

    async fn get_authenticated_user(&self) -> Result<String, GitHubError> {
        let resp = self
            .authed_get(&format!("{}/user", API_ROOT))
            .send()
            .await?;
        let status = resp.status().as_u16();
        if status == 200 {
            let user: AuthUser = resp.json().await?;
            Ok(user.login)
        } else {
            let message = resp.text().await.unwrap_or_default();
            Err(GitHubError::Api { status, message })
        }
    }

    async fn fork_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<ForkInfo, GitHubError> {
        let url = format!("{}/repos/{}/{}/forks", API_ROOT, owner, repo);
        let resp = self
            .authed_post(&url)
            .json(&serde_json::json!({}))
            .send()
            .await?;

        let status = resp.status().as_u16();
        // 202 = fork queued (async creation), 200 = fork already exists
        if status == 202 || status == 200 {
            let fork: ForkInfo = resp.json().await?;
            Ok(fork)
        } else {
            let message = resp.text().await.unwrap_or_default();
            Err(GitHubError::ForkFailed { message })
        }
    }

    async fn await_fork_ready(&self, user: &str, repo: &str) -> Result<(), GitHubError> {
        let url = format!("{}/repos/{}/{}", API_ROOT, user, repo);
        for attempt in 0..FORK_POLL_ATTEMPTS {
            let resp = self.authed_get(&url).send().await?;
            if resp.status().as_u16() == 200 {
                log::info!("[github] Fork ready after {} poll(s)", attempt + 1);
                return Ok(());
            }
            if attempt < FORK_POLL_ATTEMPTS - 1 {
                tokio::time::sleep(FORK_POLL_INTERVAL).await;
            }
        }
        Err(GitHubError::ForkTimeout)
    }

    async fn commit_to_fork(
        &self,
        fork_user: &str,
        repo: &str,
        branch: &str,
        file_path: &str,
        category: &str,
        name: &str,
        content: &str,
    ) -> Result<(), GitHubError> {
        let url = format!(
            "{}/repos/{}/{}/contents/{}",
            API_ROOT, fork_user, repo, file_path
        );
        let existing_sha = self.fetch_file_sha(&url).await.ok().flatten();

        let body = PutFileRequest {
            message: format!("community: add {}/{}", category, name),
            content: B64.encode(content.as_bytes()),
            sha: existing_sha,
            branch: Some(branch.to_string()),
        };

        let resp = self.authed_put(&url).json(&body).send().await?;
        let status = resp.status().as_u16();
        if status == 200 || status == 201 {
            log::info!("[github] Committed {} to fork/{}", file_path, fork_user);
            Ok(())
        } else {
            let message = resp.text().await.unwrap_or_default();
            Err(GitHubError::Api { status, message })
        }
    }

    async fn open_pull_request(
        &self,
        upstream_owner: &str,
        repo: &str,
        fork_user: &str,
        fork_branch: &str,
        category: &str,
        name: &str,
    ) -> Result<PullRequestResponse, GitHubError> {
        let url = format!("{}/repos/{}/{}/pulls", API_ROOT, upstream_owner, repo);

        let body = CreatePrRequest {
            title: format!("[Community] Add {}/{}", category, name),
            body: format!(
                "Community template module submission from MorBlogger Theme Editor.\n\n\
                 - **Category:** `{}`\n\
                 - **Module:** `{}`\n\n\
                 Please review and merge if the module meets the compendium standards.",
                category, name
            ),
            head: format!("{}:{}", fork_user, fork_branch),
            base: "main".to_string(),
        };

        let resp = self.authed_post(&url).json(&body).send().await?;
        let status = resp.status().as_u16();

        if status == 201 {
            Ok(resp.json::<PullRequestResponse>().await?)
        } else if status == 422 {
            let text = resp.text().await.unwrap_or_default();
            // GitHub signals a duplicate PR with 422 + message containing this phrase.
            if text.to_ascii_lowercase().contains("pull request already exists") {
                Err(GitHubError::PullRequestDuplicate)
            } else {
                Err(GitHubError::PullRequestFailed { message: text })
            }
        } else {
            let message = resp.text().await.unwrap_or_default();
            Err(GitHubError::Api { status, message })
        }
    }

    /// Returns the current blob SHA for a file, or `None` if the file does not exist.
    async fn fetch_file_sha(&self, url: &str) -> Result<Option<String>, GitHubError> {
        let resp = self.authed_get(url).send().await?;
        if resp.status().as_u16() == 200 {
            let meta: PutFileResponse = resp.json().await?;
            Ok(meta.content.and_then(|m| m.sha))
        } else {
            Ok(None)
        }
    }

    // ── Request builder helpers ───────────────────────────────────────────────

    fn authed_get(&self, url: &str) -> reqwest::RequestBuilder {
        self.client
            .get(url)
            .bearer_auth(&self.token)
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
    }

    fn authed_post(&self, url: &str) -> reqwest::RequestBuilder {
        self.client
            .post(url)
            .bearer_auth(&self.token)
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
    }

    fn authed_put(&self, url: &str) -> reqwest::RequestBuilder {
        self.client
            .put(url)
            .bearer_auth(&self.token)
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
    }
}

// ─── OAuth Device Flow helpers ────────────────────────────────────────────────

/// Step 1 of the Device Authorization Grant: request a device + user code.
/// Show `response.user_code` and `response.verification_uri` in the UI, then
/// poll `poll_for_token` at `response.interval`-second intervals.
pub async fn request_device_code(client_id: &str) -> Result<DeviceCodeResponse, GitHubError> {
    let client = anon_client();
    let resp = client
        .post("https://github.com/login/device/code")
        .header("Accept", "application/json")
        .form(&[("client_id", client_id), ("scope", "public_repo")])
        .send()
        .await?;

    let status = resp.status().as_u16();
    if resp.status().is_success() {
        Ok(resp.json::<DeviceCodeResponse>().await?)
    } else {
        let message = resp.text().await.unwrap_or_default();
        Err(GitHubError::Api { status, message })
    }
}

/// Step 2: Single poll attempt for an OAuth access token.
/// Returns `Ok(token)` once the user completes authorisation.
/// `AuthorizationPending` and `SlowDown` are expected transient states — keep polling.
pub async fn poll_for_token(
    client_id: &str,
    device_code: &str,
) -> Result<String, GitHubError> {
    let client = anon_client();
    let resp = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&[
            ("client_id", client_id),
            ("device_code", device_code),
            ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
        ])
        .send()
        .await?;

    let poll: TokenPollResponse = resp.json().await?;
    if let Some(token) = poll.access_token {
        return Ok(token);
    }

    Err(match poll.error.as_deref() {
        Some("authorization_pending") => GitHubError::AuthorizationPending,
        Some("slow_down") => GitHubError::SlowDown,
        Some("access_denied") => GitHubError::AccessDenied,
        Some("expired_token") => GitHubError::ExpiredToken,
        other => GitHubError::Api {
            status: 0,
            message: other.unwrap_or("unknown_error").to_string(),
        },
    })
}

// ─── Private helpers ──────────────────────────────────────────────────────────

fn anon_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent(concat!("MorBloggerThemeEditor/", env!("CARGO_PKG_VERSION")))
        .build()
        .unwrap_or_default()
}

/// Strip path-traversal and shell-special characters from a filename component.
fn sanitize_path_component(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0' => '_',
            c => c,
        })
        .collect();
    let cleaned = cleaned.trim_start_matches('.').to_string();
    let cleaned = if cleaned.is_empty() {
        "custom_module".to_string()
    } else {
        cleaned
    };
    if cleaned.to_ascii_lowercase().ends_with(".xml") {
        cleaned
    } else {
        format!("{}.xml", cleaned)
    }
}
