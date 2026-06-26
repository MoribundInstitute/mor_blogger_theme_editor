//! Cursor asset pipeline.
//!
//! Three input sources, one output type. The point of this module is the
//! invariant: a `CursorAsset` cannot be built, and its CSS cannot be read,
//! without a `License` riding along. License binding is in the type system,
//! not in a reviewer's memory.
//!
//! - `SuiGeneris`     : we draw the SVG. License is ours (CC0).
//! - `GnomeExtraction`: we pull frames out of someone else's X11 theme. The
//!                      asset INHERITS the upstream license. No license resolved
//!                      -> no asset. Copyleft with no license text -> no asset.
//! - `LocalPath`      : user points at their own files. They assert the rights;
//!                      we record that assertion and move on.

use base64::Engine;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug)]
pub enum CursorError {
    /// Extraction asked for a repo we have no license mapping for and couldn't
    /// detect one. Hard stop — we do not scrape unlicensed assets.
    UnknownLicense(String),
    /// Copyleft license with no license text to carry. Can't bind -> refuse.
    CopyleftMissingText(String),
    /// LocalPath without the user confirming they have the rights.
    RightsNotConfirmed,
    Fetch(String),
    Convert(String),
}

impl std::fmt::Display for CursorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CursorError::UnknownLicense(r) => write!(f, "no license known or detected for '{r}'; refusing to extract"),
            CursorError::CopyleftMissingText(r) => write!(f, "'{r}' is copyleft but carries no license text; refusing to emit"),
            CursorError::RightsNotConfirmed => write!(f, "local cursor: user did not confirm distribution rights"),
            CursorError::Fetch(e) => write!(f, "fetch failed: {e}"),
            CursorError::Convert(e) => write!(f, "rasterize failed: {e}"),
        }
    }
}

/// The license an asset carries. Every CursorAsset has exactly one. Not Option.
#[derive(Debug, Clone)]
pub struct License {
    pub spdx: String,            // "GPL-3.0-only", "CC0-1.0", "LicenseRef-UserAsserted"
    pub author: String,
    pub source_url: Option<String>,
    pub commit: Option<String>,  // pin the exact source for copyleft "provide source"
    pub copyleft: bool,
    pub full_text: String,       // upstream LICENSE/COPYING verbatim. Required if copyleft.
    pub modifications: String,   // what we did to the original. GPL §5: state changes.
}

impl License {
    /// Our own work. Public domain dedication, no strings.
    fn sui_generis() -> Self {
        License {
            spdx: "CC0-1.0".into(),
            author: "MorBlogger (original work)".into(),
            source_url: None,
            commit: None,
            copyleft: false,
            full_text: String::new(),
            modifications: "original work".into(),
        }
    }
}

/// Output type. `css` is PRIVATE: the only way out is `to_css_block`, which
/// always prepends the attribution header. You cannot get the cursor without
/// the license. That is the whole design.
pub struct CursorAsset {
    pub name: String,
    css: String,
    pub license: License,
}

impl CursorAsset {
    /// The single chokepoint. Every builder routes through here, so the
    /// copyleft-needs-text rule is enforced exactly once and can't be bypassed.
    fn new_checked(name: impl Into<String>, css: impl Into<String>, license: License) -> Result<Self, CursorError> {
        let name = name.into();
        if license.copyleft && license.full_text.trim().is_empty() {
            return Err(CursorError::CopyleftMissingText(name));
        }
        Ok(CursorAsset { name, css: css.into(), license })
    }

    /// CSS for `--theme-cursor-default`, with the license welded to the front
    /// as a comment. There is no accessor that returns bare `css`.
    pub fn to_css_block(&self) -> String {
        let l = &self.license;
        let src = l.source_url.as_deref().unwrap_or("(none)");
        let commit = l.commit.as_deref().unwrap_or("(unpinned)");
        format!(
            "/* cursor: {name}\n   SPDX-License-Identifier: {spdx}\n   Author: {author}\n   Source: {src} @ {commit}\n   Modifications: {mods}\n*/\n--theme-cursor-default: {css};",
            name = self.name,
            spdx = l.spdx,
            author = l.author,
            mods = l.modifications,
            css = self.css,
        )
    }
}

