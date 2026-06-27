use crate::config::AboutPageConfig;
use crate::render::pages::escape_html;

/// Emits the About page as pure layout.
/// Upgraded to a 2-Column "Instructor / Author Profile" aesthetic.
pub fn generate_about_html(config: &AboutPageConfig) -> String {
    let mut html = String::new();

    // Chained CSS fallbacks: var(--theme-*, var(--old-stencil-token, #hex))
    html.push_str(
        r##"<style>
        .mor-about-container {
            display: grid;
            grid-template-columns: 320px 1fr;
            gap: 48px;
            max-width: 1100px;
            margin: 0 auto;
            color: var(--theme-text, var(--fg-base, #cccccc));
            font-family: var(--theme-font, system-ui, sans-serif);
            align-items: start;
        }

        /* Sticky Profile Card */
        .mor-about-sidebar {
            background: var(--theme-bg-panel, var(--bg-panel, #252525));
            border: 1px solid var(--theme-border, var(--border-color, #333));
            border-radius: 8px;
            padding: 40px 32px;
            text-align: center;
            position: sticky;
            top: 40px;
            display: flex;
            flex-direction: column;
            align-items: center;
        }

        .mor-about-avatar {
            width: 180px;
            height: 180px;
            border-radius: 50%;
            object-fit: cover;
            border: 4px solid var(--theme-bg-panel, var(--bg-panel, #252525));
            outline: 2px solid var(--theme-accent, var(--accent, #60cdff));
            outline-offset: 4px;
            margin-bottom: 24px;
            box-shadow: 0 12px 24px rgba(0, 0, 0, 0.4);
            background-color: var(--theme-bg-elevated, var(--bg-panel, #1e1e1e));
        }

        .mor-about-title-block h1 {
            margin: 0 0 8px 0;
            color: var(--theme-text, var(--fg-base, #ffffff));
            font-size: 2rem;
            line-height: 1.2;
            letter-spacing: -0.5px;
        }

        .mor-about-kicker {
            font-size: 0.9rem;
            text-transform: uppercase;
            letter-spacing: 2px;
            color: var(--theme-accent, var(--accent, #60cdff));
            margin-bottom: 32px;
            font-weight: 600;
        }

        /* Social Links */
        .mor-about-links {
            display: flex;
            flex-direction: column;
            gap: 12px;
            width: 100%;
            border-top: 1px solid var(--theme-border-light, var(--border-color, #444));
            padding-top: 32px;
        }

        .mor-about-link {
            background: transparent;
            color: var(--theme-text, var(--fg-base, #ccc));
            border: 1px solid var(--theme-border, var(--border-color, #444));
            padding: 12px 16px;
            text-decoration: none;
            border-radius: 6px;
            font-size: 0.95rem;
            transition: all 0.15s ease;
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 500;
        }

        .mor-about-link:hover {
            background: var(--theme-bg-elevated, var(--bg-panel, #333));
            color: var(--theme-accent, var(--accent, #60cdff));
            border-color: var(--theme-accent, var(--accent, #60cdff));
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
        }

        .mor-about-link.email-link {
            background: rgba(96, 205, 255, 0.1);
            color: var(--theme-accent, var(--accent, #60cdff));
            border-color: rgba(96, 205, 255, 0.3);
            font-weight: bold;
        }

        .mor-about-link.email-link:hover {
            background: var(--theme-accent, var(--accent, #60cdff));
            color: #111;
        }

        /* Biography Content */
        .mor-about-content {
            padding-top: 16px;
        }

        .mor-about-content-title {
            font-size: 2.5rem;
            margin: 0 0 32px 0;
            color: var(--theme-text, var(--fg-base, #ffffff));
            letter-spacing: -1px;
            border-bottom: 2px solid var(--theme-border, var(--border-color, #333));
            padding-bottom: 16px;
        }

        .mor-about-bio {
            line-height: 1.8;
            font-size: 1.15rem;
            color: var(--theme-text-muted, var(--fg-dim, #aaaaaa));
        }

        .mor-about-bio p {
            margin-top: 0;
            margin-bottom: 24px;
        }
        
        .mor-about-bio p:first-of-type {
            font-size: 1.3rem;
            color: var(--theme-text, var(--fg-base, #ffffff));
            line-height: 1.7;
        }

        @media (max-width: 900px) {
            .mor-about-container {
                grid-template-columns: 1fr;
                gap: 40px;
            }
            .mor-about-sidebar {
                position: static;
                padding: 32px 20px;
            }
        }
        </style>
        "##,
    );

    let avatar_html = if !config.profile_image_url.trim().is_empty() {
        format!(
            r##"<img src="{}" alt="Profile avatar" class="mor-about-avatar" />"##,
            escape_html(&config.profile_image_url)
        )
    } else {
        // Fallback placeholder if no image provided
        r##"<div class="mor-about-avatar" style="display: flex; align-items: center; justify-content: center; font-size: 4rem; color: var(--theme-border-light, #444); font-family: monospace;">?</div>"##.to_string()
    };

    let mut links_html = String::new();
    if !config.contact_email.trim().is_empty() {
        links_html.push_str(&format!(
            r##"<a href="mailto:{}" class="mor-about-link email-link">Contact Me</a>"##,
            escape_html(&config.contact_email)
        ));
    }

    for link in &config.social_links {
        links_html.push_str(&format!(
            r##"<a href="{}" class="mor-about-link" target="_blank" rel="noopener noreferrer">{}</a>"##,
            escape_html(&link.url),
            escape_html(&link.label)
        ));
    }

    // Convert raw double-newlines into proper HTML paragraphs
    let bio_paragraphs: Vec<String> = escape_html(&config.bio_text)
        .split("\n\n")
        .filter(|p| !p.trim().is_empty())
        .map(|p| format!("<p>{}</p>", p.replace('\n', "<br/>")))
        .collect();

    let bio_html = bio_paragraphs.join("\n");

    html.push_str(&format!(
        r##"<div class="mor-about-container">
            <aside class="mor-about-sidebar">
                {avatar}
                <div class="mor-about-title-block">
                    <h1>{title}</h1>
                    <div class="mor-about-kicker">{kicker}</div>
                </div>
                <div class="mor-about-links">
                    {links}
                </div>
            </aside>

            <main class="mor-about-content">
                <h2 class="mor-about-content-title">About the Instructor</h2>
                <div class="mor-about-bio">
                    {bio}
                </div>
            </main>
        </div>
        "##,
        avatar = avatar_html,
        kicker = escape_html(&config.kicker),
        title = escape_html(&config.title),
        bio = bio_html,
        links = links_html
    ));

    format!("{}{}", crate::render::pages::page_chrome_overrides(&config.layout), html)
}
