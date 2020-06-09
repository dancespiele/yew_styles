use crate::layouts::{
    container::{AlignContent, Container, Direction, Mode, Wrap},
    item::{Item, ItemLayout},
};
use crate::styles::{get_pallete, get_size, get_style, Palette, Size, Style};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

pub struct Card {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or(Callback::noop())]
    pub ondrag_signal: Callback<DragEvent>,
    #[prop_or(Callback::noop())]
    pub ondragend_signal: Callback<DragEvent>,
    #[prop_or(Callback::noop())]
    pub ondragenter_signal: Callback<DragEvent>,
    #[prop_or(Callback::noop())]
    pub ondragexit_signal: Callback<DragEvent>,
    #[prop_or(Callback::noop())]
    pub ondragleave_signal: Callback<DragEvent>,
    #[prop_or(Callback::noop())]
    pub ondragover_signal: Callback<DragEvent>,
    #[prop_or(Callback::noop())]
    pub ondragstart_signal: Callback<DragEvent>,
    #[prop_or(Callback::noop())]
    pub ondrop_signal: Callback<DragEvent>,
    #[prop_or(false)]
    pub draggable: bool,
    #[prop_or(None)]
    pub header: Option<Html>,
    #[prop_or(4)]
    pub header_size: i8,
    #[prop_or(None)]
    pub body: Option<Html>,
    #[prop_or(6)]
    pub body_size: i8,
    #[prop_or(None)]
    pub footer: Option<Html>,
    #[prop_or(2)]
    pub footer_size: i8,
    #[prop_or(None)]
    pub single_content: Option<Html>,
    #[prop_or(Palette::Standard)]
    pub card_type: Palette,
    #[prop_or(Style::Regular)]
    pub card_style: Style,
    #[prop_or(Size::Medium)]
    pub card_size: Size,
    #[prop_or(true)]
    pub interaction_effect: bool,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
}

pub enum Msg {
    Draged(DragEvent),
    DragedEnd(DragEvent),
    DragedEnter(DragEvent),
    DragedExit(DragEvent),
    DragedLeave(DragEvent),
    DragedOver(DragEvent),
    DragedStart(DragEvent),
    Dropped(DragEvent),
}

impl Component for Card {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Draged(drag_event) => {
                self.props.ondrag_signal.emit(drag_event);
            }
            Msg::DragedEnd(drag_event) => {
                self.props.ondragend_signal.emit(drag_event);
            }
            Msg::DragedEnter(drag_event) => {
                self.props.ondragenter_signal.emit(drag_event);
            }
            Msg::DragedExit(drag_event) => {
                self.props.ondragexit_signal.emit(drag_event);
            }
            Msg::DragedLeave(drag_event) => {
                self.props.ondragleave_signal.emit(drag_event);
            }
            Msg::DragedOver(drag_event) => {
                self.props.ondragover_signal.emit(drag_event);
            }
            Msg::DragedStart(drag_event) => {
                self.props.ondragstart_signal.emit(drag_event);
            }
            Msg::Dropped(drag_event) => {
                self.props.ondrop_signal.emit(drag_event);
            }
        };

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div
                id=self.props.id
                class=format!(
                    "card {} {} {} {} {}",
                    get_pallete(self.props.card_type.clone()),
                    if self.props.interaction_effect {
                        "interaction"
                    } else {
                        ""
                    },
                    get_size(self.props.card_size.clone()),
                    get_style(self.props.card_style.clone()),
                    self.props.class_name.clone(),
                )
                draggable = self.props.draggable
                ondrag = self.link.callback(|e| Msg::Draged(e))
                ondragend = self.link.callback(|e| Msg::Draged(e))
                ondragenter = self.link.callback(|e| Msg::Draged(e))
                ondragexit = self.link.callback(|e| Msg::Draged(e))
                ondragleave = self.link.callback(|e| Msg::Draged(e))
                ondragover = self.link.callback(|e| Msg::Draged(e))
                ondragstart = self.link.callback(|e| Msg::Draged(e))
                ondrop = self.link.callback(|e| Msg::Draged(e))
            >
                {get_content(
                    self.props.single_content.clone(),
                    self.props.header.clone(),
                    self.props.header_size,
                    self.props.body.clone(),
                    self.props.body_size,
                    self.props.footer.clone(),
                    self.props.footer_size,
                )}
            </div>
        }
    }
}

