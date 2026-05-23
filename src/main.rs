#![allow(non_snake_case)]

mod app;
mod clipboard;
mod config;
mod defaults;
mod diagnostics;
mod presets;
mod render;
mod ui;
mod rehydration;

use app::App;

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");

    dioxus::launch(App);
}
