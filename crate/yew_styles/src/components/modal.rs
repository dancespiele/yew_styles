use crate::styles::colors::{get_styles, transparentize};
use crate::styles::helpers::{
    get_palette, get_palette_style, get_size, get_style, Palette, Size, Style,
};
use crate::utils::get_html_element_by_class;
use gloo::utils;
use stylist::{css, Style as StyleList, StyleSource, YieldStyle};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::Element;
use yew::prelude::*;
use yew::{html::Scope, start_app_with_props};

/// # Modal component
///
/// ## Features required
///
/// modal
///
/// ## Example
///
/// ```rust
/// use wasm_bindgen::JsCast;
/// use web_sys::HtmlElement;
/// use yew::prelude::*;
/// use yew::utils::document;
/// use yew_prism::Prism;
/// use yew_styles::button::Button;
/// use yew_styles::modal::Modal;
/// use yew_styles::styles::helpers::{get_size, Palette, Size, Style};
///
/// pub struct ModalExample {
///     link: ComponentLink<Self>,
///     show_modal: bool,
/// }
///
/// pub enum Msg {
///     CloseModal,
///     OpenModal,
///     CloseModalByKb(KeyboardEvent),
/// }
///
/// impl Component for ModalExample {
///     type Message = Msg;
///     type Properties = ();
///
///     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
///         Self {
///             link,
///             show_modal: false,
///         }
///     }
///
///     fn update(&mut self, msg: Self::Message) -> ShouldRender {
///         let body_style = document()
///             .body()
///             .unwrap()
///             .dyn_into::<HtmlElement>()
///             .unwrap()
///             .style();
///
///         match msg {
///             Msg::CloseModal(index) => {
///                 body_style.set_property("overflow", "auto").unwrap();
///                 self.show_modal = false;
///             }
///             Msg::CloseModalByKb(keyboard_event) => {
///                 if keyboard_event.key_code() == 27 {
///                     body_style.set_property("overflow", "auto").unwrap();
///                     self.show_modal = false;
///                 }
///             }
///             Msg::OpenModal => {
///                 body_style.set_property("overflow", "hidden").unwrap();
///
///                 self.show_modal = true;
///             }
///         };
///         true
///     }
///
///     fn change(&mut self, _: Self::Properties) -> ShouldRender {
///         false
///     }
///
///     fn view(&self) -> Html {
///         html! {
///             <>
///                 <Modal
///                     header=html!{
///                         <b>{"Standard modal"}</b>
///                     }
///                     header_palette=Palette::Link
///                     body=html!{
///                         <div class="body-content">
///                             <p>{"This is a example modal"}</p>
///                             <Button
///                                 button_palette= Palette::Info
///                                 onclick_signal= self.link.callback(|_| Msg::CloseModal)
///                             >{"Accept"}</Button>
///                         </div>
///                     }
///                     body_style=Style::Outline
///                     body_palette=Palette::Link
///                     is_open=self.show_modal
///                     onclick_signal= self.link.callback(|_| Msg::CloseModal)
///                     onkeydown_signal= self.link.callback(Msg::CloseModalByKb)
///                 />
///                 <Button
///                     button_palette= Palette::Primary
///                     onclick_signal= self.link.callback(Msg::OpenModal)
///                 >{"Standard modal"}</Button>
///             </>
///         }
///     }
/// }
/// ```    
pub struct Modal {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// Header of the modal. Required
    pub header: Html,
    /// body of the modal. Required
    pub body: Html,
    /// if it is true, shows the modal otherwise is hidden. Required
    pub is_open: bool,
    /// click event for modal (usually to close the modal)
    #[prop_or(Callback::noop())]
    pub onclick_signal: Callback<MouseEvent>,
    /// keyboard event for modal (usually to close the modal)
    #[prop_or(Callback::noop())]
    pub onkeydown_signal: Callback<KeyboardEvent>,
    /// Type modal background style. Default `Palette::Standard`
    #[prop_or(Palette::Standard)]
    pub modal_palette: Palette,
    /// Three diffent modal standard sizes. Default `Size::Medium`
    #[prop_or(Size::Medium)]
    pub modal_size: Size,
    /// Type modal header style. Default `Palette::Standard`
    #[prop_or(Palette::Standard)]
    pub header_palette: Palette,
    /// Modal header styles. Default `Style::Regular`
    #[prop_or(Style::Regular)]
    pub header_style: Style,
    /// If hove, focus, active effects are enable in the header. Default `false`
    #[prop_or(false)]
    pub header_interaction: bool,
    /// Type modal body style. Default `Palette::Standard`
    #[prop_or(Palette::Standard)]
    pub body_palette: Palette,
    /// Modal body styles. Default `Style::Regular`
    #[prop_or(Style::Regular)]
    pub body_style: Style,
    /// If hove, focus, active effects are enable in the body. Default `false`
    #[prop_or(false)]
    pub body_interaction: bool,
    /// If the modal content get the focus. Set to false if the modal includes input events. Default `true`
    #[prop_or(true)]
    pub auto_focus: bool,
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
    Clicked(MouseEvent),
    Pressed(KeyboardEvent),
}

