use wasm_bindgen_test::*;
use web_sys::window;
use yew::prelude::*;
use yew_styles::{
    container::{
        AlignContent, AlignItems, Container, Direction, JustifyContent, Mode,
        Props as PropsContainer, Wrap,
    },
    item::{AlignSelf, Item, ItemLayout, Props as PropsItem},
};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn should_create_a_container() {
    let props_container = PropsContainer {
        direction: Direction::Row,
        wrap: Wrap::Wrap,
        index: 0,
        justify_content: JustifyContent::Center(Mode::NoMode),
        align_content: AlignContent::Center(Mode::NoMode),
        align_items: AlignItems::Center(Mode::NoMode),
        class_name: String::from("layout-test"),
        name: String::from("layout-test"),
        children: Children::new(vec![html! {
            <div id="container">{"Container"}</div>
        }]),
    };

    let link = ComponentLink::new();

    let container = Container::create(props_container, link.clone());

    let container_vnode = container.render();

    let vnode_expected = html! {
        <div class="container container-layout-test-0 layout-test">
            <>
                <div id="container">{"Container"}</div>
            </>
        </div>
    };

    assert_eq!(container_vnode, vnode_expected);
}

#[wasm_bindgen_test]
fn should_create_item() {
    let props_item = PropsItem {
        layouts: vec![ItemLayout::ItXs(12)],
        align_self: AlignSelf::Center,
        name: "item-test".to_string(),
        class_name: "item-test".to_string(),
        index: 0,
        onsignal: Callback::noop(),
        children: Children::new(vec![html! {
            <div id="item">{"Item"}</div>
        }]),
    };

    let link = ComponentLink::new();

    let item = Item::create(props_item, link.clone());

    let item_vnode = item.render();

    let vnode_expected = html! {
        <div
            onclick=Callback::noop()
            class="item item-item-test-0 it-xs-12 item-test">
            <>
                <div id="item">{"Item"}</div>
            </>
        </div>
    };

    assert_eq!(item_vnode, vnode_expected);
}

#[wasm_bindgen_test]
fn should_create_clickable_item() {
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

    let props_item = PropsItem {
        layouts: vec![ItemLayout::ItXs(12)],
        align_self: AlignSelf::Center,
        name: "item-test".to_string(),
        class_name: "item-test".to_string(),
        index: 0,
        onsignal: on_add_item_div,
        children: Children::new(vec![html! {
            <div id="item">{"Item"}</div>
        }]),
    };

    props_item.onsignal.emit(());

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
