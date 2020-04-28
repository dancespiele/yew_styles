use wasm_bindgen_test::*;
use web_sys::window;
use yew::prelude::*;

pub enum Msg {
    Clicked,
}

/// # Navbar Item component
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
///                 let mut console = ConsoleService::new();
///                 console.log(format!("{}", menu))
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
///                            onsignal=link.callback(move |_| Msg::ChangeMenu(String::from("Home")))>
///                            <span>{"Home"}</span>
///                        </NavbarItem>
///                        <NavbarItem
///                            onsignal=link.callback(move |_| Msg::ChangeMenu(String::from("Shop")))>
///                            <span>{"Shop"}</span>
///                        </NavbarItem>
///                        <NavbarItem
///                            onsignal=link.callback(move |_| Msg::ChangeMenu(String::from("Shop")))>
///                            <span>{"Shop"}</span>
///                        </NavbarItem>
///                        <NavbarItem
///                            onsignal=link.callback(move |_| Msg::ChangeMenu(String::from("About us")))>   
///                            <span>{"About us"}</span>
///                        </NavbarItem>
///                        <NavbarItem
///                            onsignal=link.callback(move |_| Msg::ChangeMenu(String::from("Contact")))>   
///                            <span>{"Contact"}</span>
///                        </NavbarItem>
///                    </NavbarContainer>
///              </Navbar>
///         }
///     }
/// }
/// ```
pub struct NavbarItem {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// click event for navbar item
    #[prop_or(Callback::noop())]
    pub onsignal: Callback<()>,
    pub children: Children,
}

impl Component for NavbarItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NavbarItem { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.props.onsignal.emit(());
            }
        }

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;

        false
    }

    fn view(&self) -> Html {
        html! {
            <div
                class=format!("navbar-item {}", self.props.class_name)
                onclick=self.link.callback(|_| Msg::Clicked)
            >
                {self.props.children.render()}
            </div>
        }
    }
}

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn should_create_navbar_item() {
    let navbar_item_props = Props {
        class_name: "navbar-item-test".to_string(),
        onsignal: Callback::noop(),
        children: Children::new(vec![html! {
            <div id="item">{"Item"}</div>
        }]),
    };

    let link = ComponentLink::new();
    let navbar_item = NavbarItem::create(navbar_item_props, link);
    let navbar_item_vnode = navbar_item.render();

    let vnode_expected = html! {
        <div
            onclick=Callback::noop()
            class="navbar-item navbar-item-test">
            <>
                <div id="item">
                    {"Item"}
                </div>
            </>
        </div>
    };

    assert_eq!(navbar_item_vnode, vnode_expected);
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
        class_name: "navbar-item-test".to_string(),
        onsignal: on_add_item_div,
        children: Children::new(vec![html! {
            <div id="item">{"Item"}</div>
        }]),
    };

    navbar_item_props.onsignal.emit(());

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
