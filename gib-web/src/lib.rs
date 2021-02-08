mod index;
mod compoents;

use wasm_bindgen::prelude::*;
use yew::App;
use crate::index::Index;

#[wasm_bindgen]
pub fn index() {
    App::<Index>::new().mount_to_body();
}
