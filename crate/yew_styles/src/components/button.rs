use crate::styles::{get_pallete, get_style, Palette, Style};
use wasm_bindgen_test::*;
use web_sys::window;
use yew::prelude::*;

/// The standard sizes for button
#[derive(Clone)]
pub enum Size {
    Small,
    Medium,
    Big,
}

/// # Button component
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew::services::ConsoleService;
/// use yew_styles::{
///     button::{Button, Size},
///     styles::{Palette, Style},
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
///                 console.log(format!("{}", greeting))
///             }
///         }
///         false
///     }
///
///     fn view(&self) -> Html {
///        html! {
///          <Button
///             onsignal=link.callback(move |_| Msg::Clicked("Hello world"))
///             class_name="hello-world"
///             button_type=Pallete::Standard
///             button_style=Style::Light
///             size=Size::Medium
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
    button_type: String,
    size: String,
    button_style: String,
    class_name: String,
    onsignal: Callback<()>,
    children: Children,
}

impl From<Props> for ButtonProps {
    fn from(props: Props) -> Self {
        ButtonProps {
            button_type: get_pallete(props.button_type),
            size: get_size(props.size),
            button_style: get_style(props.button_style),
            class_name: props.class_name,
            onsignal: props.onsignal,
            children: props.children,
        }
    }
}

#[derive(Clone, Properties)]
pub struct Props {
    /// Type botton style. Options included in `Pallete`
    #[prop_or(Palette::Standard)]
    pub button_type: Palette,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// Three diffent button standard sizes. Options included in `Size`
    #[prop_or(Size::Medium)]
    pub size: Size,
    /// Button styles. Options included in `Style`
    #[prop_or(Style::Regular)]
    pub button_style: Style,
    /// Click event for button
    pub onsignal: Callback<()>,
    pub children: Children,
}

pub enum Msg {
    Clicked,
}

pub fn get_size(size: Size) -> String {
    match size {
        Size::Small => String::from("small"),
        Size::Medium => String::from("medium"),
        Size::Big => String::from("big"),
    }
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Button {
            link,
            props: ButtonProps::from(props),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.props.onsignal.emit(());
            }
        };

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = ButtonProps::from(props);
        true
    }

    fn view(&self) -> Html {
        html! {
            <button
                onclick=self.link.callback(|_| Msg::Clicked)
                class=format!("button {} {} {} {}",
                    self.props.button_type.clone(),
                    self.props.size.clone(),
                    self.props.button_style.clone(),
                    self.props.class_name.clone())
            > { self.props.children.render() }
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
        size: Size::Medium,
        button_style: Style::Regular,
        onsignal: onchange_name,
        button_type: Palette::Standard,
        children: Children::new(vec![html! {<div id="submenu">{"another menu"}</div>}]),
    };

    let link = ComponentLink::new();

    let mut button = Button::create(props.clone(), link);

    props.onsignal.emit(());

    button.change(props);

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
    let on_add_child = Callback::from(|_| {
        let body = window().unwrap().document().unwrap().body().unwrap();

        let child_element = window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();

        child_element.set_text_content(Some("child"));
        child_element.set_id("child");
        body.append_child(&child_element).unwrap();
    });

    let props = Props {
        class_name: String::from("test-button"),
        size: Size::Medium,
        button_style: Style::Regular,
        onsignal: on_add_child,
        button_type: Palette::Standard,
        children: Children::new(vec![html! {<div id="parent">{"parent"}</div>}]),
    };

    let link = ComponentLink::new();

    let mut button = Button::create(props.clone(), link.clone());

    props.onsignal.emit(());

    button.change(props);

    let button_vnode = button.render();

    let vnode_expected = html! {
        <button
            onclick=link.callback(|_| Msg::Clicked)
            class="button standard medium regular test-button">
            <>
                <div id="parent">{"parent"}</div>
            </>
        </button>
    };

    assert_eq!(button_vnode, vnode_expected);
}
