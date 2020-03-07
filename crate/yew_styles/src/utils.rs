#[cfg(feature = "std_web")]
extern crate stdweb;
extern crate wasm_bindgen;
#[cfg(feature = "web_sys")]
extern crate web_sys;

#[cfg(feature = "std_web")]
use stdweb::js;
#[cfg(feature = "web_sys")]
use wasm_bindgen::JsCast;
#[cfg(feature = "web_sys")]
use web_sys::{window, HtmlElement};

pub fn create_style(style: String, value: String, wrap: String) {
    #[cfg(feature = "web_sys")]
    {
        let document = window().unwrap().document().unwrap();
        let element = document
            .get_elements_by_class_name(&wrap)
            .get_with_index(0)
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();

        element.style().set_property(&style, &value).unwrap();
    }

    #[cfg(feature = "std_web")]
    js! {
        const element = document.getElementsByClassName(@{wrap})[0];
        element.style[@{style}] = @{value};
    };
}
