use wasm_bindgen_test::*;
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

    let layout = Container::create(props_container, link.clone());

    let layout_vnode = layout.render();

    let vnode_expected = html! {
        <div class="container container-layout-test-0 layout-test">
            <>
                <div id="container">{"Container"}</div>
            </>
        </div>
    };

    assert_eq!(layout_vnode, vnode_expected);
}
