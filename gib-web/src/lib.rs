#![recursion_limit = "1024"]

mod components;
mod index;
mod util;

use crate::index::Index;
use wasm_bindgen::prelude::*;
use yew::App;

#[wasm_bindgen]
pub fn index() {
    App::<Index>::new().mount_to_body();
}
