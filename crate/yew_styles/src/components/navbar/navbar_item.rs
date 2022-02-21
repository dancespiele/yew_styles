use gloo::utils;
use stylist::{css, StyleSource};
use wasm_bindgen_test::*;
use web_sys::window;
use yew::prelude::*;
use yew::start_app_with_props;

pub enum Msg {
    Clicked(MouseEvent),
}

/// # Navbar Item component
///
/// ## Features required
///
/// navbar
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew::services::ConsoleService;
/// use yew_styles::{
///     navbar::{
///         navbar_component::{Fixed, Navbar},
///         navbar_container::NavbarContainer,
///         navbar_item::NavbarItem,
///     },
///     styles::{Palette, Style},
///     layouts::{
///         container::{JustifyContent, Mode},
///     },
/// };
///
/// pub struct App {
///   link: ComponentLink<Self>,
/// }
///
/// pub enum Msg {
///   ChangeMenu(String),
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
///             Msg::ChangeMenu(menu) => {
///                 let mut console = ConsoleService::log(format!("{}", menu));
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
///            <Navbar
///                fixed=Fixed::None
///                navbar_style=Style::Light
///                navbar_type=Palette::Info
///                branch=html!{<img src="/assets/spielrs_logo.png"></img>}>
///                    <NavbarContainer justify_content=JustifyContent::FlexStart(Mode::NoMode)>
///                        <NavbarItem
///                            onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from("Home")))>
///                            <span>{"Home"}</span>
///                        </NavbarItem>
///                        <NavbarItem
///                            onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from("Shop")))>
///                            <span>{"Shop"}</span>
///                        </NavbarItem>
///                        <NavbarItem
///                            onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from("Shop")))>
///                            <span>{"Shop"}</span>
///                        </NavbarItem>
///                        <NavbarItem
///                            onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from("About us")))>   
///                            <span>{"About us"}</span>
///                        </NavbarItem>
///                        <NavbarItem
///                            onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from("Contact")))>   
///                            <span>{"Contact"}</span>
///                        </NavbarItem>
///                    </NavbarContainer>
///              </Navbar>
///         }
///     }
/// }
/// ```
pub struct NavbarItem;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
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
    /// Active nav item style. Default false
    /// if hove, focus, active effects are enable. Default `true`
    #[prop_or(true)]
    pub interaction_effect: bool,
    #[prop_or(false)]
    pub active: bool,
    /// Click event for navbar item
    #[prop_or(Callback::noop())]
    pub onclick_signal: Callback<MouseEvent>,
    /// Set css styles directly in the component
    #[prop_or(css!(""))]
    pub styles: StyleSource<'static>,
    pub children: Children,
}

impl Component for NavbarItem {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(mouse_event) => {
                ctx.props().onclick_signal.emit(mouse_event);
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            code_ref,
            key,
            class_name,
            id,
            interaction_effect,
            active,
            onclick_signal,
            styles,
            children,
        } = &ctx.props();

        html! {
            <div
                class={
                    classes!("navbar-item", if *active {
                    "active"
                    } else {
                        ""
                    },
                    if *interaction_effect {
                        "interaction"
                    } else {
                        ""
                    },
                    class_name.clone(),
                    styles.clone()
                    )
                }
                id={id.clone()}
                key={key.clone()}
                ref={code_ref.clone()}
                onclick={ctx.link().callback(Msg::Clicked)}
            >
                {children.clone()}
            </div>
        }
    }
}

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn should_create_navbar_item() {
    let props = Props {
        key: "".to_string(),
        code_ref: NodeRef::default(),
        class_name: "navbar-item-test".to_string(),
        id: "navbar-item-id-test".to_string(),
        onclick_signal: Callback::noop(),
        active: false,
        interaction_effect: true,
        styles: css!("background-color: #918d94;"),
        children: Children::new(vec![html! {
            <div id="item">{"Item"}</div>
        }]),
    };

    start_app_with_props::<NavbarItem>(props);

    let item_element = utils::document().get_element_by_id("item").unwrap();

    assert_eq!(item_element.text_content().unwrap(), "Item".to_string());
}

#[wasm_bindgen_test]
fn should_create_clickable_navbar_item() {
    let on_add_item_div = Callback::from(|_| {
        let body = window().unwrap().document().unwrap().body().unwrap();

        let child_element = window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();

        child_element.set_text_content(Some("item2"));
        child_element.set_id("item2");
        body.append_child(&child_element).unwrap();
    });

    let navbar_item_props = Props {
        key: "".to_string(),
        code_ref: NodeRef::default(),
        class_name: "navbar-item-test".to_string(),
        id: "navbar-item-id-test".to_string(),
        active: false,
        interaction_effect: true,
        onclick_signal: on_add_item_div,
        styles: css!("background-color: #918d94;"),
        children: Children::new(vec![html! {
            <div id="item">{"Item"}</div>
        }]),
    };

    let mouse_event = MouseEvent::new("click").unwrap();

    navbar_item_props.onclick_signal.emit(mouse_event);

    let updated_content = window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("item2")
        .unwrap()
        .text_content()
        .unwrap();

    assert_eq!(updated_content, "item2".to_string());
}
