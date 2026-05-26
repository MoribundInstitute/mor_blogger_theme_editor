use serde::{Deserialize, Serialize};

use super::styling::{ColorConfig, TypographyConfig};
use crate::config::MenuLink;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StaticPagesConfig {
    pub sync_with_global_theme: bool,
    pub custom_colors: Option<ColorConfig>,
    pub custom_typography: Option<TypographyConfig>,

    pub archive: ArchivePageConfig,
    pub categories: CategoriesPageConfig,
    pub about: AboutPageConfig,
    pub portfolio: PortfolioPageConfig,
    pub lms: LmsConfig,
}

impl Default for StaticPagesConfig {
    fn default() -> Self {
        Self {
            sync_with_global_theme: true,
            custom_colors: None,
            custom_typography: None,

            archive: ArchivePageConfig::default(),
            categories: CategoriesPageConfig::default(),
            about: AboutPageConfig::default(),
            portfolio: PortfolioPageConfig::default(),
            lms: LmsConfig::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ArchivePageConfig {
    pub kicker: String,
    pub title: String,
    pub description: String,
    pub max_results: u32,
}

impl Default for ArchivePageConfig {
    fn default() -> Self {
        Self {
            kicker: "THE_MORIBUND_INSTITUTE // ARCHIVE".to_string(),
            title: "Chronological Archive".to_string(),
            description: "A date-sorted index of Institute posts, lessons, wiki walks, commentaries, and assorted textual machinery.".to_string(),
            max_results: 150,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CategoriesPageConfig {
    pub kicker: String,
    pub title: String,
    pub description: String,
    pub enabled_sections: Vec<String>,
}

impl Default for CategoriesPageConfig {
    fn default() -> Self {
        Self {
            kicker: "THE_MORIBUND_INSTITUTE // CATEGORIES".to_string(),
            title: "Browse Categories".to_string(),
            description: "A subject index for Institute posts, lessons, wiki walks, lexicographical rummaging, video commentary, and other classified whatnot.".to_string(),
            enabled_sections: vec![
                "000 General Works".to_string(),
                "100 Philosophy".to_string(),
                "200 Religion".to_string(),
                "300 Social Sciences".to_string(),
                "400 Language".to_string(),
                "500 Science".to_string(),
                "600 Technology".to_string(),
                "700 Arts".to_string(),
                "800 Literature".to_string(),
                "900 History".to_string(),
            ],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AboutPageConfig {
    pub kicker: String,
    pub title: String,
    pub profile_image_url: String,
    pub bio_text: String,
    pub contact_email: String,
    pub social_links: Vec<MenuLink>,
}

impl Default for AboutPageConfig {
    fn default() -> Self {
        Self {
            kicker: "THE_MORIBUND_INSTITUTE // ABOUT".to_string(),
            title: "About".to_string(),
            profile_image_url: String::new(),
            bio_text:
                "Write a short biography, project manifesto, or institutional origin myth here."
                    .to_string(),
            contact_email: String::new(),
            social_links: vec![
                MenuLink {
                    label: "Website".to_string(),
                    url: "#".to_string(),
                },
                MenuLink {
                    label: "GitHub".to_string(),
                    url: "#".to_string(),
                },
            ],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PortfolioPageConfig {
    pub kicker: String,
    pub title: String,
    pub description: String,
    pub columns: u8,
    pub gallery_images: Vec<String>,
}

impl Default for PortfolioPageConfig {
    fn default() -> Self {
        Self {
            kicker: "THE_MORIBUND_INSTITUTE // PORTFOLIO".to_string(),
            title: "Portfolio".to_string(),
            description:
                "A curated gallery of projects, artwork, experiments, and assorted visual whatnot."
                    .to_string(),
            columns: 3,
            gallery_images: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LmsConfig {
    pub kicker: String,
    pub title: String,
    pub description: String,
    pub course_title: String,
    pub course_description: String,
    pub lessons: Vec<LmsLessonConfig>,
}

impl Default for LmsConfig {
    fn default() -> Self {
        Self {
            kicker: "THE_MORIBUND_INSTITUTE // LESSONS".to_string(),
            title: "Learning Hub".to_string(),
            description: "A structured index for lessons, study paths, course notes, and educational machinery.".to_string(),
            course_title: "Starter Course".to_string(),
            course_description: "A short description of the course or learning path.".to_string(),
            lessons: vec![
                LmsLessonConfig {
                    title: "Lesson One".to_string(),
                    description: "Introduce the topic, goal, or skill covered in this lesson.".to_string(),
                    url: "#".to_string(),
                    duration_label: "10 min".to_string(),
                },
                LmsLessonConfig {
                    title: "Lesson Two".to_string(),
                    description: "Continue the learning path with a second structured lesson.".to_string(),
                    url: "#".to_string(),
                    duration_label: "15 min".to_string(),
                },
            ],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LmsLessonConfig {
    pub title: String,
    pub description: String,
    pub url: String,
    pub duration_label: String,
}

impl Default for LmsLessonConfig {
    fn default() -> Self {
        Self {
            title: "Untitled Lesson".to_string(),
            description: String::new(),
            url: "#".to_string(),
            duration_label: String::new(),
        }
    }
}
