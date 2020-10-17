use super::highlighters::{navbar_code, navbar_with_a_tag};
use wasm_bindgen::JsCast;
use web_sys::Element;
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
    item_menu: Vec<Vec<bool>>,
    close_navbar_mobile: bool,
}

#[derive(Clone)]
struct NavbarStyle {
    style: Style,
    name: String,
}

struct NavbarType {
    navbar_palette: Palette,
    name: String,
}

struct ElementRender {
    element: Html,
    index: usize,
}

pub enum Msg {
    ChangeType(usize, usize, String),
    CloseNavarMobile(MouseEvent),
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
            item_menu: vec![vec![true, false, false, false]; 50],
            close_navbar_mobile: false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeType(index, item_index, navbar_menu) => {
                self.navbar_menu[index] = navbar_menu;
                for (i, _) in self.item_menu[index].clone().into_iter().enumerate() {
                    self.item_menu[index][i] = false;
                }
                self.item_menu[index][item_index] = true;
            }
            Msg::CloseNavarMobile(mouse_event) => {
                let target_class = mouse_event
                    .target()
                    .unwrap()
                    .dyn_into::<Element>()
                    .unwrap()
                    .class_list();

                let target_parent = mouse_event
                    .target()
                    .unwrap()
                    .dyn_into::<Element>()
                    .unwrap()
                    .parent_element()
                    .unwrap()
                    .tag_name();

                if !target_class.value().contains("navbar-menu") && target_parent != "svg" {
                    self.close_navbar_mobile = true;
                } else {
                    self.close_navbar_mobile = false
                }
            }
        };

        true
    }

    fn view(&self) -> Html {
        html! {
            <div onclick=self.link.callback(|e| Msg::CloseNavarMobile(e))>
                <h1>{"Navbar Components"}</h1>

                <h2>{"Features required"}</h2>
                <span><code>{"navbar"}</code></span>

                <h2>{"Code example"}</h2>
                <Prism
                    code=navbar_code()
                    language="rust"
                />

                <h2>{"Navbar properties"}</h2>
                <ul>
                    <li><b>{"navbar_palette: "}</b>{"type navbar style. Options included in "}<code>{"Pallete"}</code>{". Default "}<code>{"Standard"}</code></li>
                    <li><b>{"button_style: "}</b>{"navbar styles. Options included in "}<code>{"Style"}</code>{". Default "}<code>{"Regular"}</code>{"."}</li>
                    <li><b>{"hide_navbar_items_mobile: "}</b>{"Hide Navbar items in mobile. Default "}<code>{"false"}</code></li>
                    <li><b>{"fixed: "}</b>{"the location of the navbar which is fixed .Options included in "}<code>{"Fixed"}</code>{". Default "}<code>{"Top"}</code>{"."}</li>
                    <li><b>{"branch: "}</b>{"vnode embedded in the beginning of the navbar, useful to include a branch logo. Optional"}</li>
                    <li><b>{"key: "}</b>{"general property to add keys."}</li>
                    <li><b>{"code_ref: "}</b>{"general property to get the ref of the component."}</li>
                    <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                    <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>
                </ul>

                <h2>{"Navbar Container properties"}</h2>
                <ul>
                    <li><b>{"justify_content: "}</b>{"set how will be justified the navbar items. Options included in "}<code>{"JustifyContent"}</code>{". Default "}<code>{"FlexStart(No Mode)"}</code>{"."}</li>
                    <li><b>{"direction: "}</b>{"which direction are placing the navbar items. Options include in "}<code>{"Direction"}</code>{". Default "}<code>{"Row"}</code>{"."}</li>
                    <li><b>{"mode: "}</b>{"safe postion handler which is additional option for justify_content. Options included in "}<code>{"Mode"}</code>{". Default "}<code>{"NoMode"}</code>{"."}</li>
                    <li><b>{"key: "}</b>{"general property to add keys."}</li>
                    <li><b>{"code_ref: "}</b>{"general property to get the ref of the component."}</li>
                    <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                    <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>
                </ul>

                <h2>{"Navbar Item properties"}</h2>
                <ul>
                    <li><b>{"active: "}</b>{"active nav item style. Default "}<code>{"false"}</code></li>
                    <li><b>{"interaction_effect: "}</b>{"if hove, focus, active effects are enable. Default "}<code>{"true"}</code>{"."}</li>
                    <li><b>{"onclick_signal: "}</b>{"click event for navbar item. Default "}<code>{"noop()"}</code></li>
                    <li><b>{"key: "}</b>{"general property to add keys."}</li>
                    <li><b>{"code_ref: "}</b>{"general property to get the ref of the component."}</li>
                    <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                    <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>
                </ul>

                <p><b>{"Note:"}</b>{" for navbar items which include yew routers or"}<code>{" a "}</code>
                    {"tag, add navbar-route in the class_name attribute of the "}<code>{"NavbarItem"}</code>
                    {" component and navbar-router in the "}<code>{"Navbar"}</code>{". For example:"}</p>

                <Prism
                    code=navbar_with_a_tag()
                    language="rust"/>

                <h2>{"Visual examples"}</h2>
                {get_style(self.link.clone(), self.navbar_menu.clone(), self.item_menu.clone(), self.close_navbar_mobile)}
            </div>
        }
    }
}

