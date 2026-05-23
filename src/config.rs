#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ThemeConfig {
    pub site: SiteConfig,
    pub colors: ColorConfig,
    pub assets: AssetConfig,
    pub background: BackgroundConfig,
    pub buttons: ButtonConfig,
    pub typography: TypographyConfig,
    pub seo: SeoConfig,
    pub menu_links: Vec<MenuLink>,
    pub footer: FooterConfig,
    pub plugins: PluginConfig,
    pub static_pages: StaticPagesConfig,
    /// CSS rules contributed by the active preset (if any). Appended to the
    /// exported theme's `<b:skin>` block AFTER the token-driven `:root`, so
    /// preset rules can override or extend the base style.
    ///
    /// Empty for hand-built themes and for re-imported exports — preset
    /// CSS does not round-trip through rehydration.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub preset_css: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            site: SiteConfig::default(),
            colors: ColorConfig::default(),
            assets: AssetConfig::default(),
            background: BackgroundConfig::default(),
            buttons: ButtonConfig::default(),
            typography: TypographyConfig::default(),
            seo: SeoConfig::default(),
            menu_links: vec![
                MenuLink {
                    label: "Home".to_string(),
                    url: "/".to_string(),
                },
                MenuLink {
                    label: "Archive".to_string(),
                    url: "/archive.html".to_string(),
                },
            ],
            footer: FooterConfig::default(),
            plugins: PluginConfig::default(),
            static_pages: StaticPagesConfig::default(),
            preset_css: String::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SiteConfig {
    pub site_title: String,
    pub site_subtitle: String,
    pub header_logo_url: String,
    pub home_url: String,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            site_title: "My Blogger Theme".to_string(),
            site_subtitle: "A custom Blogger theme".to_string(),
            header_logo_url: String::new(),
            home_url: "/".to_string(),
        }
    }
}

/// Color palette. Most fields are single CSS colors (used in `color`,
/// `border-color`, etc., which cannot accept gradients). `bg_panel` and
/// `bg_elevated` are richer: they drive `background:` shorthand and can be
/// either a solid color or a linear gradient.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ColorConfig {
    pub bg_base: String,
    pub bg_panel: SurfaceFill,
    pub bg_elevated: SurfaceFill,
    pub fg_base: String,
    pub fg_muted: String,
    pub accent: String,
    pub border: String,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            bg_base: "#222129".to_string(),
            bg_panel: SurfaceFill::solid("#2b2933"),
            bg_elevated: SurfaceFill::solid("#343140"),
            fg_base: "#f2eadf".to_string(),
            fg_muted: "#bc8d6b".to_string(),
            accent: "#a9aae2".to_string(),
            border: "#6f6078".to_string(),
        }
    }
}

/// A surface fill — either a flat color or a two-stop linear gradient.
///
/// The struct holds *all* fields for both modes (color + from + to + angle),
/// so toggling between modes doesn't lose the user's previous values. The
/// `mode` field selects which fields are actually used.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SurfaceFill {
    pub mode: SurfaceMode,
    pub color: String,
    pub gradient_from: String,
    pub gradient_to: String,
    pub gradient_angle_deg: u16,
}

impl Default for SurfaceFill {
    fn default() -> Self {
        Self::solid("#2b2933")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub enum SurfaceMode {
    #[default]
    Solid,
    Gradient,
}

impl SurfaceFill {
    /// Convenience constructor for a solid fill where all gradient fields
    /// mirror the solid color (so toggling to gradient mode shows that color
    /// as both stops instead of empty strings).
    pub fn solid(color: impl Into<String>) -> Self {
        let c = color.into();
        Self {
            mode: SurfaceMode::Solid,
            gradient_from: c.clone(),
            gradient_to: c.clone(),
            gradient_angle_deg: 180,
            color: c,
        }
    }

    /// Render this fill as a CSS `background:` value.
    pub fn to_css(&self) -> String {
        match self.mode {
            SurfaceMode::Solid => self.color.clone(),
            SurfaceMode::Gradient => format!(
                "linear-gradient({}deg, {}, {})",
                self.gradient_angle_deg, self.gradient_from, self.gradient_to
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AssetConfig {
    pub favicon_url: String,
    pub social_card_image_url: String,
}

impl Default for AssetConfig {
    fn default() -> Self {
        Self {
            favicon_url: String::new(),
            social_card_image_url: String::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum BackgroundMode {
    Solid { color: String },
    Gradient { from: String, to: String, angle_deg: u16 },
    Tile { url: String },
}

impl Default for BackgroundMode {
    fn default() -> Self {
        Self::Solid {
            color: "#222129".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BackgroundConfig {
    pub mode: BackgroundMode,
}

impl Default for BackgroundConfig {
    fn default() -> Self {
        Self {
            mode: BackgroundMode::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ButtonConfig {
    pub radius: String,
    pub border_width: String,
    pub text_transform: String,
}

impl Default for ButtonConfig {
    fn default() -> Self {
        Self {
            radius: "0px".to_string(),
            border_width: "1px".to_string(),
            text_transform: "none".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TypographyConfig {
    pub body_font_stack: String,
    pub heading_font_stack: String,
    pub mono_font_stack: String,
    pub base_size: String,
    pub scale_ratio: String,
    pub line_height: String,
    pub heading_weight: String,
}

impl Default for TypographyConfig {
    fn default() -> Self {
        Self {
            body_font_stack: "serif".to_string(),
            heading_font_stack: "serif".to_string(),
            mono_font_stack: "monospace".to_string(),
            base_size: "16px".to_string(),
            scale_ratio: "1.2".to_string(),
            line_height: "1.6".to_string(),
            heading_weight: "700".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SeoConfig {
    pub meta_description: String,
    pub meta_keywords: String,
    pub custom_robots: String,
    pub license_url: String,
    pub author_name: String,
}

impl Default for SeoConfig {
    fn default() -> Self {
        Self {
            meta_description: String::new(),
            meta_keywords: String::new(),
            custom_robots: "index, follow".to_string(),
            license_url: String::new(),
            author_name: String::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MenuLink {
    pub label: String,
    pub url: String,
}

impl Default for MenuLink {
    fn default() -> Self {
        Self {
            label: "Link".to_string(),
            url: "#".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FooterConfig {
    pub footer_text: String,
    pub footer_license_label: String,
    pub footer_license_url: String,
}

impl Default for FooterConfig {
    fn default() -> Self {
        Self {
            footer_text: "Powered by Blogger".to_string(),
            footer_license_label: String::new(),
            footer_license_url: String::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PluginConfig {
    pub custom_js: String,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            custom_js: String::new(),
        }
    }
}

// ==========================================
// STATIC PAGES CONFIGURATION
// ==========================================

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StaticPagesConfig {
    pub sync_with_global_theme: bool,
    pub custom_colors: Option<ColorConfig>,
    pub custom_typography: Option<TypographyConfig>,
    pub archive: ArchivePageConfig,
    pub categories: CategoriesPageConfig,
}

impl Default for StaticPagesConfig {
    fn default() -> Self {
        Self {
            sync_with_global_theme: true,
            custom_colors: None,
            custom_typography: None,
            archive: ArchivePageConfig::default(),
            categories: CategoriesPageConfig::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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