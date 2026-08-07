#![allow(non_snake_case)]

mod shell;

use shell::MobileApp;

fn main() {
    dioxus::launch(MobileApp);
}
