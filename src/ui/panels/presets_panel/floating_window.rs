use dioxus::prelude::*;

use crate::presets::all_presets;

use super::card::PresetCard;
use super::drag::PRESET_FLOATING_DRAG_JS;
use super::signals::ThemeSignals;

#[derive(Props, Clone, PartialEq)]
pub struct PresetFloatingWindowProps {
    pub signals: ThemeSignals,
    pub active_preset: Signal<Option<&'static str>>,
    pub show_undocked_presets: Signal<bool>,
}

#[component]
pub fn PresetFloatingWindow(props: PresetFloatingWindowProps) -> Element {
    let presets = all_presets();
    let active = props.active_preset;
    let mut show_undocked_presets = props.show_undocked_presets;

    rsx! {
        script { dangerous_inner_html: "{PRESET_FLOATING_DRAG_JS}" }

        div {
            class: "preset-floating-window",

            div {
                class: "preset-floating-window-bar floating-editor-window-bar",

                div {
                    class: "preset-floating-window-title-group",
                    span { class: "floating-editor-grip", "⠿" }
                    div {
                        div {
                            class: "preset-floating-window-title",
                            "Theme Presets"
                        }
                        div {
                            class: "preset-floating-window-subtitle",
                            "Detached preset browser"
                        }
                    }
                }

                div {
                    class: "floating-editor-window-actions",

                    button {
                        class: "editor-mini-button",
                        onclick: move |_| {
                            show_undocked_presets.set(false);
                        },
                        "Dock"
                    }
                }
            }

            div {
                class: "preset-floating-window-body",

                div {
                    class: "preset-rail preset-rail-undocked",

                    for preset in presets.iter() {
                        PresetCard {
                            key: "undocked-{preset.id}",
                            preset: preset.clone(),
                            is_active: active.read().map(|id| id == preset.id).unwrap_or(false),
                            signals: props.signals,
                            active_preset: active,
                        }
                    }
                }
            }
        }
    }
}
