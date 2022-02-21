use crate::styles::helpers::{get_palette, get_size, get_style, Palette, Size, Style};
use gloo::utils;
use stylist::{css, StyleSource};
use wasm_bindgen_test::*;
use yew::html::Scope;
use yew::prelude::*;
use yew::start_app_with_props;
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
/// use yew_styles::styles::helpers::{Palette, Size, Style};
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
///                             plain_text=Some(lipsum(1).replace(".", ""))
///                             html_text=None
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
    props: Props,
}

#[derive(Clone, PartialEq)]
pub enum Header {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Clone, PartialEq)]
pub enum TextType {
    Title(Header),
    Plain,
    Paragraph,
    Alert,
    Tag,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// How is showing the text. Required
    pub text_type: TextType,
    /// Text plain to show. Required
    #[prop_or_default]
    pub plain_text: String,
    /// Text in html to show. Required
    pub html_text: Option<Html>,
    /// if hove, focus, active effects are enable. Only for tag type. Default `false`
    #[prop_or(false)]
    pub interaction_effect: bool,
    /// A dragged item (element or text selection) is dragged. Only for tag type
    #[prop_or(Callback::noop())]
    pub ondrag_signal: Callback<DragEvent>,
    /// A drag operation ends. Only for tag type
    #[prop_or(Callback::noop())]
    pub ondragend_signal: Callback<DragEvent>,
    /// A dragged item enters a valid drop target. Only for tag type
    #[prop_or(Callback::noop())]
    pub ondragenter_signal: Callback<DragEvent>,
    /// An element is no longer the drag operation's immediate selection target. Only for tag type
    #[prop_or(Callback::noop())]
    pub ondragexit_signal: Callback<DragEvent>,
    /// A dragged item leaves a valid drop target. Only for tag type
    #[prop_or(Callback::noop())]
    pub ondragleave_signal: Callback<DragEvent>,
    /// A dragged item is being dragged over a valid drop target
    /// Every few hundred milliseconds. Only for tag type
    #[prop_or(Callback::noop())]
    pub ondragover_signal: Callback<DragEvent>,
    /// The user starts dragging an item. Only for tag type
    #[prop_or(Callback::noop())]
    pub ondragstart_signal: Callback<DragEvent>,
    /// An item is dropped on a valid drop target. Only for tag type
    #[prop_or(Callback::noop())]
    pub ondrop_signal: Callback<DragEvent>,
    /// Click event only for text tag type
    #[prop_or(Callback::noop())]
    pub onclick_signal: Callback<MouseEvent>,
    /// Click event only for text tag type with removable in true
    #[prop_or(Callback::noop())]
    pub ondelete_signal: Callback<MouseEvent>,
    /// If the item is draggable. Only for tag type. Default `false`
    #[prop_or(false)]
    pub draggable: bool,
    /// If the tag can be deleted. Default `false`
    #[prop_or(false)]
    pub removable: bool,
    /// Type text purpose style. Only for tag and alert type. Default `Palette::Standard`
    #[prop_or(Palette::Standard)]
    pub text_palette: Palette,
    /// Text styles. Only for tag and alert type. Default `Style::Regular`
    #[prop_or(Style::Regular)]
    pub text_style: Style,
    /// Three diffent text standard sizes. Not for title type. Default `Size::Medium`
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
    /// Set css styles directly in the component
    #[prop_or(css!(""))]
    pub styles: StyleSource<'static>,
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

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: *ctx.props(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Draged(drag_event) => {
                ctx.props().ondrag_signal.emit(drag_event);
            }
            Msg::DragedEnd(drag_event) => {
                ctx.props().ondragend_signal.emit(drag_event);
            }
            Msg::DragedEnter(drag_event) => {
                ctx.props().ondragenter_signal.emit(drag_event);
            }
            Msg::DragedExit(drag_event) => {
                ctx.props().ondragexit_signal.emit(drag_event);
            }
            Msg::DragedLeave(drag_event) => {
                ctx.props().ondragleave_signal.emit(drag_event);
            }
            Msg::DragedOver(drag_event) => {
                ctx.props().ondragover_signal.emit(drag_event);
            }
            Msg::DragedStart(drag_event) => {
                ctx.props().ondragstart_signal.emit(drag_event);
            }
            Msg::Dropped(drag_event) => {
                ctx.props().ondrop_signal.emit(drag_event);
            }
            Msg::Clicked(mouse_event) => ctx.props().onclick_signal.emit(mouse_event),
            Msg::Deleted(mouse_event) => ctx.props().ondelete_signal.emit(mouse_event),
        }

        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props = *ctx.props();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        get_text(
            ctx.props().text_type.clone(),
            ctx.props().clone(),
            ctx.link().clone(),
        )
    }
}