fn get_content(
    single_content: Option<Html>,
    header: Option<Html>,
    header_size: i8,
    body: Option<Html>,
    body_size: i8,
    footer: Option<Html>,
    footer_size: i8,
) -> Html {
    if let Some(single_content_node) = single_content {
        html! {
            <div class="card-single-content">
                {single_content_node}
            </div>
        }
    } else {
        html! {
            <Container class_name="card-container" wrap = Wrap::Wrap direction=Direction::Column align_content=AlignContent::Center(Mode::NoMode)>
                <Item layouts=vec!(ItemLayout::ItXs(header_size))>
                    {get_content_part(header, "card-header")}
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(body_size))>
                    {get_content_part(body, "card-body")}
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(footer_size))>
                    {get_content_part(footer, "card-footer")}
                </Item>
            </Container>
        }
    }
}

fn get_content_part(content: Option<Html>, class_content: &str) -> Html {
    if let Some(content_node) = content {
        html! {
            <div class=class_content>
                {content_node}
            </div>
        }
    } else {
        html! {}
    }
}

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn should_create_card_with_three_parts() {
    let props = Props {
        ondrag_signal: Callback::noop(),
        ondragend_signal: Callback::noop(),
        ondragenter_signal: Callback::noop(),
        ondragexit_signal: Callback::noop(),
        ondragleave_signal: Callback::noop(),
        ondragover_signal: Callback::noop(),
        ondragstart_signal: Callback::noop(),
        ondrop_signal: Callback::noop(),
        draggable: false,
        header: Some(html! {
            <div id="header">{"header"}</div>
        }),
        header_size: 4,
        body: Some(html! {
            <div id="body">{"body"}</div>
        }),
        body_size: 6,
        footer: Some(html! {
            <div id="footer">{"footer"}</div>
        }),
        footer_size: 2,
        single_content: None,
        card_type: Palette::Primary,
        card_style: Style::Regular,
        card_size: Size::Medium,
        interaction_effect: false,
        class_name: "class-card-test".to_string(),
        id: "id-card-rest".to_string(),
    };

    let card: App<Card> = App::new();
    card.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let header_element = utils::document().get_element_by_id("header").unwrap();

    let body_element = utils::document().get_element_by_id("body").unwrap();

    let footer_element = utils::document().get_element_by_id("footer").unwrap();

    assert_eq!(header_element.text_content().unwrap(), "header".to_string());

    assert_eq!(body_element.text_content().unwrap(), "body".to_string());

    assert_eq!(footer_element.text_content().unwrap(), "footer".to_string());
}

#[wasm_bindgen_test]
fn should_create_card_with_single_content() {
    let props = Props {
        ondrag_signal: Callback::noop(),
        ondragend_signal: Callback::noop(),
        ondragenter_signal: Callback::noop(),
        ondragexit_signal: Callback::noop(),
        ondragleave_signal: Callback::noop(),
        ondragover_signal: Callback::noop(),
        ondragstart_signal: Callback::noop(),
        ondrop_signal: Callback::noop(),
        draggable: false,
        header: None,
        header_size: 4,
        body: None,
        body_size: 6,
        footer: None,
        footer_size: 2,
        single_content: Some(html! {
            <div id="single-content">{"single content"}</div>
        }),
        card_type: Palette::Primary,
        card_style: Style::Regular,
        card_size: Size::Medium,
        interaction_effect: false,
        class_name: "class-card-test".to_string(),
        id: "id-card-rest".to_string(),
    };

    let card: App<Card> = App::new();
    card.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let single_content_element = utils::document()
        .get_element_by_id("single-content")
        .unwrap();

    assert_eq!(
        single_content_element.text_content().unwrap(),
        "single content".to_string()
    );
}

#[wasm_bindgen_test]
fn should_ignore_parts_when_single_content_exist() {
    let props = Props {
        ondrag_signal: Callback::noop(),
        ondragend_signal: Callback::noop(),
        ondragenter_signal: Callback::noop(),
        ondragexit_signal: Callback::noop(),
        ondragleave_signal: Callback::noop(),
        ondragover_signal: Callback::noop(),
        ondragstart_signal: Callback::noop(),
        ondrop_signal: Callback::noop(),
        draggable: false,
        header: Some(html! {
            <div id="header">{"header"}</div>
        }),
        header_size: 4,
        body: Some(html! {
            <div id="body">{"body"}</div>
        }),
        body_size: 6,
        footer: Some(html! {
            <div id="footer">{"footer"}</div>
        }),
        footer_size: 2,
        single_content: Some(html! {
            <div id="single-content">{"single content"}</div>
        }),
        card_type: Palette::Primary,
        card_style: Style::Regular,
        card_size: Size::Medium,
        interaction_effect: false,
        class_name: "class-card-test".to_string(),
        id: "id-card-rest".to_string(),
    };

    let card: App<Card> = App::new();
    card.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let header_element = utils::document().get_element_by_id("header");

    assert_eq!(header_element, None);

    let single_content_element = utils::document()
        .get_element_by_id("single-content")
        .unwrap();

    assert_eq!(
        single_content_element.text_content().unwrap(),
        "single content".to_string()
    );
}
