use dioxus::prelude::*;

use crate::ui::panels::presets_panel::ThemeSignals;

pub fn apply_hotswap_json(signals: ThemeSignals, json_text: String) {
    let Ok(val) = serde_json::from_str::<serde_json::Value>(&json_text) else {
        log::error!("Load Data failed: invalid JSON payload.");
        return;
    };

    let apply_str = |val: &serde_json::Value, path: &[&str], sig: &mut Signal<String>| {
        let mut current = val;

        for p in path {
            if let Some(next) = current.get(p) {
                current = next;
            } else {
                return;
            }
        }

        if let Some(s_val) = current.as_str() {
            sig.set(s_val.to_string());
        }
    };

    let mut s = signals;

    apply_str(&val, &["site", "site_title"], &mut s.site_title);
    apply_str(&val, &["site", "site_subtitle"], &mut s.site_subtitle);
    apply_str(&val, &["site", "header_logo_url"], &mut s.header_logo_url);
    apply_str(&val, &["site", "home_url"], &mut s.home_url);

    apply_str(&val, &["assets", "favicon_url"], &mut s.favicon_url);
    apply_str(
        &val,
        &["assets", "social_card_image_url"],
        &mut s.social_card_image_url,
    );

    if let Some(menus) = val.get("menu_links").and_then(|m| m.as_array()) {
        if let Some(m) = menus.get(0) {
            apply_str(m, &["label"], &mut s.menu_1_label);
            apply_str(m, &["url"], &mut s.menu_1_url);
        }

        if let Some(m) = menus.get(1) {
            apply_str(m, &["label"], &mut s.menu_2_label);
            apply_str(m, &["url"], &mut s.menu_2_url);
        }

        if let Some(m) = menus.get(2) {
            apply_str(m, &["label"], &mut s.menu_3_label);
            apply_str(m, &["url"], &mut s.menu_3_url);
        }

        if let Some(m) = menus.get(3) {
            apply_str(m, &["label"], &mut s.menu_4_label);
            apply_str(m, &["url"], &mut s.menu_4_url);
        }
    }

    apply_str(&val, &["seo", "meta_description"], &mut s.meta_description);
    apply_str(&val, &["seo", "meta_keywords"], &mut s.meta_keywords);
    apply_str(&val, &["seo", "custom_robots"], &mut s.custom_robots);
    apply_str(&val, &["seo", "author_name"], &mut s.author_name);
    apply_str(&val, &["seo", "license_url"], &mut s.license_url);

    apply_str(&val, &["footer", "footer_text"], &mut s.footer_text);
    apply_str(
        &val,
        &["footer", "footer_license_label"],
        &mut s.footer_license_label,
    );
    apply_str(
        &val,
        &["footer", "footer_license_url"],
        &mut s.footer_license_url,
    );

    apply_str(&val, &["plugins", "custom_js"], &mut s.custom_js);

    if let Some(ads_val) = val.get("ads") {
        match serde_json::from_value(ads_val.clone()) {
            Ok(ads_config) => s.ads.set(ads_config),
            Err(err) => log::error!("Load Data failed: invalid ads config: {}", err),
        }
    }
}
