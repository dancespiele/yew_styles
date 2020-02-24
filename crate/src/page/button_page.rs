use components::{
    get_button_style, get_button_type, get_size, Button, ButtonStyle, ButtonType, Size,
};
use yew::prelude::*;

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
            <div class="container">
                <div class="buttons-example">
                    {get_button_styles(self.link.clone())}
                </div>
                <div class="action">
                    {self.button_type.clone()}
                </div>
            </div>
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
    let styles: Vec<ButtonStyle> = vec![
        ButtonStyle::Regular,
        ButtonStyle::Light,
        ButtonStyle::Outline,
    ];

    styles
        .into_iter()
        .map(move |style| {
            html! {
                <>
                    <h2>{get_button_style(style.clone()).to_uppercase()}</h2>
                    {get_sizes(style, link.clone())}
                </>
            }
        })
        .collect::<Html>()
}

fn get_sizes(button_style: ButtonStyle, link: ComponentLink<ButtonPage>) -> Html {
    let sizes: Vec<Size> = vec![Size::Small, Size::Medium, Size::Big];

    sizes
        .into_iter()
        .map(move |size| get_buttons(size, button_style.clone(), link.clone()))
        .collect::<Html>()
}

fn get_buttons(size: Size, button_style: ButtonStyle, link: ComponentLink<ButtonPage>) -> Html {
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
    let button_types_enum: Vec<ButtonType> = vec![
        ButtonType::Standard,
        ButtonType::Primary,
        ButtonType::Secondary,
        ButtonType::Info,
        ButtonType::Link,
        ButtonType::Success,
        ButtonType::Warning,
        ButtonType::Danger,
    ];
    let mut index = 0;

    html! {
        <div class="show-size">
            <h3>{get_size(size.clone()).to_uppercase()}</h3>
            {
                button_types.into_iter().map(|button_type| {
                    let button = html! {
                        <Button
                            onsignal=link.callback(move |_| Msg::ChangeType(button_type.to_string().clone()))
                            class_name="button-page"
                            button_type=button_types_enum[index].clone()
                            button_style=button_style.clone()
                            size=size.clone()
                        >{to_first_upercase(&get_button_type(button_types_enum[index].clone()))}
                        </Button>
                    };
                    index = index + 1;
                    button
                }).collect::<Html>()
            }
        </div>
    }
}
