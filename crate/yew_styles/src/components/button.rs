use crate::styles::{get_pallete, get_size, get_style, Palette, Size, Style};
use wasm_bindgen_test::*;
use web_sys::window;
use yew::prelude::*;
use yew::{utils, App};

/// # Button component
///
/// ## Features required
///
/// button
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew::services::ConsoleService;
/// use yew_styles::{
///     button::{Button},
///     styles::{Palette, Style, Size},
/// };
///
/// pub struct App {
///   link: ComponentLink<Self>,
/// }
///
/// pub enum Msg {
///   Clicked(String),
/// }
/// #[derive(Clone, Properties)]
/// pub struct Props {}
///
/// impl Component for App {
///     type Message = Msg;
///     type Properties = Props;
///
///     fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
///         App {
///             link
///         }
///     }
///
///     fn update(&mut self, msg: Self::Message) -> ShouldRender {
///         match msg {
///             Msg::Clicked(greeting) => {
///                 let mut console = ConsoleService::new();
///                 console.log(&format!("{}", greeting))
///             }
///         }
///         false
///     }
///
///     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
///         false
///     }
///
///     fn view(&self) -> Html {
///        html! {
///          <Button
///             onclick_signal=link.callback(move |_| Msg::Clicked(String::from("Hello world")))
///             class_name="hello-world"
///             button_palette=Pallete::Standard
///             button_style=Style::Light
///             button_size=Size::Medium
///          >{"Greeting"}</Button>
///        }
///     }
/// }
/// ```
pub struct Button {
    link: ComponentLink<Self>,
    props: ButtonProps,
}

struct ButtonProps {
    button_palette: String,
    button_size: String,
    button_style: String,
    class_name: String,
    onclick_signal: Callback<MouseEvent>,
    children: Children,
}

impl From<Props> for ButtonProps {
    fn from(props: Props) -> Self {
        ButtonProps {
            button_palette: get_pallete(props.button_palette),
            button_size: get_size(props.button_size),
            button_style: get_style(props.button_style),
            class_name: props.class_name,
            onclick_signal: props.onclick_signal,
            children: props.children,
        }
    }
}

#[derive(Clone, Properties)]
pub struct Props {
    /// Type botton style. Options included in `Pallete`
    #[prop_or(Palette::Standard)]
    pub button_palette: Palette,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    /// Three diffent button standard sizes. Options included in `Size`
    #[prop_or(Size::Medium)]
    pub button_size: Size,
    /// Button styles. Options included in `Style`
    #[prop_or(Style::Regular)]
    pub button_style: Style,
    /// Click event for button. Required
    pub onclick_signal: Callback<MouseEvent>,
    pub children: Children,
}

pub enum Msg {
    Clicked(MouseEvent),
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props: ButtonProps::from(props),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked(mouse_event) => {
                self.props.onclick_signal.emit(mouse_event);
            }
        };

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = ButtonProps::from(props);
        true
    }

    fn view(&self) -> Html {
        html! {
            <button
                onclick=self.link.callback(Msg::Clicked)
                class=format!("button {} {} {} {}",
                    self.props.button_palette.clone(),
                    self.props.button_size.clone(),
                    self.props.button_style.clone(),
                    self.props.class_name.clone())
            > { self.props.children.clone() }
            </button>
        }
    }
}

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
        id: String::from("button-id-test"),
        button_size: Size::Medium,
        button_style: Style::Regular,
        onclick_signal: onchange_name,
        button_palette: Palette::Standard,
        children: Children::new(vec![html! {<div id="submenu">{"another menu"}</div>}]),
    };

    let mouse_event = MouseEvent::new("click").unwrap();

    props.onclick_signal.emit(mouse_event);

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
    let props = Props {
        class_name: String::from("test-button"),
        id: String::from("button-id-test"),
        button_size: Size::Medium,
        button_style: Style::Regular,
        onclick_signal: Callback::noop(),
        button_palette: Palette::Standard,
        children: Children::new(vec![html! {<div id="result">{"result"}</div>}]),
    };

    let button: App<Button> = App::new();
    button.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let button_element = utils::document()
        .get_elements_by_tag_name("button")
        .get_with_index(0)
        .unwrap();

    let child = button_element.first_element_child().unwrap();

    assert_eq!(button_element.tag_name(), "BUTTON");
    assert_eq!(child.id(), "result");
}
