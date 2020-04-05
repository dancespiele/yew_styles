use wasm_bindgen_test::*;
use web_sys::window;
use yew::prelude::*;
use yew_styles::navbar::navbar_item::{NavbarItem, Props};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn should_create_navbar_item() {
    let navbar_item_props = Props {
        class_name: "navbar-item-test".to_string(),
        onsignal: Callback::noop(),
        children: Children::new(vec![html! {
            <div id="item">{"Item"}</div>
        }]),
    };

    let link = ComponentLink::new();
    let navbar_item = NavbarItem::create(navbar_item_props, link.clone());
    let navbar_item_vnode = navbar_item.render();

    let vnode_expected = html! {
        <div
            onclick=Callback::noop()
            class="navbar-item navbar-item-test">
            <>
                <div id="item">
                    {"Item"}
                </div>
            </>
        </div>
    };

    assert_eq!(navbar_item_vnode, vnode_expected);
}

#[wasm_bindgen_test]
fn should_create_clickable_navbar_item() {
    let on_add_item_div = Callback::from(|_| {
        let body = window().unwrap().document().unwrap().body().unwrap();

        let child_element = window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();

        child_element.set_text_content(Some("item2"));
        child_element.set_id("item2");
        body.append_child(&child_element).unwrap();
    });

    let navbar_item_props = Props {
        class_name: "navbar-item-test".to_string(),
        onsignal: on_add_item_div,
        children: Children::new(vec![html! {
            <div id="item">{"Item"}</div>
        }]),
    };

    navbar_item_props.onsignal.emit(());

    let updated_content = window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("item2")
        .unwrap()
        .text_content()
        .unwrap();

    assert_eq!(updated_content, "item2".to_string());
}
