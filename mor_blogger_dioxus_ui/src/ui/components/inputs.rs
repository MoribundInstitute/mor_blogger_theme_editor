use dioxus::prelude::*;

#[component]
pub fn EditorCard(title: String, children: Element) -> Element {
    rsx! {
        section {
            class: "editor-card",
            SectionTitle { title }
            {children}
        }
    }
}

#[component]
pub fn EditorInput(
    label: String,
    value: Signal<String>,
    input_type: String,
    placeholder: String,
) -> Element {
    let theme = try_consume_context::<crate::app::state::ThemeState>();
    let is_color = input_type == "color";

    rsx! {
        div {
            class: "editor-field-group",

            label {
                class: "editor-field-label",
                "{label}"
            }

            if is_color {
                ColorInput {
                    value: value(),
                    oninput: move |v: String| value.set(v),
                }
            } else {
                input {
                    r#type: "{input_type}",
                    value: "{value}",
                    placeholder: "{placeholder}",
                    class: "editor-field",
                    oninput: move |e| value.set(e.value()),
                    // change = gesture committed (blur / Enter): one undo step.
                    onchange: move |_| {
                        if let Some(theme) = theme {
                            theme.commit();
                        }
                    },
                }
            }
        }
    }
}

/// Drop-in `input[type=color]` wired to the shared recent-colors strip and the
/// undo history (a committed pick = one undo step). Use `swatches: false` in
/// tight inline layouts where the strip has no room — recording still happens.
#[component]
pub fn ColorInput(
    value: String,
    oninput: EventHandler<String>,
    #[props(default = "editor-field editor-color-field".to_string())] class: String,
    #[props(default)] style: String,
    #[props(default = true)] swatches: bool,
) -> Element {
    let theme = try_consume_context::<crate::app::state::ThemeState>();
    let record = move |c: &str| {
        if let Some(theme) = theme {
            theme.push_recent_color(c);
            theme.commit();
        }
    };
    rsx! {
        input {
            r#type: "color",
            class: "{class}",
            style: "{style}",
            value: "{value}",
            // Color inputs sometimes sit inside clickable rows (effect toggles);
            // opening the picker must not trigger the row.
            onclick: move |e| e.stop_propagation(),
            oninput: move |e| oninput.call(e.value()),
            onchange: move |e| record(&e.value()),
        }
        if swatches {
            RecentColorSwatches {
                onpick: move |c: String| {
                    oninput.call(c.clone());
                    record(&c);
                }
            }
        }
    }
}

/// Penpot-style recent-color strip (newest first, capped in `ThemeState`).
/// Renders nothing until a color has been committed anywhere in the app, and
/// sits under each color field so a pick always has an unambiguous target.
#[component]
pub fn RecentColorSwatches(onpick: EventHandler<String>) -> Element {
    let Some(theme) = try_consume_context::<crate::app::state::ThemeState>() else {
        return rsx! {};
    };
    let recents = theme.recent_colors.read().clone();
    if recents.is_empty() {
        return rsx! {};
    }

    rsx! {
        div {
            class: "recent-swatches",
            for color in recents {
                button {
                    class: "recent-swatch",
                    title: "{color}",
                    style: "background: {color};",
                    onclick: {
                        let color = color.clone();
                        move |_| onpick.call(color.clone())
                    },
                }
            }
        }
    }
}

#[component]
pub fn EditorSelect(
    label: String,
    value: String,
    options: Vec<(String, String)>,
    onchange: EventHandler<Event<FormData>>,
) -> Element {
    let theme = try_consume_context::<crate::app::state::ThemeState>();
    rsx! {
        div {
            class: "editor-field-group",

            label {
                class: "editor-field-label",
                "{label}"
            }

            select {
                class: "editor-select",
                value: "{value}",
                onchange: move |e| {
                    onchange.call(e);
                    if let Some(theme) = theme {
                        theme.commit();
                    }
                },

                for (opt_val, opt_label) in options {
                    option {
                        value: "{opt_val}",
                        selected: opt_val == value,
                        "{opt_label}"
                    }
                }
            }
        }
    }
}

/// Callback-flavored text field for state that lives inside a struct signal
/// (where [`EditorInput`]'s `Signal<String>` binding can't reach).
#[component]
pub fn EditorTextField(
    label: String,
    value: String,
    placeholder: String,
    oninput: EventHandler<String>,
) -> Element {
    let theme = try_consume_context::<crate::app::state::ThemeState>();
    rsx! {
        div {
            class: "editor-field-group",

            label {
                class: "editor-field-label",
                "{label}"
            }

            input {
                r#type: "text",
                value: "{value}",
                placeholder: "{placeholder}",
                class: "editor-field",
                oninput: move |e| oninput.call(e.value()),
                onchange: move |_| {
                    if let Some(theme) = theme {
                        theme.commit();
                    }
                },
            }
        }
    }
}

#[component]
pub fn EditorCheckbox(label: String, checked: bool, onchange: EventHandler<bool>) -> Element {
    let theme = try_consume_context::<crate::app::state::ThemeState>();
    rsx! {
        label {
            style: "display: flex; align-items: center; gap: 8px; font-size: 0.8rem; color: var(--fg-base); cursor: pointer;",
            input {
                r#type: "checkbox",
                checked: checked,
                onchange: move |e| {
                    onchange.call(e.checked());
                    if let Some(theme) = theme {
                        theme.commit();
                    }
                },
            }
            "{label}"
        }
    }
}

#[component]
pub fn SectionTitle(title: String) -> Element {
    rsx! {
        h3 {
            class: "editor-card-title",
            "{title}"
        }
    }
}

#[component]
pub fn PanelNote(title: String, body: String) -> Element {
    rsx! {
        div {
            class: "editor-note",

            h4 {
                class: "editor-note-title",
                "{title}"
            }

            p {
                class: "editor-note-body",
                "{body}"
            }
        }
    }
}