pub enum CursorSource {
    SuiGeneris { name: &'static str },
    GnomeExtraction { repo: String, variant: String },
    LocalPath { path: PathBuf, declared_license: String, rights_confirmed: bool },
}

pub fn build(source: CursorSource) -> Result<CursorAsset, CursorError> {
    match source {
        CursorSource::SuiGeneris { name } => sui_generis(name),
        CursorSource::GnomeExtraction { repo, variant } => extract(&repo, &variant),
        CursorSource::LocalPath { path, declared_license, rights_confirmed } => {
            local(path, declared_license, rights_confirmed)
        }
    }
}

// --- Option 1: Sui Generis ------------------------------------------------

fn sui_generis(name: &str) -> Result<CursorAsset, CursorError> {
    // Tiny inline SVGs. utf8 data-URI, no base64, no files. fill/stroke chosen
    // per name. Add more by extending this match — that is the entire cost.
    let svg = match name {
        "arrow-ice" => svg_arrow("#ece9ff", "#4c566a"),
        "arrow-amber" => svg_arrow("#ffb300", "#ffffff"),
        "arrow-neon" => svg_arrow("#00f0ff", "#0a0a0a"),
        "arrow-ink" => svg_arrow("#111111", "#ffffff"),
        _ => svg_arrow("#ffffff", "#000000"),
    };
    let css = format!("url('data:image/svg+xml;utf8,{}') 0 0, auto", svg);
    CursorAsset::new_checked(format!("sui:{name}"), css, License::sui_generis())
}

fn svg_arrow(fill: &str, stroke: &str) -> String {
    // URL-encoded inline so it drops straight into a CSS url(). Keep it one line.
    format!(
        "%3Csvg xmlns='http://www.w3.org/2000/svg' width='24' height='24' viewBox='0 0 24 24'%3E%3Cpath d='M4 2 L4 20 L9 15 L12 22 L15 21 L12 14 L19 14 Z' fill='{}' stroke='{}' stroke-width='1.5'/%3E%3C/svg%3E",
        fill, stroke
    )
}

// --- Option 2: GNOME / X11 theme extraction -------------------------------

fn extract(repo: &str, variant: &str) -> Result<CursorAsset, CursorError> {
    // Clone once; both license resolution and rasterization need the tree. We
    // still never EMIT without a bound license (new_checked is the gate), and we
    // bail before rasterizing if the license can't be resolved.
    let tmp = clone_repo(repo)?;
    let commit = git_head(&tmp)?;
    let license_text = read_license_file(&tmp); // Option<String>

    let mut license = match resolve_license(repo) {
        // In our registry: trust the SPDX, take the text from the clone.
        Some(l) => l,
        // Not in registry: scan the LICENSE we just cloned. No file, or text we
        // can't fingerprint -> refuse. Detection only ever turns a refusal into a
        // bound license; it never loosens the gate.
        None => {
            let text = license_text
                .clone()
                .ok_or_else(|| CursorError::UnknownLicense(repo.to_string()))?;
            detect_spdx(&text).ok_or_else(|| CursorError::UnknownLicense(repo.to_string()))?
        }
    };
    license.source_url = Some(format!("https://github.com/{repo}"));
    license.commit = Some(commit);
    license.full_text = license_text.unwrap_or_default();
    license.modifications =
        format!("extracted '{variant}' default frame; rasterized via xcur2png; re-encoded base64 PNG");

    // Copyleft with no carried text is rejected inside new_checked.
    let raw = rasterize(&tmp, variant)?;
    let b64 = base64::engine::general_purpose::STANDARD.encode(&raw);
    let css = format!("url('data:image/png;base64,{b64}') 0 0, auto");
    CursorAsset::new_checked(format!("{repo}:{variant}"), css, license)
}

/// Known-license registry. `None` = not in the table; the caller then scans the
/// cloned LICENSE with `detect_spdx`. `full_text`/`commit` are filled by the
/// caller from the clone.
fn resolve_license(repo: &str) -> Option<License> {
    let known = |spdx: &str, author: &str, copyleft: bool| License {
        spdx: spdx.into(),
        author: author.into(),
        source_url: Some(format!("https://github.com/{repo}")),
        commit: None,
        copyleft,
        full_text: String::new(),
        modifications: String::new(),
    };
    match repo {
        "EliverLara/Sweet" => Some(known("GPL-3.0-only", "EliverLara", true)),
        "ful1e5/Bibata_Cursor" => Some(known("GPL-3.0-only", "ful1e5", true)),
        "keeferrourke/capitaine-cursors" => Some(known("LGPL-3.0-only", "Keefer Rourke", true)),
        _ => None,
    }
}

/// License scanner for repos outside the registry. Fingerprints the LICENSE text
/// we already cloned — no corpus crate, no network, no similarity scoring. An
/// explicit `SPDX-License-Identifier:` tag wins; otherwise match the canonical
/// title line. `None` = unrecognized, which the caller turns into a refusal.
fn detect_spdx(text: &str) -> Option<License> {
    let (spdx, copyleft) = if let Some(id) = spdx_tag(text) {
        let cl = is_copyleft(&id);
        (id, cl)
    } else if text.contains("GNU AFFERO GENERAL PUBLIC LICENSE") {
        ("AGPL-3.0-only".to_string(), true)
    } else if text.contains("GNU LESSER GENERAL PUBLIC LICENSE") {
        ("LGPL-3.0-only".to_string(), true)
    } else if text.contains("GNU GENERAL PUBLIC LICENSE") {
        if text.contains("Version 3") {
            ("GPL-3.0-only".to_string(), true)
        } else if text.contains("Version 2") {
            ("GPL-2.0-only".to_string(), true)
        } else {
            return None;
        }
    } else if text.contains("Mozilla Public License Version 2.0") {
        ("MPL-2.0".to_string(), true)
    } else if text.contains("Apache License") && text.contains("Version 2.0") {
        ("Apache-2.0".to_string(), false)
    } else if text.contains("MIT License")
        || text.contains("Permission is hereby granted, free of charge")
    {
        ("MIT".to_string(), false)
    } else if text.contains("CC0") || text.contains("Creative Commons Zero") {
        ("CC0-1.0".to_string(), false)
    } else if text.contains("This is free and unencumbered software released into the public domain") {
        ("Unlicense".to_string(), false)
    } else {
        return None;
    };
    Some(License {
        spdx,
        author: "(detected from LICENSE)".into(),
        source_url: None,
        commit: None,
        copyleft,
        full_text: String::new(),
        modifications: String::new(),
    })
}

/// Pull the token after an `SPDX-License-Identifier:` tag, if present.
fn spdx_tag(text: &str) -> Option<String> {
    let i = text.find("SPDX-License-Identifier:")?;
    let rest = &text[i + "SPDX-License-Identifier:".len()..];
    let id = rest.lines().next()?.trim().trim_end_matches('*').trim();
    (!id.is_empty()).then(|| id.to_string())
}

/// Copyleft families. Errs toward `true` — a false "copyleft" only forces us to
/// carry the license text we already have; a false "permissive" would strip
/// obligations, so when unsure we treat it as copyleft.
fn is_copyleft(spdx: &str) -> bool {
    let s = spdx.to_ascii_uppercase();
    s.starts_with("GPL")
        || s.starts_with("AGPL")
        || s.starts_with("LGPL")
        || s.starts_with("MPL")
        || s.starts_with("EPL")
        || s.contains("CC-BY-SA")
}

fn clone_repo(repo: &str) -> Result<PathBuf, CursorError> {
    let tmp = std::env::temp_dir().join(format!("mor-cursor-{}", repo.replace('/', "_")));
    run(Command::new("git")
        .args(["clone", "--depth", "1", &format!("https://github.com/{repo}.git")])
        .arg(&tmp))
        .map_err(CursorError::Fetch)?;
    Ok(tmp)
}

fn git_head(tmp: &PathBuf) -> Result<String, CursorError> {
    // Pin the source so copyleft "provide the corresponding source" is honest.
    run(Command::new("git").arg("-C").arg(tmp).args(["rev-parse", "HEAD"]))
        .map(|s| s.trim().to_string())
        .map_err(CursorError::Fetch)
}

fn read_license_file(tmp: &PathBuf) -> Option<String> {
    for candidate in ["LICENSE", "COPYING", "LICENSE.txt", "COPYING.txt", "LICENSE.md"] {
        if let Ok(text) = std::fs::read_to_string(tmp.join(candidate)) {
            return Some(text);
        }
    }
    None
}

/// Seam: rasterize the `variant` cursor's default frame to PNG. xcur2png's exact
/// in-theme path is layout-dependent; this is the one wired-in tool call.
fn rasterize(tmp: &PathBuf, variant: &str) -> Result<Vec<u8>, CursorError> {
    let png = tmp.join(format!("{variant}.png"));
    run(Command::new("xcur2png")
        .arg(tmp.join("cursors").join(variant))
        .arg("-d")
        .arg(tmp))
        .map_err(CursorError::Convert)?;
    std::fs::read(&png).map_err(|e| CursorError::Convert(e.to_string()))
}

fn run(cmd: &mut Command) -> Result<String, String> {
    let out = cmd.output().map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).into_owned());
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

