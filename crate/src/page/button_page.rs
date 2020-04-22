use super::highlighters::button_code;
use yew::prelude::*;
use yew_prism::Prism;
use yew_styles::{
    button::{get_size, Button, Size},
    layouts::item::{Item, ItemLayout},
    styles::{get_pallete, get_style, Palette, Style},
};

pub struct ButtonPage {
    link: ComponentLink<Self>,
    button_type: String,
}

pub enum Msg {
    ChangeType(String),
}

#[derive(Clone, Properties)]
pub struct Props {}

impl Component for ButtonPage {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ButtonPage {
            link,
            button_type: String::from("standard"),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeType(type_button) => {
                self.button_type = type_button;
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h1>{"Button Component"}</h1>
                </Item>

                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h2>{"Code example"}</h2>
                    <Prism
                        code=button_code()
                        language="rust"
                    />
                </Item>

                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h2>{"Propeties"}</h2>
                    <ul>
                        <li><b>{"button_type: "}</b>{"type botton style. Options included in "}<code>{"Pallete"}</code>{". Default "}<code>{"Standard"}</code></li>
                        <li><b>{"size: "}</b>{"three diffent button standard sizes. Options included in "}<code>{"Size"}</code>{". Default "}<code>{"Medium"}</code></li>
                        <li><b>{"button_style: "}</b>{"button styles. Options included in "}<code>{"Style"}</code>{". Default "}<code>{"Regular"}</code></li>
                        <li><b>{"onsignal: "}</b>{"click event for button. Required"}</li>
                    </ul>
                </Item>

                <h2>{"Visual examples"}</h2>
                <div class="container-button">
                    <div class="buttons-example">
                        {get_button_styles(self.link.clone())}
                    </div>
                    <div class="action">
                        {self.button_type.clone()}
                    </div>
                </div>
            </>
        }
    }
}

fn to_first_upercase(word: &str) -> String {
    let mut letters = word.chars();

    match letters.next() {
        None => String::new(),
        Some(letter) => letter.to_uppercase().collect::<String>() + letters.as_str(),
    }
}

fn get_button_styles(link: ComponentLink<ButtonPage>) -> Html {
    let styles: Vec<Style> = vec![Style::Regular, Style::Light, Style::Outline];

    styles
        .into_iter()
        .map(move |style| {
            html! {
                <>
                    <h3>{get_style(style.clone()).to_uppercase()}</h3>
                    {get_sizes(style, link.clone())}
                </>
            }
        })
        .collect::<Html>()
}

fn get_sizes(button_style: Style, link: ComponentLink<ButtonPage>) -> Html {
    let sizes: Vec<Size> = vec![Size::Small, Size::Medium, Size::Big];

    sizes
        .into_iter()
        .map(move |size| get_buttons(size, button_style.clone(), link.clone()))
        .collect::<Html>()
}

fn get_buttons(size: Size, button_style: Style, link: ComponentLink<ButtonPage>) -> Html {
    let button_types: Vec<&str> = vec![
        "Standard",
        "Primary",
        "Secondary",
        "Info",
        "Link",
        "Success",
        "Warning",
        "Danger",
    ];
    let button_types_enum: Vec<Palette> = vec![
        Palette::Standard,
        Palette::Primary,
        Palette::Secondary,
        Palette::Info,
        Palette::Link,
        Palette::Success,
        Palette::Warning,
        Palette::Danger,
    ];
    let mut index = 0;

    html! {
        <div class="show-size">
            <h4>{get_size(size.clone()).to_uppercase()}</h4>
            {
                button_types.into_iter().map(|button_type| {
                    let button = html! {
                        <>
                            <Button
                                onsignal=link.callback(move |_| Msg::ChangeType(button_type.to_string().clone()))
                                class_name="button-page"
                                button_type=button_types_enum[index].clone()
                                button_style=button_style.clone()
                                size=size.clone()
                            >{to_first_upercase(&get_pallete(button_types_enum[index].clone()))}
                            </Button>
                        </>
                    };
                    index += 1;
                    button
                }).collect::<Html>()
            }
        </div>
    }
}
