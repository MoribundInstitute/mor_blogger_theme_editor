//! Ads rendering: produces the four substitution strings consumed by
//! `theme.rs` to implement the four ads modes.
//!
//! - **None**: all four functions return empty strings
//! - **Always**: the sidebar function returns the native AdSense widget
//!   (Blogger fills in `<data:adCode/>` server-side from the blogger's
//!   linked AdSense account); the other three are still empty
//! - **OptIn / OptOut**: the consent banner and runtime script functions
//!   produce client-side markup that gates ad loading on viewer consent
//!
//! See the AdsConfig docs in `config.rs` for the full policy explanation.

use crate::config::{AdBannerPosition, AdSlotPlacement, AdsConfig, AdsMode};

const WIDGET_ADSENSE_SIDEBAR: &str = include_str!("../template_parts/widgets/adsense_sidebar.xml");

// ==========================================================================
// Ads rendering helpers
// ==========================================================================

/// Sidebar widget body for the Always mode. Empty string for None/OptIn/
/// OptOut. For OptIn/OptOut the sidebar slot is rendered by the consent
/// banner script, not by Blogger's native widget machinery, so we don't
/// emit a `<b:widget type='AdSense'>` here.
pub(super) fn render_ads_widget_sidebar(ads: &AdsConfig) -> String {
    match ads.mode {
        AdsMode::Always => WIDGET_ADSENSE_SIDEBAR.to_string(),
        _ => String::new(),
    }
}

/// `<head>` script. None/Always: empty (Blogger auto-injects the AdSense
/// loader when a native AdSense widget exists). OptIn: empty (the script
/// is loaded by ADS_RUNTIME_SCRIPT after consent). OptOut: empty (same;
/// loaded by runtime script on first paint).
pub(super) fn render_ads_head_script(_ads: &AdsConfig) -> String {
    // Reserved for future use. Currently all script loading is centralized
    // in render_ads_runtime_script so the consent state machine has one
    // place to live.
    String::new()
}

/// Consent banner DOM. The blogger-chosen banner_position selects markup
/// class; the runtime script picks behavior off the same class. The copy
/// is fixed (no engagement-encouragement language) per editor policy.
pub(super) fn render_ads_consent_banner(ads: &AdsConfig) -> String {
    if !matches!(ads.mode, AdsMode::OptIn | AdsMode::OptOut) {
        return String::new();
    }

    let position_class = match ads.banner_position {
        AdBannerPosition::BottomSlideUp => "mor-ads-banner-bottom",
        AdBannerPosition::TopPushDown => "mor-ads-banner-top",
        AdBannerPosition::ModalOverlay => "mor-ads-banner-modal",
    };

    let (copy, buttons) = match ads.mode {
        AdsMode::OptIn => (
            "This site uses advertising.",
            r#"<button type="button" data-mor-ads-action="allow">Allow</button><button type="button" data-mor-ads-action="decline">Decline</button>"#,
        ),
        AdsMode::OptOut => (
            "This site shows ads.",
            r#"<button type="button" data-mor-ads-action="dismiss">Dismiss</button>"#,
        ),
        _ => unreachable!(),
    };

    // CDATA-wrap nothing here — it's plain HTML inside the Blogger template
    // body, which is XHTML so quotes and entities matter. The literal we
    // emit is well-formed XHTML.
    format!(
        r#"<div class="mor-ads-banner {position}" id="mor-ads-banner" role="dialog" aria-live="polite" hidden="hidden">
  <span class="mor-ads-banner-text">{copy}</span>
  <span class="mor-ads-banner-actions">{buttons}</span>
</div>"#,
        position = position_class,
        copy = copy,
        buttons = buttons,
    )
}