fn get_content(plain_text: String, html_text: Option<Html>) -> Html {
    if !plain_text.is_empty() {
        html! { <>{plain_text}</> }
    } else if let Some(html_text_value) = html_text {
        html_text_value
    } else {
        panic!("One of the props plain_text or html_text should be something");
    }
}

fn get_text(text_type: TextType, props: Props, link: Scope<Text>) -> Html {
    match text_type {
        TextType::Title(header) => get_header(header, props),
        TextType::Plain => {
            html! {
                <span
                    class={classes!("plain-text", get_size(props.text_size), props.class_name, props.styles)}
                    id={props.id}
                    key={props.key}
                    ref={props.code_ref}
                >{get_content(props.plain_text, props.html_text)}</span>
            }
        }
        TextType::Paragraph => {
            html! {
                <p
                    class={classes!("paragraph-text", get_size(props.text_size), props.class_name, props.styles)}
                    id={props.id}
                    key={props.key}
                    ref={props.code_ref}
                >{get_content(props.plain_text, props.html_text)}</p>
            }
        }
        TextType::Alert => {
            html! {
                <div
                    class={classes!(
                        "alert-text",
                        get_style(props.text_style),
                        get_palette(props.text_palette),
                        get_size(props.text_size),
                        props.class_name,
                        props.styles
                    )}
                    id ={props.id}
                    key={props.key}
                    ref={props.code_ref}
                >
                    <span>{get_content(props.plain_text, props.html_text)}</span>
                </div>
            }
        }
        TextType::Tag => {
            html! {
                <div
                    class={classes!(
                        "tag-text",
                        if props.interaction_effect {
                            "interaction"
                        } else {
                            ""
                        },
                        get_style(props.text_style),
                        get_palette(props.text_palette),
                        get_size(props.text_size.clone()),
                        props.class_name,
                        props.styles,
                    )}
                    id ={props.id}
                    key={props.key}
                    ref={props.code_ref}
                    draggable = {props.draggable.to_string()}
                    ondrag = {link.callback(Msg::Draged)}
                    ondragend = {link.callback(Msg::DragedEnd)}
                    ondragenter = {link.callback(Msg::DragedEnter)}
                    ondragexit = {link.callback(Msg::DragedExit)}
                    ondragleave = {link.callback(Msg::DragedLeave)}
                    ondragover = {link.callback(Msg::DragedOver)}
                    ondragstart = {link.callback(Msg::DragedStart)}
                    ondrop = {link.callback(Msg::Dropped)}
                    onclick = {link.callback(Msg::Clicked)}
                >
                    <span>{get_content(props.plain_text, props.html_text)}</span>
                    {if props.removable {
                        html!{
                            <div
                                class="tag-delete"
                                onclick= {link.callback(Msg::Deleted)}
                            >
                                <EditingAssets
                                    icon={EditingIcon::X}
                                    size={if props.text_size == Size::Medium {
                                        ("20".to_string(), "20".to_string())
                                    } else if props.text_size == Size::Small {
                                        ("13".to_string(), "13".to_string())
                                    } else {
                                        ("24".to_string(), "24".to_string())
                                    }}
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

fn get_header(header: Header, props: Props) -> Html {
    match header {
        Header::H1 => html! {<h1
            class={classes!("header-text", props.class_name, props.styles)}
            id={props.id}
            key={props.key}
            ref={props.code_ref}
        >{get_content(props.plain_text, props.html_text)}</h1>},
        Header::H2 => html! {<h2
            class={classes!("header-text", props.class_name, props.styles)}
            id={props.id}
            key={props.key}
            ref={props.code_ref}
        >{get_content(props.plain_text, props.html_text)}</h2>},
        Header::H3 => html! {<h3
            class={classes!("header-text", props.class_name, props.styles)}
            id={props.id}
            key={props.key}
            ref={props.code_ref}
        >{get_content(props.plain_text, props.html_text)}</h3>},
        Header::H4 => html! {<h4
            class={classes!("header-text", props.class_name, props.styles)}
            id={props.id}
            key={props.key}
            ref={props.code_ref}
        >{get_content(props.plain_text, props.html_text)}</h4>},
        Header::H5 => html! {<h5
            class={classes!("header-text", props.class_name, props.styles)}
            id={props.id}
            key={props.key}
            ref={props.code_ref}
        >{get_content(props.plain_text, props.html_text)}</h5>},
        Header::H6 => html! {<h6
            class={classes!("header-text", props.class_name, props.styles)}
            id={props.id}
            key={props.key}
            ref={props.code_ref}
        >{get_content(props.plain_text, props.html_text)}</h6>},
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
        plain_text: "hello test".to_string(),
        html_text: None,
        text_palette: Palette::Primary,
        text_style: Style::Regular,
        text_size: Size::Medium,
        interaction_effect: false,
        key: "".to_string(),
        code_ref: NodeRef::default(),
        class_name: "class-card-test".to_string(),
        styles: css!("color: blue;"),
        id: "id-text-test".to_string(),
    };

    start_app_with_props::<Text>(props);

    let plain_text_element = utils::document()
        .get_elements_by_class_name("plain-text")
        .get_with_index(0);

    assert!(plain_text_element.is_some());
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
        plain_text: "hello test".to_string(),
        html_text: None,
        text_palette: Palette::Primary,
        text_style: Style::Regular,
        text_size: Size::Medium,
        interaction_effect: false,
        key: "".to_string(),
        code_ref: NodeRef::default(),
        class_name: "class-card-test".to_string(),
        styles: css!("color: blue;"),
        id: "id-text-test".to_string(),
    };

    start_app_with_props::<Text>(props);

    let paragraph_text_element = utils::document()
        .get_elements_by_class_name("paragraph-text")
        .get_with_index(0);

    assert!(paragraph_text_element.is_some());
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
        plain_text: "hello test".to_string(),
        html_text: None,
        text_palette: Palette::Primary,
        text_style: Style::Regular,
        text_size: Size::Medium,
        interaction_effect: false,
        key: "".to_string(),
        code_ref: NodeRef::default(),
        class_name: "class-card-test".to_string(),
        styles: css!("color: blue;"),
        id: "id-text-test".to_string(),
    };

    start_app_with_props::<Text>(props);

    let alert_text_element = utils::document()
        .get_elements_by_class_name("alert-text")
        .get_with_index(0);

    assert!(alert_text_element.is_some());
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
        plain_text: "hello test".to_string(),
        html_text: None,
        text_palette: Palette::Primary,
        text_style: Style::Regular,
        text_size: Size::Medium,
        interaction_effect: false,
        key: "".to_string(),
        code_ref: NodeRef::default(),
        class_name: "class-card-test".to_string(),
        styles: css!("color: blue;"),
        id: "id-text-test".to_string(),
    };

    start_app_with_props::<Text>(props);

    let tag_text_element = utils::document()
        .get_elements_by_class_name("tag-text")
        .get_with_index(0);

    assert!(tag_text_element.is_some());
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
        plain_text: "hello test".to_string(),
        html_text: None,
        text_palette: Palette::Primary,
        text_style: Style::Regular,
        text_size: Size::Medium,
        interaction_effect: false,
        key: "".to_string(),
        code_ref: NodeRef::default(),
        class_name: "class-card-test".to_string(),
        styles: css!("color: blue;"),
        id: "id-text-test".to_string(),
    };

    start_app::<Text>();

    let tag_text_element = utils::document()
        .get_elements_by_class_name("tag-delete")
        .get_with_index(0);

    assert!(tag_text_element.is_some());
}
