use crate::utils::{get_html_element_by_class, get_random_string};
use gloo::utils;
use stylist::{css, StyleSource};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::start_app_with_props;

/// # Navbar Dropdown Container component
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
///                 ConsoleService::log(format!("{}", menu));
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
///                        <NavbarDropdown main_content=html!{
///                           <span>{menu}<ControllerAssets
///                             icon=ControllerIcon::ChevronDown
///                             size=("20".to_string(), "20".to_string())
///                           /></span>
///                        }>
///                          <NavbarDropdownItem
///                            onclick_signal=link.callback(move |_: MouseEvent| Msg::ChangeType(String::from("menu 1".to_string())))>{"menu 1"}</NavbarDropdownItem>
///                          <NavbarDropdownItem
///                            onclick_signal=link.callback(move |_: MouseEvent| Msg::ChangeType(String::from("menu 2".to_string())))>{"menu 2"}</NavbarDropdownItem>
///                          <NavbarDropdownItem
///                            onclick_signal=link.callback(move |_: MouseEvent| Msg::ChangeType(String::from("menu 3".to_string())))>{"menu 3"}</NavbarDropdownItem>
///                        </NavbarDropdown>
///                    </NavbarContainer>
///              </Navbar>
///         }
///     }
/// }
/// ```
pub struct NavbarDropdown {
    show: bool,
    key: String,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    /// clickeable content to show the dropdown. Required
    pub main_content: Html,
    /// General property to add keys
    #[prop_or_default]
    pub key: String,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// show with style when the dropdown is currrently active
    #[prop_or(false)]
    pub active: bool,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    /// Set css styles directly in the component
    #[prop_or(css!(""))]
    pub styles: StyleSource<'static>,
    pub children: Children,
}

pub enum Msg {
    ShowDropdown,
    HideDropdown,
}

impl Component for NavbarDropdown {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        let key = get_random_string(10);

        Self { show: false, key }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowDropdown => {
                self.show = true;
            }
            Msg::HideDropdown => {
                self.show = false;
            }
        }
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let navbar_dropdown =
                get_html_element_by_class(&format!("navbar-dropdown-{}", self.key), 0);
            let navbar_dropdown_width = navbar_dropdown.offset_width();

            let navbar_dropdown_container =
                get_html_element_by_class(&format!("navbar-dropdown-container-{}", self.key), 0);

            navbar_dropdown_container
                .style()
                .set_property("width", &format!("{}px", navbar_dropdown_width))
                .unwrap()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            main_content,
            key,
            class_name,
            active,
            id,
            styles,
            children,
        } = &ctx.props();

        html! {
            <div
                class={classes!("navbar-dropdown", format!("navbar-dropdown-{}", self.key), if *active {
                    "active"
                } else {
                    ""
                }, class_name.clone(), styles.clone())}
                id={id.clone()}
                key={key.clone()}
                onmouseover={ctx.link().callback(|_| Msg::ShowDropdown)}
                onmouseleave={ctx.link().callback(|_| Msg::HideDropdown)}
                onclick={ctx.link().callback(|_| Msg::HideDropdown)}
                >
                <div class="main-content">{main_content.clone()}</div>
                {get_items(self.show, self.key.clone(), children.clone())}
            </div>
        }
    }
}

fn get_items(show: bool, key: String, children: Children) -> Html {
    html! {
        <ul class={classes!(format!("navbar-dropdown-container-{}", key), if show { "active"} else {"inactive"})}>
            {children.clone()}
        </ul>
    }
}

#[wasm_bindgen_test]
fn should_create_navbar_dropdown_container() {
    let props = Props {
        main_content: html! {<div id="test">{"test"}</div>},
        active: false,
        key: String::from("navbar-dropdown-1"),
        class_name: String::from("class-test"),
        id: String::from("id-test"),
        styles: css!("background-color: #918d94;"),
        children: Children::new(vec![html! {
            <div id="item">{"Item"}</div>
        }]),
    };

    start_app_with_props::<NavbarDropdown>(props);

    let content_element = utils::document().get_element_by_id("test").unwrap();
    assert_eq!(content_element.text_content().unwrap(), "test".to_string());
}
