extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};

pub fn create_style(style: String, value: String, wrap: String) {
    let document = window().unwrap().document().unwrap();
    let element = document
        .get_elements_by_class_name(&wrap)
        .get_with_index(0)
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    element.style().set_property(&style, &value).unwrap();
}
