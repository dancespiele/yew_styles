use super::highlighters::get_dropdown;
use inflector::Inflector;
use yew::prelude::*;
use yew_assets::controller_assets::{ControllerAssets, ControllerIcon};
use yew_prism::Prism;
use yew_styles::dropdown::{Dropdown, DropdownItem};
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::{get_style, Palette, Size, Style};

pub struct DropDownPage {
    link: ComponentLink<Self>,
    menu: String,
}

pub enum Msg {
    ChangeMenu(String),
}

impl Component for DropDownPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            menu: String::from(""),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeMenu(menu) => {
                self.menu = menu;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{"Dropdown Component"}</h1>

                <h2>{"Features required"}</h2>
                <span><code>{"dropdown"}</code></span>

                <h2>{"Code example"}</h2>
                <Prism
                    code=get_dropdown()
                    language="rust"
                />

                <h2>{"Dropdown Container properties"}</h2>
                <ul>
                    <li><b>{"main_content: "}</b>{"clickeable content to show the dropdown. Required"}</li>
                    <li><b>{"key: "}</b>{"general property to add keys."}</li>
                    <li><b>{"dropdown_palette: "}</b>{"type dropdown style. Options included in "}<code>{"Pallete"}</code>{". Default "}<code>{"Standard"}</code>{"."}</li>
                    <li><b>{"dropdown_size: "}</b>{"three diffent dropdown standard sizes. Options included in "}<code>{"Size"}</code>{". Default "}<code>{"Medium"}</code>{"."}</li>
                    <li><b>{"dropdown_style: "}</b>{"dropdown styles. Options included in "}<code>{"Style"}</code>{". Default "}<code>{"Regular"}</code>{"."}</li>
                    <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                    <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>
                    <li><b>{"styles: "}</b>{"use stylist-rs to write styles in the component. Example: "}<code>{"css!(\"background-color: #918d94;\")"}</code></li>
                </ul>

                <h2>{"Dropdown Item properties"}</h2>
                <ul>
                    <li><b>{"onclick_signal: "}</b>{"click event for dropdown item. Default "}<code>{"noop()"}</code></li>
                    <li><b>{"key: "}</b>{"general property to add keys."}</li>
                    <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                    <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>
                    <li><b>{"styles: "}</b>{"use stylist-rs to write styles in the component. Example: "}<code>{"css!(\"background-color: #918d94;\")"}</code></li>
                </ul>

                <h2>{"Visual Example"}</h2>
                <h3>{"Sizes"}</h3>
                <Container direction=Direction::Row wrap=Wrap::Wrap>
                    {get_sizes(self.link.clone())}
                </Container>
                <h3>{"Styles"}</h3>
                <Container direction=Direction::Row wrap=Wrap::Wrap>
                    {get_styles(self.link.clone())}
                </Container>
            </>
        }
    }
}

fn get_sizes(link: ComponentLink<DropDownPage>) -> Html {
    let sizes: Vec<Size> = vec![Size::Small, Size::Medium, Size::Big];

    sizes
        .into_iter()
        .map(|size| {
            html! {
                <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                <Dropdown
                    main_content=html!{<span>{"Menu"}<ControllerAssets
                        icon=ControllerIcon::ChevronDown
                        size=("20".to_string(), "20".to_string())
                    /></span>}
                    dropdown_size=size
                    >
                    <DropdownItem
                        onclick_signal=link.callback(|_| Msg::ChangeMenu(String::from("Menu 1")))>{"Menu 1"}</DropdownItem>
                    <DropdownItem
                        onclick_signal=link.callback(|_| Msg::ChangeMenu(String::from("Menu 2")))>{"Menu 2"}</DropdownItem>
                    <DropdownItem
                        onclick_signal=link.callback(|_| Msg::ChangeMenu(String::from("Menu 3")))>{"Menu 3"}</DropdownItem>
                </Dropdown>
                </Item>
            }
        })
        .collect::<Html>()
}

fn get_styles(link: ComponentLink<DropDownPage>) -> Html {
    let styles: Vec<Style> = vec![Style::Regular, Style::Outline, Style::Light];

    styles
        .into_iter()
        .map(|style| {
            html! {
                <>
                    <Item layouts=vec!(ItemLayout::ItXs(12))>
                        <h4>{get_style(style.clone()).to_pascal_case()}</h4>
                    </Item>
                    {get_types(style, link.clone())}
                </>
            }
        })
        .collect::<Html>()
}

fn get_types(style: Style, link: ComponentLink<DropDownPage>) -> Html {
    let palette: Vec<Palette> = vec![
        Palette::Standard,
        Palette::Primary,
        Palette::Secondary,
        Palette::Info,
        Palette::Link,
        Palette::Success,
        Palette::Warning,
        Palette::Danger,
        Palette::Clean,
    ];

    palette
        .into_iter()
        .map(|palette| {
            html! {
                <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                    <Dropdown
                        main_content=html!{<span>{"Menu"}<ControllerAssets
                            icon=ControllerIcon::ChevronDown
                            size=("20".to_string(), "20".to_string())
                        /></span>}
                        dropdown_style=style.clone()
                        dropdown_palette=palette
                        >
                        <DropdownItem
                            onclick_signal=link.callback(|_| Msg::ChangeMenu(String::from("Menu 1")))>{"Menu 1"}</DropdownItem>
                        <DropdownItem
                            onclick_signal=link.callback(|_| Msg::ChangeMenu(String::from("Menu 2")))>{"Menu 2"}</DropdownItem>
                        <DropdownItem
                            onclick_signal=link.callback(|_| Msg::ChangeMenu(String::from("Menu 3")))>{"Menu 3"}</DropdownItem>
                    </Dropdown>
                </Item>
            }
        })
        .collect::<Html>()
}
