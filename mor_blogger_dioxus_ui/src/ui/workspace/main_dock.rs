use dioxus::prelude::*;

#[component]
pub fn MainDock(
    tabs: Element,
    toolbar: Element,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "editor-center-workspace",
            style: "flex: 1 1 auto; min-width: 0; min-height: 0; display: flex; flex-direction: column; padding: 24px; overflow: hidden; position: relative;",

            div {
                class: "export-panel-header",
                div {
                    class: "preview-toolbar-group", 
                    style: "margin-bottom: 8px;",
                    {tabs}
                }
                div {
                    class: "export-toolbar export-toolbar-primary",
                    {toolbar}
                }
            }

            {children}
        }
    }
}