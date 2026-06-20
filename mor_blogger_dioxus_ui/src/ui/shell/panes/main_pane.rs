use dioxus::prelude::*;

#[component]
pub fn MainPane(
    #[props(default = false)] hide_header: bool,
    tabs: Element,
    #[props(default)] toolbar: Option<Element>,
    children: Element,
) -> Element {
    // FIX: Do the math outside the rsx macro
    let padding = if hide_header { "8px" } else { "24px" };

    rsx! {
        div {
            class: "editor-center-workspace",
            style: "flex: 1 1 auto; min-width: 0; min-height: 0; display: flex; flex-direction: column; padding: {padding}; overflow: hidden; position: relative;",

            if !hide_header {
                div {
                    class: "export-panel-header",
                    div {
                        class: "preview-toolbar-group",
                        style: "margin-bottom: 8px; align-items: center;",
                        {tabs}
                    }
                    if let Some(tb) = toolbar {
                        div {
                            class: "export-toolbar export-toolbar-primary",
                            {tb}
                        }
                    }
                }
            }

            {children}
        }
    }
}
