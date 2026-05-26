use mor_blogger_theme_editor::App;

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");

    // Dioxus automatically boots the native Desktop window because
    // it is defined as the default feature in your Cargo.toml
    dioxus::launch(App);
}
