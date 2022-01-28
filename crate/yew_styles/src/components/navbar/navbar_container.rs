use crate::layouts::container::{Container, Direction, JustifyContent, Mode, Wrap};
use yew::prelude::*;

pub enum Msg {
    Clicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// set how will be justified the navbar items. Default `JustifyContent::FlexStart(Mode::NoMode)`
    #[prop_or(JustifyContent::FlexStart(Mode::NoMode))]
    pub justify_content: JustifyContent,
    /// which direction are placing the navbar items. Default `Direction::Row`
    #[prop_or(Direction::Row)]
    pub direction: Direction,
    pub children: Children,
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
}

/// # Navbar Container component
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
///                    </NavbarContainer>
///              </Navbar>
///         }
///     }
/// }
/// ```
pub struct NavbarContainer;

impl Component for NavbarContainer {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            justify_content,
            direction,
            children,
            code_ref,
            key,
            class_name,
            id,
        } = &ctx.props();

        html! {
            <Container
                class_name={format!("navbar-container {}", class_name)}
                id={id.clone()}
                key={key.clone()}
                ref={code_ref.clone()}
                direction={direction.clone()}
                wrap={Wrap::Wrap}
                justify_content={justify_content.clone()}>
                    {children.clone()}
            </Container>
        }
    }
}
