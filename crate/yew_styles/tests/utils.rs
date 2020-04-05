use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{window, HtmlElement};
use yew_styles::utils::create_style;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn should_set_style_prop() {
    let body = window().unwrap().document().unwrap().body().unwrap();

    let element = window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();

    element.set_text_content(Some("item"));
    element.set_class_name("item");
    body.append_child(&element).unwrap();

    create_style(
        "padding".to_string(),
        "10px".to_string(),
        "item".to_string(),
    );

    let item = window()
        .unwrap()
        .document()
        .unwrap()
        .get_elements_by_class_name("item")
        .get_with_index(0)
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    let value = item.style().get_property_value("padding").unwrap();

    assert_eq!(value, "10px");
}
