//! Ads configuration panel.
//!
//! The editor takes a neutral position on advertising. This panel exposes
//! four modes — None (default), Always, OptIn, OptOut — plus the per-slot
//! and banner-position config that the consent modes need.
//!
//! Validation is friction-free: nothing here refuses to save or export.
//! When required fields are missing for the chosen mode, the panel shows
//! a warning banner so the blogger knows their export will be incomplete.
//! Whether to fix it is their choice.

use dioxus::prelude::*;

use crate::config::{AdBannerPosition, AdSlot, AdSlotPlacement, AdsConfig, AdsMode};

#[component]
pub fn AdsPanel(mut ads: Signal<AdsConfig>) -> Element {
    let current = ads();
    let warnings = current.warnings();
    let needs_publisher_data = current.requires_publisher_data();

    rsx! {
        div {
            class: "editor-panel ads-panel",

            h3 { "Advertising" }

            p {
                class: "ads-panel-blurb",
                "This editor takes no position on whether your blog should run ads. You decide."
            }

            // ---- Mode picker ---------------------------------------------
            fieldset {
                class: "ads-mode-fieldset",
                legend { "Ads mode" }

                AdsModeRadio {
                    current_mode: current.mode,
                    value: AdsMode::None,
                    label: "No ads",
                    description: "Your theme contains no advertising markup. (default)",
                    on_select: move |m| {
                        let mut next = ads();
                        next.mode = m;
                        ads.set(next);
                    },
                }
                AdsModeRadio {
                    current_mode: current.mode,
                    value: AdsMode::Always,
                    label: "Ads always on",
                    description: "Uses Blogger's native AdSense widget. Readers see ads on every page if you've linked an AdSense account in Blogger's Settings → Earnings.",
                    on_select: move |m| {
                        let mut next = ads();
                        next.mode = m;
                        ads.set(next);
                    },
                }
                AdsModeRadio {
                    current_mode: current.mode,
                    value: AdsMode::OptIn,
                    label: "Ads off until viewer consents",
                    description: "Readers see a small banner; ads only load if they choose to allow them. Requires your AdSense publisher ID and at least one slot ID below. Not GDPR-compliant on its own — EU readers also require a certified Consent Management Platform.",
                    on_select: move |m| {
                        let mut next = ads();
                        next.mode = m;
                        ads.set(next);
                    },
                }
                AdsModeRadio {
                    current_mode: current.mode,
                    value: AdsMode::OptOut,
                    label: "Ads on until viewer dismisses",
                    description: "Ads load on first visit; readers can dismiss them and the choice sticks. Requires publisher ID and slots. Not GDPR-compliant on its own.",
                    on_select: move |m| {
                        let mut next = ads();
                        next.mode = m;
                        ads.set(next);
                    },
                }
            }

            // ---- Warnings ------------------------------------------------
            if !warnings.is_empty() {
                div {
                    class: "ads-warnings",
                    role: "alert",
                    strong { "Export warnings" }
                    ul {
                        for w in warnings.iter() {
                            li { "{w}" }
                        }
                    }
                    p {
                        class: "ads-warnings-note",
                        "Export will still work but the ads in your theme will not render correctly until these are fixed."
                    }
                }
            }

            // ---- Publisher ID + slots (only relevant for OptIn/OptOut) ---
            if needs_publisher_data {
                div {
                    class: "ads-publisher-block",

                    label {
                        class: "ads-field-label",
                        "AdSense publisher ID"
                        input {
                            r#type: "text",
                            placeholder: "ca-pub-1234567890123456",
                            value: "{current.publisher_id}",
                            oninput: move |e| {
                                let mut next = ads();
                                next.publisher_id = e.value();
                                ads.set(next);
                            },
                        }
                        small { "Find this in your AdSense account under Account → Settings → Account information." }
                    }

                    SlotEditor { ads }
                }

                BannerPositionPicker { ads }

                details {
                    class: "ads-tos-disclosure",
                    summary { "About these modes and AdSense policy" }
                    p {
                        "Opt-in and opt-out modes inject AdSense scripts gated by viewer-side JavaScript. \
                         This pattern is common but is not independently verified against current AdSense Program Policies. \
                         You are responsible for compliance with your own AdSense account terms."
                    }
                    p {
                        "For EU/UK readers you additionally need a certified Consent Management Platform — the banner shipped here is opt-in UX, not a CMP."
                    }
                }
            }
        }
    }
}

#[component]
fn AdsModeRadio(
    current_mode: AdsMode,
    value: AdsMode,
    label: &'static str,
    description: &'static str,
    on_select: EventHandler<AdsMode>,
) -> Element {
    let checked = current_mode == value;
    rsx! {
        label {
            class: if checked { "ads-mode-radio is-selected" } else { "ads-mode-radio" },
            input {
                r#type: "radio",
                name: "ads-mode",
                checked: checked,
                onchange: move |_| on_select.call(value),
            }
            div {
                class: "ads-mode-radio-text",
                strong { "{label}" }
                p { "{description}" }
            }
        }
    }
}

