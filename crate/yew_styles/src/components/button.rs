use crate::styles::colors::get_styles;
use crate::styles::helpers::{
    get_palette, get_palette_style, get_size, get_style, Palette, Size, Style,
};
use gloo::utils;
use stylist::{css, StyleSource, YieldStyle};
use wasm_bindgen_test::*;
use web_sys::window;
use yew::prelude::*;
use yew::start_app;

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
///                 let mut console = ConsoleService::log(&format!("{}", menu));
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
    props: ButtonProps,
}

#[derive(PartialEq)]
struct ButtonProps {
    button_palette: String,
    button_size: String,
    button_style: String,
    class_name: String,
    id: String,
    key: String,
    code_ref: NodeRef,
    onclick_signal: Callback<MouseEvent>,
    styles: StyleSource<'static>,
    children: Children,
}

impl YieldStyle for Button {
    fn style_from(&self) -> StyleSource<'static> {
        let styles = get_styles();
        let style = self.props.button_style.clone();
        let color = styles
            .get(style.as_str())
            .unwrap()
            .iter()
            .find(|palette| palette.name == self.props.button_palette.clone())
            .unwrap();

        css!(
            r#"
                padding: 5px 10px;
                border: none;
                border-radius: 4px;
                cursor: pointer;
                font-family: Rosario;
                font-size: 18px;

                ${palette}

                &.small {
                    font-size: 12px;
                }

                &.big {
                    font-size: 26px;
                }
            "#,
            palette = get_palette_style(color, true)
        )
    }
}

impl From<Props> for ButtonProps {
    fn from(props: Props) -> Self {
        ButtonProps {
            button_palette: get_palette(props.button_palette),
            button_size: get_size(props.button_size),
            button_style: get_style(props.button_style),
            class_name: props.class_name,
            id: props.id,
            key: props.key,
            code_ref: props.code_ref,
            onclick_signal: props.onclick_signal,
            styles: props.styles,
            children: props.children,
        }
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    /// Type botton style. Default `Palette::Standard`
    #[prop_or(Palette::Standard)]
    pub button_palette: Palette,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    /// General property to get the ref of the component
    #[prop_or_default]
    pub code_ref: NodeRef,
    /// General property to add keys
    #[prop_or_default]
    pub key: String,
    /// Three diffent button standard sizes. Default `Size::Medium`
    #[prop_or(Size::Medium)]
    pub button_size: Size,
    /// Button styles. Default `Style::Regular`
    #[prop_or(Style::Regular)]
    pub button_style: Style,
    /// Click event for button. Required
    pub onclick_signal: Callback<MouseEvent>,
    /// Set css styles directly in the component
    #[prop_or(css!(""))]
    pub styles: StyleSource<'static>,
    pub children: Children,
}

pub enum Msg {
    Clicked(MouseEvent),
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ButtonProps::from(*ctx.props()),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(mouse_event) => {
                ctx.props().onclick_signal.emit(mouse_event);
            }
        };

        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props = ButtonProps::from(*ctx.props());
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            button_palette,
            class_name,
            id,
            code_ref,
            key,
            button_size,
            button_style,
            onclick_signal,
            styles,
            children,
        } = &ctx.props();

        html! {
            <button
                onclick={ctx.link().callback(Msg::Clicked)}
                class={classes!("button",
                    self.props.button_palette.clone(),
                    self.props.button_size.clone(),
                    self.props.button_style.clone(),
                    class_name.clone(),
                    styles.clone(),
                )}
                key={key.clone()}
                ref={code_ref.clone()}
                id={id.clone()}
            > { children.clone() }
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
        key: "".to_string(),
        code_ref: NodeRef::default(),
        button_size: Size::Medium,
        button_style: Style::Regular,
        onclick_signal: onchange_name,
        button_palette: Palette::Standard,
        styles: css!("background-color: #918d94;"),
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
    impl Default for Props {
        fn default() -> Props {
            Props {
                class_name: String::from("test-button"),
                id: String::from("button-id-test"),
                key: "".to_string(),
                code_ref: NodeRef::default(),
                button_size: Size::Medium,
                button_style: Style::Regular,
                onclick_signal: Callback::noop(),
                button_palette: Palette::Standard,
                styles: css!("background-color: #918d94;"),
                children: Children::new(vec![html! {<div id="result">{"result"}</div>}]),
            }
        }
    }

    start_app::<Button>();

    let button_element = utils::document()
        .get_elements_by_tag_name("button")
        .get_with_index(0)
        .unwrap();

    let child = button_element.first_element_child().unwrap();

    assert_eq!(button_element.tag_name(), "BUTTON");
    assert_eq!(child.id(), "result");
}
