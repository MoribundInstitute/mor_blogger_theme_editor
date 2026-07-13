pub use mor_blogger_core::config::BlogPost;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiteData {
    pub posts: Vec<BlogPost>,
}

impl Default for SiteData {
    fn default() -> Self {
        Self {
            posts: vec![
                BlogPost {
                    title: "Welcome to Your Live Preview".to_string(),
                    date: "24 Oct, 2026".to_string(),
                    tags: vec!["Preview".to_string(), "Getting Started".to_string()],
                    snippet: "This preview shows exactly how your blog will look once your theme is published to Blogger: the same layout, fonts, and colors your readers will see. No surprises after export.".to_string(),
                    featured_image: None,
                    body: "<p>This preview shows exactly how your blog will look once your theme is published to Blogger: the same layout, fonts, and colors your readers will see. No surprises after export.</p>\n<p><strong>Try it now:</strong> pick a different color or font in the side panels and watch this page update instantly. Nothing reloads and you never lose your place on the page.</p>\n<blockquote>\"What you see here is what your readers get. This preview is built from the very same theme file you upload to Blogger.\"</blockquote>\n<p>Handy shortcut: turn on <strong>X-Ray</strong> (the skeleton button in the toolbar), then click any text, background, or <code data-edit-target=\"typography.mono_font_stack\">code snippet</code> to jump straight to the setting that controls it. Hold <strong>Shift</strong> and click any icon to swap it.</p>".to_string(),
                    url: "#".to_string(),
                    author_name: "Moribund Engine".to_string(),
                }
            ],
        }
    }
}

impl SiteData {
    pub fn ensure_minimum_posts(&mut self, minimum: usize) -> bool {
        let current_count = self.posts.len();
        if current_count >= minimum {
            return false; // No injection needed
        }

        for i in current_count..minimum {
            self.posts.push(BlogPost {
                title: format!("Auto-Generated Dummy Post #{}", i + 1),
                date: "25 Oct, 2026".to_string(),
                tags: vec!["Preview".to_string(), "Grid".to_string()],
                snippet: "This post was automatically injected by the Moribund engine to demonstrate grid layout features.".to_string(),
                featured_image: None,
                body: "This post was automatically injected by the Moribund engine to demonstrate grid layout features.".to_string(),
                url: "#".to_string(),
                author_name: "Moribund Engine".to_string(),
            });
        }
        true // Returns true so the UI knows it injected data
    }
}
