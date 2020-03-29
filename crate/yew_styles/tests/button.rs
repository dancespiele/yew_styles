#![cfg(target_arch = "wasm32")]
extern crate wasm_bindgen_test;
extern crate yew_styles;

use wasm_bindgen_test::*;
use web_sys::window;
use yew::prelude::*;
use yew_styles::{
    button::{Button, Msg, Props, Size},
    styles::{Palette, Style},
};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn should_trigger_action_when_button_clicked() {
    let body = window().unwrap().document().unwrap().body().unwrap();

    let element = window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    element.set_text_content(Some("home"));
    element.set_id("menu");

    body.append_child(&element).unwrap();

    let onchange_name = Callback::from(|_| {
        let content = window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("menu")
            .unwrap();

        content.set_text_content(Some("about"));
    });

    let props = Props {
        class_name: String::from("test-button"),
        size: Size::Medium,
        button_style: Style::Regular,
        onsignal: onchange_name,
        button_type: Palette::Standard,
        children: Children::new(vec![html! {<div id="submenu">{"another menu"}</div>}]),
    };

    let link = ComponentLink::new();

    let mut button = Button::create(props.clone(), link);

    props.onsignal.emit(());

    button.change(props);

    let updated_content = window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("menu")
        .unwrap()
        .text_content()
        .unwrap();

    assert_eq!(updated_content, String::from("about"));
}

#[wasm_bindgen_test]
fn should_create_button_component() {
    use web_sys::window;

    let on_add_child = Callback::from(|_| {
        let body = window().unwrap().document().unwrap().body().unwrap();

        let child_element = window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();

        child_element.set_text_content(Some("child"));
        child_element.set_id("child");
        body.append_child(&child_element).unwrap();
    });

    let props = Props {
        class_name: String::from("test-button"),
        size: Size::Medium,
        button_style: Style::Regular,
        onsignal: on_add_child.clone(),
        button_type: Palette::Standard,
        children: Children::new(vec![html! {<div id="parent">{"parent"}</div>}]),
    };

    let link = ComponentLink::new();

    let mut button = Button::create(props.clone(), link.clone());

    props.onsignal.emit(());

    button.change(props);

    let button_vnode = button.render();

    let vnode_expected = html! {
        <button
            onclick=link.callback(|_| Msg::Clicked)
            class="button standard medium regular test-button">
            <>
                <div id="parent">{"parent"}</div>
            </>
        </button>
    };

    assert_eq!(button_vnode, vnode_expected);
}
