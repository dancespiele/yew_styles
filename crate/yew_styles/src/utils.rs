extern crate rand;
extern crate wasm_bindgen;
extern crate web_sys;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
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

pub fn get_random_string(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .collect::<String>()
}

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

#[wasm_bindgen_test]
fn should_generate_random_string() {
    let mut random_values: Vec<String> = vec![];
    let mut i = 0;
    loop {
        random_values.push(get_random_string(10));
        i += 1;

        if i == 1000 {
            break;
        }
    }

    let mut i = 0;
    for value in &random_values {
        let mut index = 0;
        let repeat = &random_values.iter().any(move |random_value| {
            let exist = random_value == value && i != index;
            index += 1;
            exist
        });
        assert_eq!(*repeat, false);
        i += 1;
    }
}