/// Runtime JS. The state machine:
///
/// - On first paint: read `localStorage` key `mor_ads_consent_v1`.
/// - OptIn mode:
///     * unset    -> show banner
///     * "allow"  -> load AdSense, reveal slots
///     * "decline"-> do nothing
/// - OptOut mode:
///     * unset    -> show banner AND load ads (default is on)
///     * "allow"  -> load ads (same as unset, no banner)
///     * "decline"-> do not load, hide containers (sticky)
///
/// Persistence is forever per-browser per-origin. Once a viewer makes a
/// choice in OptIn or OptOut mode, they don't get prompted again.
pub(super) fn render_ads_runtime_script(ads: &AdsConfig) -> String {
    if !matches!(ads.mode, AdsMode::OptIn | AdsMode::OptOut) {
        return String::new();
    }

    let mode_token = match ads.mode {
        AdsMode::OptIn => "optin",
        AdsMode::OptOut => "optout",
        _ => unreachable!(),
    };

    // Build the slot descriptors as a small JSON array. JS reads this on
    // page load and injects `<ins class="adsbygoogle">` tags into matching
    // containers.
    let mut slot_descriptors = String::from("[");
    for (i, slot) in ads.slots.iter().enumerate() {
        if i > 0 {
            slot_descriptors.push(',');
        }
        let placement = match slot.placement {
            AdSlotPlacement::Sidebar => "sidebar",
            AdSlotPlacement::BetweenPosts => "between",
            AdSlotPlacement::Both => "both",
        };
        // escape_attr handles quotes; slot_id is user-supplied so escape it
        slot_descriptors.push_str(&format!(
            r#"{{"slot":"{}","placement":"{}"}}"#,
            escape_js_string(&slot.slot_id),
            placement,
        ));
    }
    slot_descriptors.push(']');

    let publisher_id = escape_js_string(&ads.publisher_id);

    // The script is wrapped in CDATA because Blogger's parser is XHTML and
    // any literal `<`, `&`, `<=`, etc. inside a bare <script> tag will be
    // misparsed. CDATA escape pattern matches what render_custom_plugin_scripts
    // already does for user-supplied JS.
    format!(
        r#"<script type='text/javascript'>
//<![CDATA[
(function() {{
  var MODE = "{mode}";
  var PUB  = "{pub}";
  var SLOTS = {slots};
  var KEY = "mor_ads_consent_v1";

  function getConsent() {{
    try {{ return window.localStorage.getItem(KEY); }} catch (e) {{ return null; }}
  }}
  function setConsent(v) {{
    try {{ window.localStorage.setItem(KEY, v); }} catch (e) {{}}
  }}
  function injectAdsenseLoader() {{
    if (document.getElementById("mor-ads-loader") || !PUB) return;
    var s = document.createElement("script");
    s.id = "mor-ads-loader";
    s.async = true;
    s.src = "https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=" + encodeURIComponent(PUB);
    s.setAttribute("crossorigin", "anonymous");
    document.head.appendChild(s);
  }}
  function injectSlot(slot, placement) {{
    if (!slot || !PUB) return;
    var containers = document.querySelectorAll('[data-mor-ad-placement="' + placement + '"], [data-mor-ad-placement="both"]');
    for (var i = 0; i < containers.length; i++) {{
      var c = containers[i];
      if (c.querySelector("ins.adsbygoogle")) continue;
      var ins = document.createElement("ins");
      ins.className = "adsbygoogle";
      ins.style.display = "block";
      ins.setAttribute("data-ad-client", PUB);
      ins.setAttribute("data-ad-slot", slot);
      ins.setAttribute("data-ad-format", "auto");
      ins.setAttribute("data-full-width-responsive", "true");
      c.appendChild(ins);
      try {{ (window.adsbygoogle = window.adsbygoogle || []).push({{}}); }} catch (e) {{}}
    }}
  }}
  function loadAllAds() {{
    injectAdsenseLoader();
    for (var i = 0; i < SLOTS.length; i++) {{
      injectSlot(SLOTS[i].slot, SLOTS[i].placement);
    }}
  }}
  function hideAds() {{
    var nodes = document.querySelectorAll("ins.adsbygoogle, [data-mor-ad-placement]");
    for (var i = 0; i < nodes.length; i++) {{
      nodes[i].style.display = "none";
    }}
  }}
  function showBanner() {{
    var b = document.getElementById("mor-ads-banner");
    if (!b) return;
    b.removeAttribute("hidden");
  }}
  function hideBanner() {{
    var b = document.getElementById("mor-ads-banner");
    if (!b) return;
    b.setAttribute("hidden", "hidden");
  }}

  function handleAction(action) {{
    if (action === "allow") {{
      setConsent("allow");
      hideBanner();
      loadAllAds();
    }} else if (action === "decline" || action === "dismiss") {{
      setConsent("decline");
      hideBanner();
      hideAds();
    }}
  }}

  function init() {{
    var consent = getConsent();

    if (MODE === "optin") {{
      if (consent === "allow") {{
        loadAllAds();
      }} else if (consent === "decline") {{
        // Stay quiet.
      }} else {{
        showBanner();
      }}
    }} else if (MODE === "optout") {{
      if (consent === "decline") {{
        // Sticky off.
      }} else {{
        loadAllAds();
        if (consent !== "allow") {{
          showBanner();
        }}
      }}
    }}

    document.addEventListener("click", function(e) {{
      var t = e.target;
      if (!t || !t.getAttribute) return;
      var action = t.getAttribute("data-mor-ads-action");
      if (action) handleAction(action);
    }});
  }}

  if (document.readyState === "loading") {{
    document.addEventListener("DOMContentLoaded", init);
  }} else {{
    init();
  }}
}})();
//]]>
</script>"#,
        mode = mode_token,
        pub = publisher_id,
        slots = slot_descriptors,
    )
}

/// Minimal JS-string escape. Newlines, quotes, backslashes. The values
/// passing through here are publisher IDs and slot IDs, which AdSense
/// constrains to alphanumeric plus hyphens — so this is defense-in-depth
/// rather than a hot path.
fn escape_js_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            '\\' => out.push_str(r"\\"),
            '"' => out.push_str(r#"\""#),
            '\n' => out.push_str(r"\n"),
            '\r' => out.push_str(r"\r"),
            _ => out.push(c),
        }
    }
    out
}
