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
                    title: "WYSIWYG: The Tier 3 Architecture".to_string(),
                    date: "24 Oct, 2026".to_string(),
                    tags: vec!["Preview".to_string(), "Architecture".to_string()],
                    snippet: "This is a 100% accurate representation of your exported Blogger XML. It maps the exact CSS class hooks, variables, and DOM structures used by the Blogger engine, completely eliminating visual guesswork.".to_string(),
                    featured_image: None,
                    body: "<p>This is a 100% accurate representation of your exported Blogger XML. It maps the exact CSS class hooks, variables, and DOM structures used by the Blogger engine, completely eliminating visual guesswork.</p>\n<p>Furthermore, Dioxus now runs a <strong>Two-Way DOM Morpher</strong> inside the iframe. Modifying colors, fonts, and text fields in the left/right docks will update the preview instantly without causing destructive iframe reloads or scroll-jumping.</p>\n<blockquote>\"WYSIWYG means What You See Is What You Get. No more making up shite.\"</blockquote>\n<p>Shift+Click on any text, background, or <code data-edit-target=\"typography.mono_font_stack\">code block</code> to instantly jump to the relevant editor panel via the JS interop bridge.</p>".to_string(),
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