// --- Option 3: Bring Your Own ---------------------------------------------

fn local(path: PathBuf, declared_license: String, rights_confirmed: bool) -> Result<CursorAsset, CursorError> {
    // We don't know what this is or who made it. The user asserts the rights;
    // we record the assertion verbatim and stamp it LicenseRef so it's never
    // mistaken for cleared. No confirmation -> no asset.
    if !rights_confirmed {
        return Err(CursorError::RightsNotConfirmed);
    }
    let bytes = std::fs::read(&path).map_err(|e| CursorError::Fetch(e.to_string()))?;
    let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    let mime = if path.extension().and_then(|e| e.to_str()) == Some("svg") {
        "image/svg+xml"
    } else {
        "image/png"
    };
    let css = format!("url('data:{mime};base64,{b64}') 0 0, auto");
    let license = License {
        spdx: format!("LicenseRef-UserAsserted: {}", declared_license.trim()),
        author: "user-provided".into(),
        source_url: Some(path.display().to_string()),
        commit: None,
        copyleft: false, // user's problem to declare; we don't infer copyleft here
        full_text: String::new(),
        modifications: "supplied by user; embedded verbatim".into(),
    };
    CursorAsset::new_checked(format!("local:{}", path.display()), css, license)
}

// --- Credits ledger -------------------------------------------------------