#[component]
fn SlotEditor(mut ads: Signal<AdsConfig>) -> Element {
    let current = ads();
    rsx! {
        div {
            class: "ads-slots-block",

            div {
                class: "ads-slots-header",
                strong { "Ad slots" }
                button {
                    r#type: "button",
                    onclick: move |_| {
                        let mut next = ads();
                        next.slots.push(AdSlot::default());
                        ads.set(next);
                    },
                    "+ Add slot"
                }
            }

            if current.slots.is_empty() {
                p {
                    class: "ads-slots-empty",
                    "No slots yet. Add at least one to make ads render."
                }
            }

            for (idx, slot) in current.slots.iter().enumerate() {
                SlotRow {
                    key: "{idx}",
                    index: idx,
                    slot: slot.clone(),
                    on_change: move |(i, new_slot): (usize, AdSlot)| {
                        let mut next = ads();
                        if let Some(s) = next.slots.get_mut(i) {
                            *s = new_slot;
                        }
                        ads.set(next);
                    },
                    on_remove: move |i: usize| {
                        let mut next = ads();
                        if i < next.slots.len() {
                            next.slots.remove(i);
                        }
                        ads.set(next);
                    },
                }
            }
        }
    }
}

#[component]
fn SlotRow(
    index: usize,
    slot: AdSlot,
    on_change: EventHandler<(usize, AdSlot)>,
    on_remove: EventHandler<usize>,
) -> Element {
    let placement = slot.placement;
    let slot_id_for_input = slot.slot_id.clone();
    let slot_for_select = slot.clone();
    rsx! {
        div {
            class: "ads-slot-row",

            label {
                class: "ads-slot-id-label",
                "Slot ID"
                input {
                    r#type: "text",
                    placeholder: "1234567890",
                    value: "{slot_id_for_input}",
                    oninput: move |e| {
                        let mut next = slot.clone();
                        next.slot_id = e.value();
                        on_change.call((index, next));
                    },
                }
            }

            label {
                class: "ads-slot-placement-label",
                "Placement"
                select {
                    value: placement_value(placement),
                    onchange: move |e| {
                        let mut next = slot_for_select.clone();
                        next.placement = match e.value().as_str() {
                            "between" => AdSlotPlacement::BetweenPosts,
                            "both" => AdSlotPlacement::Both,
                            _ => AdSlotPlacement::Sidebar,
                        };
                        on_change.call((index, next));
                    },
                    option { value: "sidebar", "Sidebar" }
                    option { value: "between", "Between posts" }
                    option { value: "both", "Both" }
                }
            }

            button {
                r#type: "button",
                class: "ads-slot-remove",
                onclick: move |_| on_remove.call(index),
                "Remove"
            }
        }
    }
}

#[component]
fn BannerPositionPicker(mut ads: Signal<AdsConfig>) -> Element {
    let current = ads();
    rsx! {
        fieldset {
            class: "ads-banner-position-fieldset",
            legend { "Banner position" }

            BannerPositionRadio {
                current: current.banner_position,
                value: AdBannerPosition::BottomSlideUp,
                label: "Bottom slide-up",
                on_select: move |p| {
                    let mut next = ads();
                    next.banner_position = p;
                    ads.set(next);
                },
            }
            BannerPositionRadio {
                current: current.banner_position,
                value: AdBannerPosition::TopPushDown,
                label: "Top push-down",
                on_select: move |p| {
                    let mut next = ads();
                    next.banner_position = p;
                    ads.set(next);
                },
            }
            BannerPositionRadio {
                current: current.banner_position,
                value: AdBannerPosition::ModalOverlay,
                label: "Modal overlay",
                on_select: move |p| {
                    let mut next = ads();
                    next.banner_position = p;
                    ads.set(next);
                },
            }
        }
    }
}

#[component]
fn BannerPositionRadio(
    current: AdBannerPosition,
    value: AdBannerPosition,
    label: &'static str,
    on_select: EventHandler<AdBannerPosition>,
) -> Element {
    let checked = current == value;
    rsx! {
        label {
            class: if checked { "ads-banner-radio is-selected" } else { "ads-banner-radio" },
            input {
                r#type: "radio",
                name: "ads-banner-position",
                checked: checked,
                onchange: move |_| on_select.call(value),
            }
            span { "{label}" }
        }
    }
}

fn placement_value(p: AdSlotPlacement) -> &'static str {
    match p {
        AdSlotPlacement::Sidebar => "sidebar",
        AdSlotPlacement::BetweenPosts => "between",
        AdSlotPlacement::Both => "both",
    }
}
