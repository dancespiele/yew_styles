use lipsum::lipsum;
use yew::prelude::*;
use yew_prism::Prism;
use yew_styles::card::Card;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::{Palette, Size, Style};

pub struct CardPage {
    link: ComponentLink<Self>,
}

impl Component for CardPage {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        CardPage { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{"Card Component"}</h1>
                <Container direction=Direction::Row wrap=Wrap::Wrap>
                    {get_sizes()}
                </Container>
                <Container direction=Direction::Row wrap=Wrap::Wrap>
                    {get_styles()}
                </Container>
            </>
        }
    }
}

fn get_sizes() -> Html {
    let sizes: Vec<Size> = vec![Size::Small, Size::Medium, Size::Big];

    sizes
        .into_iter()
        .map(|size| {
            html! {
                <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                    <Card
                        card_size=size
                        header=Some(html!{<div class="image">
                            {"image"}
                        </div>})
                        body=Some(html!{
                            <div class="content">{lipsum(10)}</div>
                        })
                        footer=Some(html!{
                            <div>{lipsum(3)}</div>
                        })
                    />
                </Item>
            }
        })
        .collect::<Html>()
}

fn get_styles() -> Html {
    let styles: Vec<Style> = vec![Style::Regular, Style::Outline, Style::Light];

    styles
        .into_iter()
        .map(|style| get_types(style))
        .collect::<Html>()
}

fn get_types(style: Style) -> Html {
    let types: Vec<Palette> = vec![
        Palette::Standard,
        Palette::Primary,
        Palette::Secondary,
        Palette::Info,
        Palette::Link,
        Palette::Success,
        Palette::Warning,
        Palette::Danger,
    ];

    types
        .into_iter()
        .map(|card_type| {
            html! {
                <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                    <Card
                        card_style=style.clone()
                        card_type=card_type
                        header=Some(html!{<div class="image">
                            {"image"}
                        </div>})
                        body=Some(html!{
                            <div class="content">{lipsum(10)}</div>
                        })
                        footer=Some(html!{
                            <div>{lipsum(3)}</div>
                        })
                    />
                </Item>
            }
        })
        .collect::<Html>()
}