/// Emits the attribution block that ships WITH the theme. Copyleft assets get
/// their full license text inlined — that's the "preserve notices" obligation.
/// Call this whenever any non-sui-generis asset is in the export.
pub fn write_credits(assets: &[CursorAsset]) -> String {
    let mut out = String::from("# Third-party cursor assets\n\n");
    for a in assets {
        let l = &a.license;
        out.push_str(&format!(
            "## {}\n- SPDX: {}\n- Author: {}\n- Source: {} @ {}\n- Modifications: {}\n",
            a.name,
            l.spdx,
            l.author,
            l.source_url.as_deref().unwrap_or("(none)"),
            l.commit.as_deref().unwrap_or("(unpinned)"),
            l.modifications,
        ));
        if l.copyleft && !l.full_text.trim().is_empty() {
            out.push_str("\n<details><summary>License text</summary>\n\n```\n");
            out.push_str(l.full_text.trim());
            out.push_str("\n```\n</details>\n");
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn license_is_structurally_bound() {
        // Unknown repo: not in registry (caller then scans the clone).
        assert!(resolve_license("nobody/unknown").is_none());
        assert!(resolve_license("EliverLara/Sweet").is_some());

        // Copyleft with no carried text: cannot emit.
        let bare = License { full_text: String::new(), ..known_gpl() };
        assert!(matches!(
            CursorAsset::new_checked("x", "auto", bare),
            Err(CursorError::CopyleftMissingText(_))
        ));

        // Copyleft WITH text: ok, and the license rides in the CSS output.
        let ok = CursorAsset::new_checked("sweet:left_ptr", "url('...'), auto", known_gpl()).unwrap();
        let block = ok.to_css_block();
        assert!(block.contains("SPDX-License-Identifier: GPL-3.0-only"));
        assert!(block.contains("github.com/EliverLara/Sweet"));

        // Sui generis: self-licensed CC0, always emits.
        let s = build(CursorSource::SuiGeneris { name: "arrow-ice" }).unwrap();
        assert!(s.to_css_block().contains("CC0-1.0"));

        // BYO without rights confirmation: refused.
        assert!(matches!(
            build(CursorSource::LocalPath {
                path: "/nope.png".into(),
                declared_license: "MIT".into(),
                rights_confirmed: false,
            }),
            Err(CursorError::RightsNotConfirmed)
        ));
    }

    #[test]
    fn scanner_fingerprints_and_refuses() {
        // GPLv3 title -> detected, copyleft.
        let gpl = detect_spdx("GNU GENERAL PUBLIC LICENSE\nVersion 3, 29 June 2007").unwrap();
        assert_eq!(gpl.spdx, "GPL-3.0-only");
        assert!(gpl.copyleft);

        // Explicit SPDX tag wins over title heuristics.
        let tagged = detect_spdx("// SPDX-License-Identifier: MIT\n...").unwrap();
        assert_eq!(tagged.spdx, "MIT");
        assert!(!tagged.copyleft);

        // MIT body -> permissive.
        let mit = detect_spdx("Permission is hereby granted, free of charge, to any person").unwrap();
        assert_eq!(mit.spdx, "MIT");

        // Unrecognized -> None, which the caller turns into a refusal.
        assert!(detect_spdx("some random readme, not a license").is_none());

        // Copyleft classifier errs toward true.
        assert!(is_copyleft("LGPL-3.0-only"));
        assert!(is_copyleft("CC-BY-SA-4.0"));
        assert!(!is_copyleft("Apache-2.0"));
    }

    fn known_gpl() -> License {
        License {
            spdx: "GPL-3.0-only".into(),
            author: "EliverLara".into(),
            source_url: Some("https://github.com/EliverLara/Sweet".into()),
            commit: Some("abc123".into()),
            copyleft: true,
            full_text: "GNU GENERAL PUBLIC LICENSE Version 3 ...".into(),
            modifications: "test".into(),
        }
    }
}
