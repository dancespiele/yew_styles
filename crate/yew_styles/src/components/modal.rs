use crate::styles::{get_pallete, get_size, get_style, Palette, Size, Style};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{Element, HtmlElement};
use yew::prelude::*;
use yew::{utils, App};

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
/// use yew_styles::styles::{get_size, Palette, Size, Style};
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
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    /// Header of the modal
    pub header: Html,
    /// body of the modal
    pub body: Html,
    /// if it is true, shows the modal otherwise is hidden
    pub is_open: bool,
    /// click event for modal (usually to close the modal)
    #[prop_or(Callback::noop())]
    pub onclick_signal: Callback<MouseEvent>,
    /// keyboard event for modal (usually to close the modal)
    #[prop_or(Callback::noop())]
    pub onkeydown_signal: Callback<KeyboardEvent>,
    /// Type modal background style
    #[prop_or(Palette::Standard)]
    pub modal_palette: Palette,
    /// Three diffent modal standard sizes
    #[prop_or(Size::Medium)]
    pub modal_size: Size,
    /// Type modal header style
    #[prop_or(Palette::Standard)]
    pub header_palette: Palette,
    /// Modal header styles
    #[prop_or(Style::Regular)]
    pub header_style: Style,
    /// If hove, focus, active effects are enable in the header
    #[prop_or(false)]
    pub header_interaction: bool,
    /// Type modal body style
    #[prop_or(Palette::Standard)]
    pub body_palette: Palette,
    /// Modal body styles
    #[prop_or(Style::Regular)]
    pub body_style: Style,
    /// If hove, focus, active effects are enable in the body
    #[prop_or(false)]
    pub body_interaction: bool,
    /// If the modal content get the focus. Set to false if the modal includes input events
    #[prop_or(true)]
    pub auto_focus: bool,
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
                let target_event = mouse_event
                    .target()
                    .unwrap()
                    .dyn_into::<Element>()
                    .unwrap()
                    .class_name();

                if target_event.starts_with("modal container") {
                    self.props.onclick_signal.emit(mouse_event);
                }
            }
            Msg::Pressed(keyboard_event) => {
                self.props.onkeydown_signal.emit(keyboard_event);
            }
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn rendered(&mut self, _first_render: bool) {
        if self.props.is_open && self.props.auto_focus {
            utils::document()
                .get_elements_by_class_name("modal")
                .get_with_index(0)
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap()
                .focus()
                .unwrap();
        }
    }

    fn view(&self) -> Html {
        get_modal(self.props.clone(), self.link.clone())
    }
}

fn get_modal(props: Props, link: ComponentLink<Modal>) -> Html {
    if props.is_open {
        html! {
            <div
                class=format!("modal container {} {}", get_pallete(props.modal_palette), props.class_name)
                tabindex="0"
                id=props.id
                onclick=link.callback(Msg::Clicked)
                onkeydown=link.callback(Msg::Pressed)
            >
                <div class=format!("modal-content {}", get_size(props.modal_size))>
                    <div class=format!(
                        "modal-header {} {} {}",
                        get_style(props.header_style),
                        get_pallete(props.header_palette),
                        if props.header_interaction { "interaction" } else { "" }
                    )>
                        {props.header}
                    </div>
                    <div class=format!(
                        "modal-body {} {} {}",
                        get_style(props.body_style),
                        get_pallete(props.body_palette),
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
    };

    let modal: App<Modal> = App::new();

    modal.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let modal_element = utils::document().get_element_by_id("modal-id-test");

    assert_eq!(modal_element, None);
}
