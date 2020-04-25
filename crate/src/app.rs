use page::{ButtonPage, HomePage, LayoutsPage, NavbarPage};
use yew::prelude::*;
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};
use yew_styles::layouts::{
    container::{AlignItems, Container, Direction, Mode, Wrap},
    item::{Item, ItemLayout},
};

pub struct App;

#[derive(Switch, Debug, Clone)]
pub enum AppRouter {
    #[to = "/!"]
    RootPath,
    #[to = "/button!"]
    ButtonPath,
    #[to = "/layouts!"]
    LayoutsPath,
    #[to = "/navbar!"]
    NavbarPath,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <Container direction=Direction::Row wrap=Wrap::Wrap>
                <Item layouts=vec!(ItemLayout::ItL(2), ItemLayout::ItXs(12))>
                    <Container
                        direction=Direction::Column
                        align_items=AlignItems::FlexStart(Mode::NoMode)
                        wrap=Wrap::Wrap
                    >
                        <Item layouts=vec!(ItemLayout::ItXs(12)) class_name="component-link">
                            <h2>{"Documentation"}</h2>
                        </Item>
                        <Item layouts=vec!(ItemLayout::ItXs(12)) class_name="component-link">
                            <RouterAnchor<AppRouter> route=AppRouter::RootPath>{"Let's start"}</RouterAnchor<AppRouter>>
                        </Item>
                        <Item layouts=vec!(ItemLayout::ItXs(12)) class_name="component-link">
                            <RouterAnchor<AppRouter> route=AppRouter::LayoutsPath>{"Layouts"}</RouterAnchor<AppRouter>>
                        </Item>
                        <Item layouts=vec!(ItemLayout::ItXs(12)) class_name="component-link">
                            <RouterAnchor<AppRouter> route=AppRouter::ButtonPath>{"Button"}</RouterAnchor<AppRouter>>
                        </Item>
                        <Item layouts=vec!(ItemLayout::ItXs(12)) class_name="component-link">
                            <RouterAnchor<AppRouter> route=AppRouter::NavbarPath>{"Navbar"}</RouterAnchor<AppRouter>>
                        </Item>
                    </Container>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItL(10) )>
                    <Router<AppRouter, ()>
                        render = Router::render(|switch: AppRouter | {
                            match switch {
                                AppRouter::RootPath => html!{<HomePage/>},
                                AppRouter::ButtonPath => html!{<ButtonPage/>},
                                AppRouter::LayoutsPath => html!{<LayoutsPage/>},
                                AppRouter::NavbarPath => html!{<NavbarPage/>},
                                AppRouter::PageNotFound(Permissive(None)) => html!{"Page not found"},
                                AppRouter::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                            }
                        } )
                        redirect = Router::redirect(|route: Route<()>| {
                            AppRouter::PageNotFound(Permissive(Some(route.route)))
                        })
                    />
                </Item>
            </Container>
        }
    }
}
