//! UI surface for the integrity engine.
//!
//! Renders the current [`DiagnosticResult`] as either a green "System OK"
//! status line or a styled error/warning list that matches the terminal
//! aesthetic of the Blogger Theme Architect.

use dioxus::prelude::*;

use crate::diagnostics::{DiagnosticResult, Severity};

#[derive(Props, Clone, PartialEq)]
pub struct DiagnosticsPanelProps {
    pub result: Signal<DiagnosticResult>,
}

#[component]
pub fn DiagnosticsPanel(props: DiagnosticsPanelProps) -> Element {
    let result = props.result.read();

    let error_count = result.errors.len();
    let warning_count = result
        .warnings
        .iter()
        .filter(|w| w.severity == Severity::Warning)
        .count();

    rsx! {
        section {
            "aria-live": "polite",
            class: "diagnostics-panel",

            if result.is_valid && warning_count == 0 {
                div {
                    class: "diagnostics-ok",
                    span {
                        class: "diagnostics-ok-icon",
                        "✓"
                    }
                    span { "sys.integrity — all checks passed" }
                }
            } else {
                div {
                    class: "diagnostics-problems",

                    div {
                        class: "diagnostics-badges",

                        if error_count > 0 {
                            span {
                                class: "diagnostics-badge diagnostics-error",
                                "{error_count} error(s)"
                            }
                        }

                        if warning_count > 0 {
                            span {
                                class: "diagnostics-badge diagnostics-warning",
                                "{warning_count} warning(s)"
                            }
                        }
                    }

                    for (i, err) in result.errors.iter().enumerate() {
                        div {
                            key: "e-{i}",
                            class: "diagnostics-error",
                            span {
                                class: "diagnostics-prefix",
                                "ERR"
                            }
                            "{err}"
                        }
                    }

                    for (i, w) in result.warnings.iter()
                        .filter(|w| w.severity == Severity::Warning)
                        .enumerate()
                    {
                        div {
                            key: "w-{i}",
                            class: "diagnostics-warning",
                            span {
                                class: "diagnostics-prefix",
                                "WRN"
                            }
                            "[{w.code}] {w.message}"
                        }
                    }
                }
            }
        }
    }
}
