use crate::styles::{get_pallete, get_size, get_style, Palette, Size, Style};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};
use yew_assets::editing_assets::{EditingAssets, EditingIcon};

pub struct Text {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone)]
pub enum TextType {
    Plain,
    Paragraph,
    Alert,
    Tag,
}

#[derive(Clone, Properties)]
pub struct Props {
    /// Text to show
    pub text: String,
    /// How is showing the text
    pub text_content_type: TextType,
    /// if hove, focus, active effects are enable, only for tag type
    #[prop_or(true)]
    pub interaction_effect: bool,
    /// A dragged item (element or text selection) is dragged, only for tag type
    #[prop_or(Callback::noop())]
    pub ondrag_signal: Callback<DragEvent>,
    /// A drag operation ends, only for tag type
    #[prop_or(Callback::noop())]
    pub ondragend_signal: Callback<DragEvent>,
    /// A dragged item enters a valid drop target, only for tag type
    #[prop_or(Callback::noop())]
    pub ondragenter_signal: Callback<DragEvent>,
    /// An element is no longer the drag operation's immediate selection target, only for tag type
    #[prop_or(Callback::noop())]
    pub ondragexit_signal: Callback<DragEvent>,
    /// A dragged item leaves a valid drop target, only for tag type
    #[prop_or(Callback::noop())]
    pub ondragleave_signal: Callback<DragEvent>,
    /// A dragged item is being dragged over a valid drop target
    /// Every few hundred milliseconds, only for tag type
    #[prop_or(Callback::noop())]
    pub ondragover_signal: Callback<DragEvent>,
    /// The user starts dragging an item, only for tag type
    #[prop_or(Callback::noop())]
    pub ondragstart_signal: Callback<DragEvent>,
    /// An item is dropped on a valid drop target, only for tag type
    #[prop_or(Callback::noop())]
    pub ondrop_signal: Callback<DragEvent>,
    /// Click event only for text tag type
    #[prop_or(Callback::noop())]
    pub onclick_signal: Callback<MouseEvent>,
    /// Click event only for text tag type with removable in true
    #[prop_or(Callback::noop())]
    pub ondelete_signal: Callback<MouseEvent>,
    /// If the item is draggable, only for tag type
    #[prop_or(false)]
    pub draggable: bool,
    /// If the tag can be seleted
    #[prop_or(false)]
    pub removable: bool,
    /// Type text purpose style, only for tag and alert type
    #[prop_or(Palette::Standard)]
    pub text_type: Palette,
    /// Text styles, only for tag and alert type
    #[prop_or(Style::Regular)]
    pub text_style: Style,
    /// Three diffent text standard sizes
    #[prop_or(Size::Medium)]
    pub text_size: Size,
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
    Clicked(MouseEvent),
    Deleted(MouseEvent),
}

impl Component for Text {
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
            Msg::Clicked(mouse_event) => self.props.onclick_signal.emit(mouse_event),
            Msg::Deleted(mouse_event) => self.props.ondelete_signal.emit(mouse_event),
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        get_text(
            self.props.text_content_type.clone(),
            self.props.clone(),
            self.link.clone(),
        )
    }
}

fn get_text(text_content_type: TextType, props: Props, link: ComponentLink<Text>) -> Html {
    match text_content_type {
        TextType::Plain => {
            html! {
                <span
                    class=format!("plain-text {} {}", get_size(props.text_size), props.class_name)
                    id=props.id
                >{props.text}</span>
            }
        }
        TextType::Paragraph => {
            html! {
                <p
                    class=format!("paragraph-text {} {}", get_size(props.text_size), props.class_name)
                    id=props.id
                >{props.text}</p>
            }
        }
        TextType::Alert => {
            html! {
                <div
                    class=format!(
                        "alert-text {} {} {} {}",
                        get_style(props.text_style),
                        get_pallete(props.text_type),
                        get_size(props.text_size),
                        props.class_name,
                    )
                    id =props.id
                >
                    <span>{props.text}</span>
                </div>
            }
        }
        TextType::Tag => {
            html! {
                <div
                    class=format!(
                        "tag-text {} {} {} {} {}",
                        if props.interaction_effect {
                            "interaction"
                        } else {
                            ""
                        },
                        get_style(props.text_style),
                        get_pallete(props.text_type),
                        get_size(props.text_size),
                        props.class_name,
                    )
                    id =props.id
                    draggable = props.draggable
                    ondrag = link.callback(Msg::Draged)
                    ondragend = link.callback(Msg::DragedEnd)
                    ondragenter = link.callback(Msg::DragedEnter)
                    ondragexit = link.callback(Msg::DragedExit)
                    ondragleave = link.callback(Msg::DragedLeave)
                    ondragover = link.callback(Msg::DragedOver)
                    ondragstart = link.callback(Msg::DragedStart)
                    ondrop = link.callback(Msg::Dropped)
                    onclick = link.callback(Msg::Clicked)
                >
                    <span>{props.text}</span>
                    {if props.removable {
                        html!{
                            <div
                                class="tag-delete"
                                onclick= link.callback(Msg::Deleted)
                            >
                                <EditingAssets
                                    icon=EditingIcon::X
                                />
                            </div>
                        }
                    } else {
                        html!{}
                    }}
                </div>

            }
        }
    }
}

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn should_create_plain_text() {
    let props = Props {
        text_content_type: TextType::Plain,
        ondrag_signal: Callback::noop(),
        ondragend_signal: Callback::noop(),
        ondragenter_signal: Callback::noop(),
        ondragexit_signal: Callback::noop(),
        ondragleave_signal: Callback::noop(),
        ondragover_signal: Callback::noop(),
        ondragstart_signal: Callback::noop(),
        ondrop_signal: Callback::noop(),
        onclick_signal: Callback::noop(),
        ondelete_signal: Callback::noop(),
        draggable: false,
        removable: false,
        text: "hello test".to_string(),
        text_type: Palette::Primary,
        text_style: Style::Regular,
        text_size: Size::Medium,
        interaction_effect: false,
        class_name: "class-card-test".to_string(),
        id: "id-text-test".to_string(),
    };

    let text: App<Text> = App::new();

    text.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let plain_text_element = utils::document()
        .get_elements_by_class_name("plain-text")
        .get_with_index(0);

    assert_eq!(plain_text_element.is_some(), true);
}

