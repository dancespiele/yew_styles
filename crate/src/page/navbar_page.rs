use super::highlighters::navbar_code;
use yew::prelude::*;
use yew_prism::Prism;
use yew_styles::{
    layouts::container::{JustifyContent, Mode},
    navbar::{
        navbar_component::{Fixed, Navbar},
        navbar_container::NavbarContainer,
        navbar_item::NavbarItem,
    },
    styles::{Palette, Style},
};

pub struct NavbarPage {
    link: ComponentLink<Self>,
    navbar_menu: Vec<String>,
}

#[derive(Clone)]
struct NavbarStyle {
    style: Style,
    name: String,
}

struct NavbarType {
    navbar_type: Palette,
    name: String,
}

struct ElementRender {
    element: Html,
    index: usize,
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
            navbar_menu: vec![String::from("home"); 50],
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
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
            <>
                <h1>{"Navbar Components"}</h1>

                <h2>{"Features required"}</h2>
                <span><code>{"navbar"}</code>{", "}<code>{"layouts"}</code>{", "}<code>{"asset"}</code></span>

                <h2>{"Code example"}</h2>
                <Prism
                    code=navbar_code()
                    language="rust"
                />

                <h2>{"Navbar properties"}</h2>
                <ul>
                    <li><b>{"navbar_type: "}</b>{"type navbar style. Options included in "}<code>{"Pallete"}</code>{". Default "}<code>{"Standard"}</code></li>
                    <li><b>{"button_style: "}</b>{"navbar styles. Options included in "}<code>{"Style"}</code>{". Default "}<code>{"Regular"}</code>{"."}</li>
                    <li><b>{"fixed: "}</b>{"the location of the navbar which is fixed .Options included in "}<code>{"Fixed"}</code>{". Default "}<code>{"Top"}</code>{"."}</li>
                    <li><b>{"branch: "}</b>{"vnode embedded in the beginning of the navbar, useful to include a branch logo. Optional"}</li>
                    <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                    <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>
                </ul>

                <h2>{"Navbar Container properties"}</h2>
                <ul>
                    <li><b>{"justify_content: "}</b>{"set how will be justified the navbar items. Options included in "}<code>{"JustifyContent"}</code>{". Default "}<code>{"FlexStart(No Mode)"}</code>{"."}</li>
                    <li><b>{"direction: "}</b>{"which direction are placing the navbar items. Options include in "}<code>{"Direction"}</code>{". Default "}<code>{"Row"}</code>{"."}</li>
                    <li><b>{"mode: "}</b>{"safe postion handler which is additional option for justify_content. Options included in "}<code>{"Mode"}</code>{". Default "}<code>{"NoMode"}</code>{"."}</li>
                    <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                    <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>
                </ul>

                <h2>{"Navbar Item properties"}</h2>
                <ul>
                    <li><b>{"onclick_signal: "}</b>{"click event for navbar item. Default "}<code>{"noop()"}</code></li>
                    <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                    <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>
                </ul>

                <h2>{"Visual examples"}</h2>
                {get_style(self.link.clone(), self.navbar_menu.clone())}
            </>
        }
    }
}

fn get_style(link: ComponentLink<NavbarPage>, navbar_menu: Vec<String>) -> Html {
    let styles = vec![
        NavbarStyle {
            style: Style::Regular,
            name: String::from("Regular Navbar"),
        },
        NavbarStyle {
            style: Style::Outline,
            name: String::from("Outline Navbar"),
        },
        NavbarStyle {
            style: Style::Light,
            name: String::from("Line Navbar"),
        },
    ];

    let mut index = 0;
    styles
        .into_iter()
        .map(|style| {
            let navbar = get_navbar_type(link.clone(), style, navbar_menu.clone(), index);

            index = navbar.index + 1;

            navbar.element
        })
        .collect::<Html>()
}

fn get_navbar_type(
    link: ComponentLink<NavbarPage>,
    style: NavbarStyle,
    navbar_menu: Vec<String>,
    index: usize,
) -> ElementRender {
    let mut navbar_type_rendered = index;
    let types = vec![
        NavbarType {
            navbar_type: Palette::Standard,
            name: String::from("Standard"),
        },
        NavbarType {
            navbar_type: Palette::Primary,
            name: String::from("Primary"),
        },
        NavbarType {
            navbar_type: Palette::Secondary,
            name: String::from("Secondary"),
        },
        NavbarType {
            navbar_type: Palette::Info,
            name: String::from("Info"),
        },
        NavbarType {
            navbar_type: Palette::Link,
            name: String::from("Link"),
        },
        NavbarType {
            navbar_type: Palette::Success,
            name: String::from("Success"),
        },
        NavbarType {
            navbar_type: Palette::Warning,
            name: String::from("Warning"),
        },
        NavbarType {
            navbar_type: Palette::Danger,
            name: String::from("Danger"),
        },
    ];

    let navbar = types
        .into_iter()
        .map(|navbar_type| {
            let element = html! {
                <div>
                    <h3>{format!("{} {}",style.name.clone(), navbar_type.name)}</h3>
                    <Navbar
                        fixed=Fixed::None
                        navbar_style=style.style.clone()
                        navbar_type=navbar_type.navbar_type
                        branch=html!{<img src="/spielrs_logo.png"/>}
                    >
                        <NavbarContainer justify_content=JustifyContent::FlexStart(Mode::NoMode)>
                            {get_menus(link.clone(), navbar_type_rendered)}
                        </NavbarContainer>
                    </Navbar>
                    <div>{navbar_menu[navbar_type_rendered].clone()}</div>
                </div>
            };

            navbar_type_rendered += 1;

            element
        })
        .collect::<Html>();

    ElementRender {
        element: navbar,
        index: navbar_type_rendered,
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
                        onclick_signal=link.callback(move |_| Msg::ChangeType(index, String::from(menu))
                    )
                    >
                        <span>{menu}</span>
                    </NavbarItem>
                </>
            }
        })
        .collect::<Html>()
}
