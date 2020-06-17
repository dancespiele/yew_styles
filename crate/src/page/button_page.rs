use super::highlighters::button_code;
use yew::prelude::*;
use yew_prism::Prism;
use yew_styles::{
    button::Button,
    styles::{get_pallete, get_size, get_style, Palette, Size, Style},
};

pub struct ButtonPage {
    link: ComponentLink<Self>,
    button_types: Vec<Vec<String>>,
}

pub enum Msg {
    ChangeType(String, usize, usize),
}

impl Component for ButtonPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ButtonPage {
            link,
            button_types: vec![vec!["".to_string(); 3]; 3],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeType(button_type, index_style, index_size) => {
                self.button_types[index_style][index_size] = button_type;
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
                <h1>{"Button Component"}</h1>

                <h2>{"Features required"}</h2>
                <span><code>{"button"}</code></span>

                <h2>{"Code example"}</h2>
                <Prism
                    code=button_code()
                    language="rust"
                />

                <h2>{"Propeties"}</h2>
                <ul>
                    <li><b>{"button_type: "}</b>{"type botton style. Options included in "}<code>{"Pallete"}</code>{". Default "}<code>{"Standard"}</code>{"."}</li>
                    <li><b>{"button_size: "}</b>{"three diffent button standard sizes. Options included in "}<code>{"Size"}</code>{". Default "}<code>{"Medium"}</code>{"."}</li>
                    <li><b>{"button_style: "}</b>{"button styles. Options included in "}<code>{"Style"}</code>{". Default "}<code>{"Regular"}</code>{"."}</li>
                    <li><b>{"onclick_signal: "}</b>{"click event for button. Required."}</li>
                    <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                    <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>
                </ul>

                <h2>{"Visual examples"}</h2>
                <div class="container-button">
                    <div class="buttons-example">
                        {get_button_styles(self.link.clone(), self.button_types.clone())}
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

fn get_button_styles(link: ComponentLink<ButtonPage>, value: Vec<Vec<String>>) -> Html {
    let styles: Vec<Style> = vec![Style::Regular, Style::Light, Style::Outline];

    styles
        .into_iter()
        .enumerate()
        .map(move |(index_style, style)| {
            html! {
                <>
                    <h3>{get_style(style.clone()).to_uppercase()}</h3>
                    {get_sizes(style, link.clone(), value.clone(), index_style)}
                </>
            }
        })
        .collect::<Html>()
}

fn get_sizes(
    button_style: Style,
    link: ComponentLink<ButtonPage>,
    value: Vec<Vec<String>>,
    index_style: usize,
) -> Html {
    let sizes: Vec<Size> = vec![Size::Small, Size::Medium, Size::Big];

    sizes
        .into_iter()
        .enumerate()
        .map(move |(index_size, size)| {
            html! {
                <>
                    {get_buttons(size, button_style.clone(), link.clone(), index_style, index_size)}
                    <div>
                        {format!("Value: {}", value[index_style][index_size])}
                    </div>
                </>
            }
        })
        .collect::<Html>()
}

fn get_buttons(
    size: Size,
    button_style: Style,
    link: ComponentLink<ButtonPage>,
    index_style: usize,
    index_size: usize,
) -> Html {
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
                button_types.into_iter().map(|bt| {
                    let button = html! {
                        <>
                            <Button
                                onclick_signal=link.callback(move |_| Msg::ChangeType(bt.to_string().clone(), index_style,index_size))
                                class_name="button-page"
                                button_type=button_types_enum[index].clone()
                                button_style=button_style.clone()
                                button_size=size.clone()
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
