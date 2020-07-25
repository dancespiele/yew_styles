use crate::styles::{get_pallete, get_size, get_style, Palette, Size, Style};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};
use yew_assets::editing_assets::{EditingAssets, EditingIcon};

/// # Text
///
/// ## Features required
///
/// text
///
/// ## Example
///
/// ```rust
/// use lipsum::lipsum;
/// use wasm_bindgen::JsCast;
/// use web_sys::Element;
/// use yew::prelude::*;
/// use yew::utils;
/// use yew_styles::layouts::{
///     container::{Container, Direction, Wrap},
///     item::{Item, ItemLayout},
/// };
/// use yew_styles::styles::{Palette, Size, Style};
/// use yew_styles::text::{Text, TextType};
///
/// pub enum Msg {
///   Dragged(DragEvent),
///   DraggedOver(DragEvent),
///   Dropped(DragEvent),
/// }
///
/// pub struct TextExample {
///  link: ComponentLink<Self>,
/// }
///
/// impl Component for TextPage {
///     type Message = Msg;
///     type Properties = ();
///
///     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
///         TextPage { link }
///     }
///
///     fn update(&mut self, msg: Self::Message) -> ShouldRender {
///         match msg {
///             Msg::Dragged(drag_event) => {
///                 let target_id = drag_event
///                     .target()
///                     .unwrap()
///                     .dyn_into::<Element>()
///                     .unwrap()
///                     .id();
///
///                 drag_event
///                     .data_transfer()
///                     .unwrap()
///                     .set_data("application/text-component", &target_id)
///                     .unwrap();
///
///                 drag_event.data_transfer().unwrap().set_drop_effect("move");
///             }
///             Msg::DraggedOver(drag_event) => {
///                 drag_event.prevent_default();
///
///                 drag_event.data_transfer().unwrap().set_drop_effect("move");
///             }
///
///             Msg::Dropped(drag_event) => {
///                 drag_event.prevent_default();
///
///                 let data = drag_event
///                     .data_transfer()
///                     .unwrap()
///                     .get_data("application/text-component")
///                     .unwrap();
///
///                 let target_element = drag_event.target().unwrap().dyn_into::<Element>().unwrap();
///
///                 target_element
///                     .append_child(&utils::document().get_element_by_id(&data).unwrap())
///                     .unwrap();
///             }
///         };
///         true
///     }
///
///     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
///         false
///     }
///
///     fn view(&self) -> Html {
///         html! {
///             <Container wrap = Wrap::Wrap direction = Direction::Row>
///                 <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
///                     <div
///                         ondrop=self.link.callback(|e| Msg::Dropped(e))
///                         ondragover=self.link.callback(|e| Msg::DraggedOver(e))
///                         class="tag-box"
///                     >
///                         {get_draggable_tags(self.link.clone())}
///                     </div>
///                 </Item>
///                 <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
///                     <div ondrop=self.link.callback(Msg::Dropped)
///                         ondragover=self.link.callback(Msg::DraggedOver)
///                         class="tag-box">
///                     </div>
///                 </Item>
///             </Container>
///         }
///     }
/// }
///
/// fn get_draggable_tags(link: ComponentLink<TextPage>) -> Html {
///     let styles = vec![Style::Regular, Style::Outline, Style::Light];
///     let palette = vec![Palette::Success, Palette::Warning, Palette::Danger];
///     let mut index = 0;
///     
///     styles
///         .into_iter()
///         .map(|style| {
///             palette
///                 .clone()
///                 .into_iter()
///                 .map(|item_palette| {
///                     let text_view = html! {
///                         <Text
///                             class_name="draggable-tag"
///                             id=format!("tag-{}", index)
///                             draggable=true
///                             ondragstart_signal=link.callback(Msg::Dragged)
///                             text_type=TextType::Tag
///                             text_size=Size::Medium
///                             text=lipsum(1).replace(".", "")
///                             text_style=style.clone()
///                             text_palette=item_palette
///                             interaction_effect= true
///                         />
///                     };
///     
///                     index += 1;
///     
///                     text_view
///                 })
///                 .collect::<Html>()
///     })
///     .collect::<Html>()
/// }
///
/// ```
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
    pub text_type: TextType,
    /// if hove, focus, active effects are enable, only for tag type
    #[prop_or(false)]
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
    /// If the tag can be deleted
    #[prop_or(false)]
    pub removable: bool,
    /// Type text purpose style, only for tag and alert type
    #[prop_or(Palette::Standard)]
    pub text_palette: Palette,
    /// Text styles, only for tag and alert type
    #[prop_or(Style::Regular)]
    pub text_style: Style,
    /// Three diffent text standard sizes
    #[prop_or(Size::Medium)]
    pub text_size: Size,
    /// General property to get the ref of the component
    #[prop_or_default]
    pub code_ref: NodeRef,
    /// General property to add keys
    #[prop_or_default]
    pub key: String,
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
            self.props.text_type.clone(),
            self.props.clone(),
            self.link.clone(),
        )
    }
}

fn get_text(text_type: TextType, props: Props, link: ComponentLink<Text>) -> Html {
    match text_type {
        TextType::Plain => {
            html! {
                <span
                    class=format!("plain-text {} {}", get_size(props.text_size), props.class_name)
                    id=props.id
                    key=props.key
                    ref=props.code_ref
                >{props.text}</span>
            }
        }
        TextType::Paragraph => {
            html! {
                <p
                    class=format!("paragraph-text {} {}", get_size(props.text_size), props.class_name)
                    id=props.id
                    key=props.key
                    ref=props.code_ref
                >{props.text}</p>
            }
        }
        TextType::Alert => {
            html! {
                <div
                    class=format!(
                        "alert-text {} {} {} {}",
                        get_style(props.text_style),
                        get_pallete(props.text_palette),
                        get_size(props.text_size),
                        props.class_name,
                    )
                    id =props.id
                    key=props.key
                    ref=props.code_ref
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
                        get_pallete(props.text_palette),
                        get_size(props.text_size.clone()),
                        props.class_name,
                    )
                    id =props.id
                    key=props.key
                    ref=props.code_ref
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
                                    size=if props.text_size == Size::Medium {
                                        ("20".to_string(), "20".to_string())
                                    } else if props.text_size == Size::Small {
                                        ("13".to_string(), "13".to_string())
                                    } else {
                                        ("24".to_string(), "24".to_string())
                                    }
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
        text_type: TextType::Plain,
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
        text_palette: Palette::Primary,
        text_style: Style::Regular,
        text_size: Size::Medium,
        interaction_effect: false,
        key: "".to_string(),
        code_ref: NodeRef::default(),
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
        text_type: TextType::Paragraph,
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
        text_palette: Palette::Primary,
        text_style: Style::Regular,
        text_size: Size::Medium,
        interaction_effect: false,
        key: "".to_string(),
        code_ref: NodeRef::default(),
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
        text_type: TextType::Alert,
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
        text_palette: Palette::Primary,
        text_style: Style::Regular,
        text_size: Size::Medium,
        interaction_effect: false,
        key: "".to_string(),
        code_ref: NodeRef::default(),
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
        text_type: TextType::Tag,
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
        text_palette: Palette::Primary,
        text_style: Style::Regular,
        text_size: Size::Medium,
        interaction_effect: false,
        key: "".to_string(),
        code_ref: NodeRef::default(),
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
        text_type: TextType::Tag,
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
        text_palette: Palette::Primary,
        text_style: Style::Regular,
        text_size: Size::Medium,
        interaction_effect: false,
        key: "".to_string(),
        code_ref: NodeRef::default(),
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
