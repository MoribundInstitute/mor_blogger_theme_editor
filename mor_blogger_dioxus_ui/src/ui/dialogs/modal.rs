// src/modal.rs
// MOR modal component. Stripped default prop string allocation bloat.

use dioxus::prelude::*;

/// Makes every `.mor-modal` draggable by its header. Installed once globally;
/// the resize grip is handled natively by `resize: both` on the modal itself.
const MODAL_DRAG_JS: &str = r#"
(function () {
    if (window.__morModalDragInstalled) return;
    window.__morModalDragInstalled = true;

    document.addEventListener('pointerdown', function (e) {
        const header = e.target.closest('.mor-modal-header');
        if (!header) return;
        if (e.target.closest('.mor-modal-close')) return;

        const modal = header.closest('.mor-modal');
        if (!modal) return;

        e.preventDefault();

        // Anchor the modal at its current spot so flex centering stops fighting the drag.
        const rect = modal.getBoundingClientRect();
        modal.style.position = 'fixed';
        modal.style.margin = '0';
        modal.style.left = rect.left + 'px';
        modal.style.top = rect.top + 'px';

        const startX = e.clientX;
        const startY = e.clientY;
        const startLeft = rect.left;
        const startTop = rect.top;
        header.style.cursor = 'grabbing';

        const onMove = function (moveEvt) {
            const nextLeft = Math.max(0, Math.min(startLeft + (moveEvt.clientX - startX), window.innerWidth - 60));
            const nextTop = Math.max(0, Math.min(startTop + (moveEvt.clientY - startY), window.innerHeight - 30));
            modal.style.left = nextLeft + 'px';
            modal.style.top = nextTop + 'px';
        };

        const onUp = function () {
            document.removeEventListener('pointermove', onMove);
            document.removeEventListener('pointerup', onUp);
            header.style.cursor = '';
        };

        document.addEventListener('pointermove', onMove);
        document.addEventListener('pointerup', onUp);
    });
})();
"#;

#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    pub open: Signal<bool>,
    pub title: String,
    pub children: Element,
    #[props(default = None)]
    pub on_close: Option<EventHandler<()>>,
    #[props(default = String::new())]
    pub style: String,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    let mut open = props.open;
    let on_close = props.on_close;

    if !open() {
        return rsx! { Fragment {} };
    }

    rsx! {
        div {
            class: "mor-modal-backdrop",
            onclick: move |_| {
                if let Some(h) = on_close { h.call(()); }
                open.set(false);
            },

            div {
                class: "mor-modal",
                // Base width injected first, then the per-dialog style, then the
                // movable/resizable overrides last so they always win (the
                // max-* caps let `resize: both` grow past a dialog's own limit).
                style: "min-width:380px; {props.style} resize: both; overflow: hidden; max-width: 95vw; max-height: 92vh;",
                onclick: |e| e.stop_propagation(),

                script { dangerous_inner_html: MODAL_DRAG_JS }

                div { class: "mor-modal-header",
                    span { "{props.title}" }
                    div {
                        class: "mor-modal-close",
                        onclick: move |e| {
                            e.stop_propagation();
                            if let Some(h) = on_close { h.call(()); }
                            open.set(false);
                        },
                        "×"
                    }
                }

                div { class: "mor-modal-body",
                    {props.children}
                }
            }
        }
    }
}