#[wasm_bindgen_test]
fn should_create_paragraph_text() {
    let props = Props {
        text_content_type: TextType::Paragraph,
        ondrag_signal: Callback::noop(),
        ondragend_signal: Callback::noop(),
        ondragenter_signal: Callback::noop(),
        ondragexit_signal: Callback::noop(),
        ondragleave_signal: Callback::noop(),
        ondragover_signal: Callback::noop(),
        ondragstart_signal: Callback::noop(),
        ondrop_signal: Callback::noop(),
        onclick_signal: Callback::noop(),
        ondelete_signal: Callback::noop(),
        draggable: false,
        removable: false,
        text: "hello test".to_string(),
        text_type: Palette::Primary,
        text_style: Style::Regular,
        text_size: Size::Medium,
        interaction_effect: false,
        class_name: "class-card-test".to_string(),
        id: "id-text-test".to_string(),
    };

    let text: App<Text> = App::new();

    text.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let paragraph_text_element = utils::document()
        .get_elements_by_class_name("paragraph-text")
        .get_with_index(0);

    assert_eq!(paragraph_text_element.is_some(), true);
}

#[wasm_bindgen_test]
fn should_create_alert_text() {
    let props = Props {
        text_content_type: TextType::Alert,
        ondrag_signal: Callback::noop(),
        ondragend_signal: Callback::noop(),
        ondragenter_signal: Callback::noop(),
        ondragexit_signal: Callback::noop(),
        ondragleave_signal: Callback::noop(),
        ondragover_signal: Callback::noop(),
        ondragstart_signal: Callback::noop(),
        ondrop_signal: Callback::noop(),
        onclick_signal: Callback::noop(),
        ondelete_signal: Callback::noop(),
        draggable: false,
        removable: false,
        text: "hello test".to_string(),
        text_type: Palette::Primary,
        text_style: Style::Regular,
        text_size: Size::Medium,
        interaction_effect: false,
        class_name: "class-card-test".to_string(),
        id: "id-text-test".to_string(),
    };

    let text: App<Text> = App::new();

    text.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let alert_text_element = utils::document()
        .get_elements_by_class_name("alert-text")
        .get_with_index(0);

    assert_eq!(alert_text_element.is_some(), true);
}

#[wasm_bindgen_test]
fn should_create_tag_text() {
    let props = Props {
        text_content_type: TextType::Tag,
        ondrag_signal: Callback::noop(),
        ondragend_signal: Callback::noop(),
        ondragenter_signal: Callback::noop(),
        ondragexit_signal: Callback::noop(),
        ondragleave_signal: Callback::noop(),
        ondragover_signal: Callback::noop(),
        ondragstart_signal: Callback::noop(),
        ondrop_signal: Callback::noop(),
        onclick_signal: Callback::noop(),
        ondelete_signal: Callback::noop(),
        draggable: false,
        removable: false,
        text: "hello test".to_string(),
        text_type: Palette::Primary,
        text_style: Style::Regular,
        text_size: Size::Medium,
        interaction_effect: false,
        class_name: "class-card-test".to_string(),
        id: "id-text-test".to_string(),
    };

    let text: App<Text> = App::new();

    text.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let tag_text_element = utils::document()
        .get_elements_by_class_name("tag-text")
        .get_with_index(0);

    assert_eq!(tag_text_element.is_some(), true);
}

#[wasm_bindgen_test]
fn should_add_delete_icon_tag_text() {
    let props = Props {
        text_content_type: TextType::Tag,
        ondrag_signal: Callback::noop(),
        ondragend_signal: Callback::noop(),
        ondragenter_signal: Callback::noop(),
        ondragexit_signal: Callback::noop(),
        ondragleave_signal: Callback::noop(),
        ondragover_signal: Callback::noop(),
        ondragstart_signal: Callback::noop(),
        ondrop_signal: Callback::noop(),
        onclick_signal: Callback::noop(),
        ondelete_signal: Callback::noop(),
        draggable: false,
        removable: true,
        text: "hello test".to_string(),
        text_type: Palette::Primary,
        text_style: Style::Regular,
        text_size: Size::Medium,
        interaction_effect: false,
        class_name: "class-card-test".to_string(),
        id: "id-text-test".to_string(),
    };

    let text: App<Text> = App::new();

    text.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let tag_text_element = utils::document()
        .get_elements_by_class_name("tag-delete")
        .get_with_index(0);

    assert_eq!(tag_text_element.is_some(), true);
}
