use super::navbar_container::NavbarContainer;
use super::navbar_item::NavbarItem;
use crate::layouts::container::{Direction, JustifyContent, Mode};
use crate::styles::helpers::{get_palette, get_style, Palette, Style};
use crate::utils::create_style;
use stylist::{css, StyleSource};
use yew::prelude::*;
use yew::Children;
use yew_assets::ux_assets::{UxAssets, UxIcon};

/// # Navbar component
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
///   close_navbar_mobile: bool,
/// }
///
/// pub enum Msg {
///   ChangeMenu(String),
///   CloseNavarMobile(MouseEvent),
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
///                 ConsoleService::log(&format!("{}", menu));
///             }
///             Msg::CloseNavarMobile(mouse_event) => {
///                 let target_class = mouse_event
///                     .target()
///                     .unwrap()
///                     .dyn_into::<Element>()
///                     .unwrap()
///                     .class_list();
///
///                 let target_parent = mouse_event
///                     .target()
///                     .unwrap()
///                     .dyn_into::<Element>()
///                     .unwrap()
///                     .parent_element()
///                     .unwrap()
///                     .tag_name();
///
///                 if !target_class.value().contains("navbar-menu") && target_parent != "svg" {
///                     self.close_navbar_mobile = true;
///                 } else {
///                     self.close_navbar_mobile = false
///                 }
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
///            <div onclick=self.link.callback(|e| Msg::CloseNavarMobile(e))>
///                 <Navbar
///                     fixed=Fixed::None
///                     navbar_style=Style::Light
///                     navbar_palette=Palette::Info
///                     hide_navbar_items_mobile = self.close_navbar_mobile
///                     branch=html!{<img src="/assets/spielrs_logo.png"></img>}>
///                         <NavbarContainer justify_content=JustifyContent::FlexStart(Mode::NoMode)>
///                             <NavbarItem
///                                 onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from("Home")))>
///                                 <span>{"Home"}</span>
///                             </NavbarItem>
///                             <NavbarItem
///                                 onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from("Shop")))>
///                                 <span>{"Shop"}</span>
///                             </NavbarItem>
///                             <NavbarItem
///                                 onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from("Shop")))>
///                                 <span>{"Shop"}</span>
///                             </NavbarItem>
///                             <NavbarItem
///                                 onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from("About us")))>
///                                 <span>{"About us"}</span>
///                             </NavbarItem>
///                             <NavbarItem
///                                 onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from("Contact")))>
///                                 <span>{"Contact"}</span>
///                             </NavbarItem>
///                         </NavbarContainer>
///                  </Navbar>
///             </div>
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

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    /// Type navbar style. Default `Standard`
    #[prop_or(Palette::Standard)]
    pub navbar_palette: Palette,
    /// Hide Navbar items in mobile. Default `false`
    #[prop_or(false)]
    pub hide_navbar_items_mobile: bool,
    /// Navbar styles. Default Regular
    #[prop_or(Style::Regular)]
    pub navbar_style: Style,
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
    /// The location of the navbar which is fixed. Default `Fixed::Top`
    #[prop_or(Fixed::Top)]
    pub fixed: Fixed,
    /// Vnode embedded in the beginning of the navbar, useful to include a branch logo
    #[prop_or_default]
    pub branch: Html,
    #[prop_or(css!(""))]
    pub styles: StyleSource<'static>,
    pub children: Children,
}

#[derive(Clone, PartialEq)]
pub struct NavbarProps {
    pub navbar_palette: String,
    pub navbar_style: String,
    pub hide_navbar_items_mobile: bool,
    pub key: String,
    pub code_ref: NodeRef,
    pub id: String,
    pub class_name: String,
    pub fixed: Fixed,
    pub branch: Html,
    pub styles: StyleSource<'static>,
    pub children: Children,
}

impl From<Props> for NavbarProps {
    fn from(props: Props) -> Self {
        NavbarProps {
            navbar_palette: get_palette(props.navbar_palette),
            navbar_style: get_style(props.navbar_style),
            hide_navbar_items_mobile: props.hide_navbar_items_mobile,
            key: props.key,
            code_ref: props.code_ref,
            id: props.id,
            class_name: props.class_name,
            fixed: props.fixed,
            branch: props.branch,
            children: props.children,
            styles: props.styles,
        }
    }
}

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
        if self.props != NavbarProps::from(props.clone()) {
            NavbarModel.init(self.props.clone());
            self.props = NavbarProps::from(props);
            if self.props.hide_navbar_items_mobile && self.display_menu {
                self.display_menu = false;
            }
            return true;
        }

        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div
                    class=classes!("navbar-mobile", self.props.navbar_style.clone(), self.props.navbar_palette.clone(), self.props.class_name.clone(), self.props.styles.clone())
                    id=self.props.id.clone()
                    key=self.props.key.clone()
                    ref=self.props.code_ref.clone()
                >
                    <div class="navbar-collapse">
                        <NavbarContainer justify_content=JustifyContent::FlexStart(Mode::NoMode)
                        direction=Direction::Row
                        class_name="navbar-container-mobile">
                        {get_branch(self.props.branch.clone())}
                        </NavbarContainer>
                        <NavbarContainer justify_content=JustifyContent::FlexEnd(Mode::NoMode)
                            direction=Direction::Row
                            class_name="navbar-container-mobile">
                            <NavbarItem
                                onclick_signal=self.link.callback(move |_| Msg::TroggleMenu)
                                class_name="navbar-menu-item"
                            >
                             <UxAssets
                                icon=UxIcon::Menu
                                size=(String::from("30"), String::from("30"))
                                class_name="navbar-menu"
                             />
                            </NavbarItem>
                        </NavbarContainer>
                    </div>
                    {if self.display_menu {
                        self.props.children.clone()
                    } else {
                        Children::new(vec![])
                    }}
                </div>

                <div
                    class=format!("navbar {} {} {}", self.props.navbar_style, self.props.navbar_palette, self.props.class_name)
                >
                <NavbarContainer justify_content=JustifyContent::Start(Mode::NoMode)
                    direction=Direction::Row
                    class_name="navbar-container-mobile">
                    {get_branch(self.props.branch.clone())}
                </NavbarContainer>
                    {if !self.display_menu {
                        self.props.children.clone()
                    } else {
                        Children::new(vec![])
                    } }
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
