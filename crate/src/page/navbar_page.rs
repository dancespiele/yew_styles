use yew::prelude::*;
use yew::services::ConsoleService;
use yew_styles::{
    container::{JustifyContent, Mode},
    navbar::{Fixed, Navbar, NavbarContainer, NavbarItem},
};

pub struct NavbarPage {
    link: ComponentLink<Self>,
    navbar_menu: Vec<String>,
    console: ConsoleService,
}

pub enum Msg {
    ChangeType(usize, String),
}

#[derive(Clone, Properties)]
pub struct Props {}

impl Component for NavbarPage {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NavbarPage {
            link,
            console: ConsoleService::new(),
            navbar_menu: vec![String::from("home"); 10],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeType(index, navbar_menu) => {
                let index_log = index.to_string();
                self.console.info(index_log.as_ref());
                self.navbar_menu[index] = navbar_menu;
            }
        };

        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="root">
                <h2>{"Basic"}</h2>
                <Navbar
                    fixed=Fixed::None
                    branch=html!{<img src="/assets/spielrs_logo.png"></img>}
                >
                    <NavbarContainer justify_content=JustifyContent::FlexStart(Mode::NoMode)>
                        {get_menus(self.link.clone(), 0)}
                    </NavbarContainer>
                </Navbar>
                <div>{self.navbar_menu[0].clone()}</div>
            </div>
        }
    }
}

fn get_menus(link: ComponentLink<NavbarPage>, index: usize) -> Html {
    let menus = vec!["home", "shop", "about us", "contact us"];

    menus
        .into_iter()
        .map(|menu| {
            html! {
                <>
                    <NavbarItem
                        onsignal=link.callback(move |_| Msg::ChangeType(index, String::from(menu)))
                    >
                        <span>{menu}</span>
                    </NavbarItem>
                </>
            }
        })
        .collect::<Html>()
}
