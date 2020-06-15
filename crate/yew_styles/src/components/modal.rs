use crate::styles::{get_pallete, get_style, Palette, Style};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

pub struct Modal {
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub header: Html,
    pub body: Html,
    #[prop_or(false)]
    pub is_open: bool,
    #[prop_or(Palette::Standard)]
    pub header_type: Palette,
    #[prop_or(Style::Regular)]
    pub header_style: Style,
    #[prop_or(Palette::Standard)]
    pub body_type: Palette,
    #[prop_or(Style::Regular)]
    pub body_style: Style,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
}

impl Component for Modal {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        get_modal(self.props.clone())
    }
}

fn get_modal(props: Props) -> Html {
    if props.is_open {
        html! {
            <div
                class=format!("modal {}",props.class_name)
                id=props.id
            >
                <div class="modal-content">
                    <div class=format!("modal-header {} {}", get_style(props.header_style), get_pallete(props.header_type))>
                        {props.header}
                    </div>
                    <div class=format!("modal-body {} {}", get_style(props.body_style), get_pallete(props.body_type))>
                        {props.body}
                    </div>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn should_create_modal_component() {
    let props = Props {
        class_name: "test-modal".to_string(),
        id: "modal-id-test".to_string(),
        header: html! {<div id="header">{"Modal Test"}</div>},
        header_style: Style::Regular,
        header_type: Palette::Standard,
        body: html! {<div id="body">{"Content Test"}</div>},
        body_style: Style::Regular,
        body_type: Palette::Standard,
        is_open: true,
    };

    let modal: App<Modal> = App::new();

    modal.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let modal_header_element = utils::document().get_element_by_id("header").unwrap();

    let modal_body_element = utils::document().get_element_by_id("body").unwrap();

    assert_eq!(modal_header_element.text_content().unwrap(), "Modal Test");
    assert_eq!(modal_body_element.text_content().unwrap(), "Content Test");
}

#[wasm_bindgen_test]
fn should_hide_modal_component_from_doom() {
    let props = Props {
        class_name: "test-modal".to_string(),
        id: "modal-id-test".to_string(),
        header: html! {<div id="header">{"Modal Test"}</div>},
        header_style: Style::Regular,
        header_type: Palette::Standard,
        body: html! {<div id="body">{"Content Test"}</div>},
        body_style: Style::Regular,
        body_type: Palette::Standard,
        is_open: false,
    };

    let modal: App<Modal> = App::new();

    modal.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let modal_element = utils::document().get_element_by_id("modal-id-test");

    assert_eq!(modal_element, None);
}
