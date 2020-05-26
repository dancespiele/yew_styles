use super::navbar_container::NavbarContainer;
use super::navbar_item::NavbarItem;
use crate::assets::{Assets, Icon};
use crate::layouts::container::{Direction, JustifyContent, Mode};
use crate::styles::{get_pallete, get_style, Palette, Style};
use crate::utils::create_style;
use yew::prelude::*;

/// the location of the navbar which is fixed
#[derive(Clone, PartialEq)]
pub enum Fixed {
    None,
    Top,
    Bottom,
}

pub enum Msg {
    TroggleMenu,
}

/// # Navbar component
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
pub struct Navbar {
    pub link: ComponentLink<Self>,
    pub props: NavbarProps,
    pub display_menu: bool,
}

struct NavbarModel;

#[derive(Clone, Properties)]
pub struct Props {
    /// Type navbar style
    #[prop_or(Palette::Standard)]
    pub navbar_type: Palette,
    /// Navbar styles
    #[prop_or(Style::Regular)]
    pub navbar_style: Style,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    /// The location of the navbar which is fixed
    #[prop_or(Fixed::Top)]
    pub fixed: Fixed,
    /// Vnode embedded in the beginning of the navbar, useful to include a branch logo
    #[prop_or_default]
    pub branch: Html,
    pub children: Children,
}

#[derive(Clone)]
pub struct NavbarProps {
    pub navbar_type: String,
    pub navbar_style: String,
    pub class_name: String,
    pub fixed: Fixed,
    pub branch: Html,
    pub children: Children,
}

impl From<Props> for NavbarProps {
    fn from(props: Props) -> Self {
        NavbarProps {
            navbar_type: get_pallete(props.navbar_type),
            navbar_style: get_style(props.navbar_style),
            class_name: props.class_name,
            fixed: props.fixed,
            branch: props.branch,
            children: props.children,
        }
    }
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Navbar {
            link,
            props: NavbarProps::from(props),
            display_menu: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TroggleMenu => {
                self.display_menu = !self.display_menu;
            }
        };

        true
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            NavbarModel.init(self.props.clone());
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        NavbarModel.init(self.props.clone());
        self.props = NavbarProps::from(props);
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div
                    class=format!("navbar-mobile {} {} {}", self.props.navbar_style, self.props.navbar_type, self.props.class_name)
                >
                    <div class="navbar-dropdown">
                        <NavbarContainer justify_content=JustifyContent::FlexStart(Mode::NoMode)
                        direction=Direction::Row
                        class_name="navbar-container-mobile">
                        {get_branch(self.props.branch.clone())}
                        </NavbarContainer>
                        <NavbarContainer justify_content=JustifyContent::FlexEnd(Mode::NoMode)
                            direction=Direction::Row
                            class_name="navbar-container-mobile">
                            <NavbarItem
                                onsignal=self.link.callback(move |_| Msg::TroggleMenu)
                            >
                             <Assets
                                asset=Icon::Menu
                                class_name="navbar-menu"
                             />
                            </NavbarItem>
                        </NavbarContainer>
                    </div>
                    {if self.display_menu {
                        self.props.children.render()
                    } else {
                        html!{}
                    }}
                </div>

                <div
                    class=format!("navbar {} {} {}", self.props.navbar_style, self.props.navbar_type, self.props.class_name)
                >
                <NavbarContainer justify_content=JustifyContent::Start(Mode::NoMode)
                    direction=Direction::Row
                    class_name="navbar-container-mobile">
                    {get_branch(self.props.branch.clone())}
                </NavbarContainer>
                    {self.props.children.render()}
                </div>
            </>
        }
    }
}

impl NavbarModel {
    fn init(self, props: NavbarProps) {
        self.set_fixed(props.fixed);
    }

    fn set_fixed(self, fixed: Fixed) {
        create_style(
            String::from("position"),
            if fixed == Fixed::None {
                String::from("inherit")
            } else {
                String::from("fixed")
            },
            String::from("navbar"),
        );

        if fixed != Fixed::None {
            create_style(
                if fixed == Fixed::Top {
                    String::from("top")
                } else {
                    String::from("bottom")
                },
                String::from("0"),
                String::from("navbar"),
            );
        }
    }
}

fn get_branch(branch: Html) -> Html {
    if branch != html! {} {
        html! {
            <div class="branch">
                {branch}
            </div>
        }
    } else {
        html! {}
    }
}
