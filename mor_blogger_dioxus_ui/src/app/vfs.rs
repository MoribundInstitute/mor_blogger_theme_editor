use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct VfsDictionary(pub Signal<HashMap<String, String>>);