impl YieldStyle for Modal {
    fn style_from(&self) -> StyleSource<'static> {
        let styles = get_styles();
        let header_style = self.props.header_style.clone();
        let body_style = self.props.body_style.clone();

        let header_color = styles
            .get(get_style(header_style).as_str())
            .unwrap()
            .iter()
            .find(|palette| palette.name == get_palette(self.props.header_palette.clone()))
            .unwrap();

        let body_color = styles
            .get(get_style(body_style).as_str())
            .unwrap()
            .iter()
            .find(|palette| palette.name == get_palette(self.props.body_palette.clone()))
            .unwrap();

        format!(
            r#"
                position: fixed;
                z-index: 1;
                top: 0;
                left: 0;
                width: 100%;
                height: 100%;
                display: flex;
                justify-content: center;
                overflow: hidden;
                background-color: {};

                .modal-content {{
                    width: 50%;
                    top: 20vh;
                    position: fixed;
                    z-index: 2;
                }}

                .modal-content .small {{
                    width: 30%;
                }}

                .modal-content .big {{
                    width: 75%;
                }}

                .modal-header {{
                    padding: 3px 15px;
                    border-radius: 8px 8px 0 0;

                    {}
                }}

                .modal-body {{
                    padding: 10px 15px;
                    border-radius: 0 0 5px 5px;

                    {}
                }}
            "#,
            transparentize("regular", -0.4),
            get_palette_style(header_color, true),
            get_palette_style(body_color, true),
        )
    }
}

impl Component for Modal {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: *ctx.props(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(mouse_event) => {
                let target_event = mouse_event
                    .target()
                    .unwrap()
                    .dyn_into::<Element>()
                    .unwrap()
                    .class_list();

                if target_event.value().starts_with("modal container") {
                    ctx.props().onclick_signal.emit(mouse_event);
                }
            }
            Msg::Pressed(keyboard_event) => {
                ctx.props().onkeydown_signal.emit(keyboard_event);
            }
        };
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props = *ctx.props();
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if self.props.is_open && self.props.auto_focus {
            let modal_form = get_html_element_by_class("modal", 0);

            modal_form.focus().unwrap();
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        get_modal(self.style(), *ctx.props(), ctx.link())
    }
}

fn get_modal(style: StyleList, props: Props, link: &Scope<Modal>) -> Html {
    if props.is_open {
        html! {
            <div
                class={classes!(style, "container", get_palette(props.modal_palette), props.class_name, props.styles)}
                key={props.key}
                ref={props.code_ref}
                tabindex={"0"}
                id={props.id}
                onclick={link.callback(Msg::Clicked)}
                onkeydown={link.callback(Msg::Pressed)}
            >
                <div class={format!("modal-content {}", get_size(props.modal_size))}>
                    <div class={classes!(
                        "modal-header",
                        if props.header_interaction { "interaction" } else { "" }
                    )}>
                        {props.header}
                    </div>
                    <div class={classes!(
                        "modal-body",
                        if props.body_interaction { "interaction" } else { "" }
                    )}>
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
        key: "".to_string(),
        code_ref: NodeRef::default(),
        onclick_signal: Callback::noop(),
        onkeydown_signal: Callback::noop(),
        modal_palette: Palette::Standard,
        modal_size: Size::Medium,
        header: html! {<div id="header">{"Modal Test"}</div>},
        header_style: Style::Regular,
        header_palette: Palette::Standard,
        header_interaction: false,
        body: html! {<div id="body">{"Content Test"}</div>},
        body_style: Style::Regular,
        body_palette: Palette::Standard,
        body_interaction: false,
        is_open: true,
        auto_focus: false,
        styles: css!(
            "modal-content {
                    color: #000;
                }"
        ),
    };

    start_app_with_props::<Modal>(props);

    let modal_header_element = utils::document().get_element_by_id("header").unwrap();

    let modal_body_element = utils::document().get_element_by_id("body").unwrap();

    assert_eq!(modal_header_element.text_content().unwrap(), "Modal Test");
    assert_eq!(modal_body_element.text_content().unwrap(), "Content Test");
}
/*
#[wasm_bindgen_test]
fn should_hide_modal_component_from_doom() {
    impl Default for Props {
        fn default() -> Self {
            Self {
                class_name: "test-modal".to_string(),
                id: "modal-id-test".to_string(),
                key: "".to_string(),
                code_ref: NodeRef::default(),
                onclick_signal: Callback::noop(),
                onkeydown_signal: Callback::noop(),
                modal_palette: Palette::Standard,
                modal_size: Size::Medium,
                header: html! {<div id="header">{"Modal Test"}</div>},
                header_style: Style::Regular,
                header_palette: Palette::Standard,
                header_interaction: false,
                body: html! {<div id="body">{"Content Test"}</div>},
                body_style: Style::Regular,
                body_palette: Palette::Standard,
                body_interaction: false,
                is_open: false,
                auto_focus: false,
                styles: css!(
                    "modal-content {
                        color: #000;
                    }"
                ),
            }
        }
    }

    start_app_with_props::<Modal>(props);

    let modal_element = utils::document().get_element_by_id("modal-id-test");

    assert_eq!(modal_element, None);
}*/