fn get_style(
    link: ComponentLink<NavbarPage>,
    navbar_menu: Vec<String>,
    item_menu: Vec<Vec<bool>>,
    close_navbar_mobile: bool,
) -> Html {
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
            let navbar = get_navbar_palette(
                link.clone(),
                style,
                navbar_menu.clone(),
                item_menu.clone(),
                index,
                close_navbar_mobile,
            );

            index = navbar.index + 1;

            navbar.element
        })
        .collect::<Html>()
}

fn get_navbar_palette(
    link: ComponentLink<NavbarPage>,
    style: NavbarStyle,
    navbar_menu: Vec<String>,
    item_menu: Vec<Vec<bool>>,
    index: usize,
    close_navbar_mobile: bool,
) -> ElementRender {
    let mut navbar_palette_rendered = index;
    let types = vec![
        NavbarType {
            navbar_palette: Palette::Standard,
            name: String::from("Standard"),
        },
        NavbarType {
            navbar_palette: Palette::Primary,
            name: String::from("Primary"),
        },
        NavbarType {
            navbar_palette: Palette::Secondary,
            name: String::from("Secondary"),
        },
        NavbarType {
            navbar_palette: Palette::Info,
            name: String::from("Info"),
        },
        NavbarType {
            navbar_palette: Palette::Link,
            name: String::from("Link"),
        },
        NavbarType {
            navbar_palette: Palette::Success,
            name: String::from("Success"),
        },
        NavbarType {
            navbar_palette: Palette::Warning,
            name: String::from("Warning"),
        },
        NavbarType {
            navbar_palette: Palette::Danger,
            name: String::from("Danger"),
        },
        NavbarType {
            navbar_palette: Palette::Clean,
            name: String::from("Clean"),
        },
    ];

    let navbar = types
        .into_iter()
        .map(|navbar_palette| {
            let element = html! {
                <div>
                    <h3>{format!("{} {}",style.name.clone(), navbar_palette.name)}</h3>
                    <Navbar
                        fixed=Fixed::None
                        navbar_style=style.style.clone()
                        navbar_palette=navbar_palette.navbar_palette
                        hide_navbar_items_mobile = close_navbar_mobile
                        branch=html!{<img src="/spielrs_logo.png"/>}
                    >
                        <NavbarContainer justify_content=JustifyContent::FlexStart(Mode::NoMode)>
                            {get_menus(link.clone(), navbar_palette_rendered, item_menu.clone())}
                        </NavbarContainer>
                    </Navbar>
                    <div>{navbar_menu[navbar_palette_rendered].clone()}</div>
                </div>
            };

            navbar_palette_rendered += 1;

            element
        })
        .collect::<Html>();

    ElementRender {
        element: navbar,
        index: navbar_palette_rendered,
    }
}

fn get_menus(link: ComponentLink<NavbarPage>, index: usize, item_menu: Vec<Vec<bool>>) -> Html {
    let menus = vec!["home", "shop", "about us", "contact us", "no interaction"];

    menus
        .into_iter()
        .enumerate()
        .map(|(item_index, menu)| {
            html! {
                <>
                    {
                        if menu != "no interaction" {
                            html!{
                                <NavbarItem
                                key=format!("nabvar-item-{}", item_index)
                                active= item_menu[index][item_index]
                                onclick_signal=link.callback(move |_| Msg::ChangeType(index, item_index, String::from(menu)))
                                >
                                    <span>{menu}</span>
                                </NavbarItem>
                            }
                        } else {
                            html!{
                                <NavbarItem
                                    key=format!("nabvar-item-{}", item_index)
                                    interaction_effect=false
                                >
                                    <span>{menu}</span>
                                </NavbarItem>
                            }
                        }
                    }
                </>
            }
        })
        .collect::<Html>()
}
