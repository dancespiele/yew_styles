use crate::styles::{get_pallete, get_size, get_style, Palette, Size, Style};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::Element;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::{utils, App};

pub struct Modal {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub header: Html,
    pub body: Html,
    #[prop_or(Callback::noop())]
    pub onclick_signal: Callback<MouseEvent>,
    #[prop_or(Callback::noop())]
    pub onkeypress_signal: Callback<KeyboardEvent>,
    #[prop_or(Palette::Standard)]
    pub modal_type: Palette,
    #[prop_or(Size::Medium)]
    pub modal_size: Size,
    #[prop_or(false)]
    pub is_open: bool,
    #[prop_or(Palette::Standard)]
    pub header_type: Palette,
    #[prop_or(Style::Regular)]
    pub header_style: Style,
    #[prop_or(false)]
    pub header_interaction: bool,
    #[prop_or(Palette::Standard)]
    pub body_type: Palette,
    #[prop_or(Style::Regular)]
    pub body_style: Style,
    #[prop_or(false)]
    pub body_interaction: bool,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
}

pub enum Msg {
    Clicked(MouseEvent),
    Pressed(KeyboardEvent),
}

impl Component for Modal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked(mouse_event) => {
                let mut console = ConsoleService::new();
                let target_event = mouse_event
                    .target()
                    .unwrap()
                    .dyn_into::<Element>()
                    .unwrap()
                    .class_name();

                console.log(&target_event);

                if target_event.starts_with("modal container") {
                    self.props.onclick_signal.emit(mouse_event);
                }
            }
            Msg::Pressed(keyboard_event) => {
                self.props.onkeypress_signal.emit(keyboard_event);
            }
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        get_modal(self.props.clone(), self.link.clone())
    }
}

fn get_modal(props: Props, link: ComponentLink<Modal>) -> Html {
    if props.is_open {
        html! {
            <div
                class=format!("modal container {} {}", get_pallete(props.modal_type), props.class_name)
                id=props.id
                onclick=link.callback(Msg::Clicked)
                onkeypress=link.callback(Msg::Pressed)
            >
                <div class=format!("modal-content {}", get_size(props.modal_size))>
                    <div class=format!(
                        "modal-header {} {} {}",
                        get_style(props.header_style),
                        get_pallete(props.header_type),
                        if props.header_interaction { "interaction" } else { "" }
                    )>
                        {props.header}
                    </div>
                    <div class=format!(
                        "modal-body {} {} {}",
                        get_style(props.body_style),
                        get_pallete(props.body_type),
                        if props.body_interaction { "interaction" } else { "" }
                    )>
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
        onclick_signal: Callback::noop(),
        onkeypress_signal: Callback::noop(),
        modal_type: Palette::Standard,
        modal_size: Size::Medium,
        header: html! {<div id="header">{"Modal Test"}</div>},
        header_style: Style::Regular,
        header_type: Palette::Standard,
        header_interaction: false,
        body: html! {<div id="body">{"Content Test"}</div>},
        body_style: Style::Regular,
        body_type: Palette::Standard,
        body_interaction: false,
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
        onclick_signal: Callback::noop(),
        onkeypress_signal: Callback::noop(),
        modal_type: Palette::Standard,
        modal_size: Size::Medium,
        header: html! {<div id="header">{"Modal Test"}</div>},
        header_style: Style::Regular,
        header_type: Palette::Standard,
        header_interaction: false,
        body: html! {<div id="body">{"Content Test"}</div>},
        body_style: Style::Regular,
        body_type: Palette::Standard,
        body_interaction: false,
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
