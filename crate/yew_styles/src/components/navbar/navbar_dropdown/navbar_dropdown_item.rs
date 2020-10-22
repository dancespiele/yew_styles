use yew::prelude::*;

/// # Navbar Dropdown Item component
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
pub struct NavbarDropdownItem {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or(Callback::noop())]
    /// Click event for dropdown item
    pub onclick_signal: Callback<MouseEvent>,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// show with style when the dropdown item is currrently active
    #[prop_or(false)]
    pub active: bool,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    pub children: Children,
}

pub enum Msg {
    Clicked(MouseEvent),
}

impl Component for NavbarDropdownItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked(mouse_event) => {
                self.props.onclick_signal.emit(mouse_event);
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            return true;
        }

        false
    }

    fn view(&self) -> Html {
        html! {
            <li
                class=("navbar-dropdown-item",if self.props.active {
                    "active"
                } else {
                    ""
                }, self.props.class_name.clone())
                id=self.props.id
                onclick=self.link.callback(Msg::Clicked)
            >{self.props.children.clone()}</li>
        }
    }
}
