use yew::prelude::*;
use yew_styles::navbar::{AligSelf, Fixed, Navbar, NavbarItem};

pub struct NavbarPage {
    link: ComponentLink<Self>,
    navbar_menu: Vec<String>,
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
            navbar_menu: vec![String::from("home"); 10],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeType(index, navbar_menu) => {
                self.navbar_menu[index] = navbar_menu;
            }
        };

        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <Navbar fixed=Fixed::Top>
                    {get_menus(self.link.clone())}
                </Navbar>
                <div>{self.navbar_menu[0].clone()}</div>
            </div>
        }
    }
}

fn get_menus(link: ComponentLink<NavbarPage>) -> Html {
    let menus = vec!["home", "shop", "about us", "contact us"];
    let mut index = 0;

    menus
        .into_iter()
        .map(|menu| {
            let item = html! {
                <>
                    <NavbarItem
                        side=AligSelf::Left
                        onsignal=link.callback(move |_| Msg::ChangeType(index, String::from(menu)))
                    >
                        <span>{menu}</span>
                    </NavbarItem>
                </>
            };

            index = index + 1;
            item
        })
        .collect::<Html>()
}
