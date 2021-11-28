use crate::page::highlighters::get_tooltip;
use yew::prelude::*;
use yew_prism::prism::Prism;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};

use yew_styles::styles::{get_size, get_style, Palette, Position, Size, Style};
use yew_styles::tooltip::Tooltip;

pub struct TooltipPage;

impl Component for TooltipPage {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{"Tooltip Component"}</h1>
                <h2>{"Features required"}</h2>
                <span><code>{"tooltip"}</code></span>
                <h2>{"Code example"}</h2>
                <Prism
                    code=get_tooltip()
                    language="rust"
                />
                <h2>{"Properties"}</h2>
                <ul>
                    <li><b>{"tooltip_style: "}</b>{"type tooltip style. Options included in "}<code>{"Pallete"}</code>{". Default "}<code>{"Standard"}</code>{"."}</li>
                    <li><b>{"tooltip_palette: "}</b>{"type spinner palette. Options included in "}<code>{"Pallete"}</code>{". Default "}<code>{"Standard"}</code>{"."}</li>
                    <li><b>{"tooltip_size: "}</b>{"three diffent tooltip standard sizes. Options included in "}<code>{"Size"}</code>{". Default "}<code>{"Medium"}</code>{"."}</li>
                    <li><b>{"tooltip_position: "}</b>{"The postion where the tooltip will show over the content target. Options included in "}<code>{"Left"}</code><code>{" Right "}</code><code>{"Above "}</code><code>{"Below"}</code>{"."}</li>
                    <li><b>{"content: "}</b>{"The content that the tooltip will show"}</li>
                    <li><b>{"key: "}</b>{"general property to add keys."}</li>
                    <li><b>{"code_ref: "}</b>{"general property to get the ref of the component."}</li>
                    <li><b>{"id: "}</b>{"general property to add custom id."}</li>
                    <li><b>{"class_name: "}</b>{"general property to add custom class styles."}</li>
                    <li><b>{"styles: "}</b>{"use stylist-rs to write styles in the component. Example: "}<code>{"css!(\"background-color: #918d94;\")"}</code></li>
                </ul>
                <h2>{"Visual examples"}</h2>
                {get_tooltip_styles()}
            </>
        }
    }
}

fn get_tooltip_styles() -> Html {
    let styles: Vec<Style> = vec![Style::Regular, Style::Light, Style::Outline];

    styles
        .into_iter()
        .enumerate()
        .map(move |(index_style, style)| {
            html! {
                <>
                    <h3>{get_style(style.clone()).to_uppercase()}</h3>
                    {get_sizes(style, index_style)}
                </>
            }
        })
        .collect::<Html>()
}

fn get_sizes(tooltip_style: Style, index_style: usize) -> Html {
    let sizes: Vec<Size> = vec![Size::Small, Size::Medium, Size::Big];

    sizes
        .into_iter()
        .enumerate()
        .map(move |(index_size, size)| {
            html! {
                <>
                    {get_tooltips_palette(size, tooltip_style.clone(), index_style, index_size)}
                </>
            }
        })
        .collect::<Html>()
}

fn get_tooltips_palette(
    size: Size,
    tooltip_style: Style,
    index_style: usize,
    index_size: usize,
) -> Html {
    let tooltip_palette: Vec<&str> = vec![
        "Standard",
        "Primary",
        "Secondary",
        "Info",
        "Link",
        "Success",
        "Warning",
        "Danger",
        "CLEAN",
    ];
    let tooltip_palette_enum: Vec<Palette> = vec![
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
    let mut index = 0;

    let tooltip_positions: Vec<Position> = vec![
        Position::Left,
        Position::Right,
        Position::Above,
        Position::Below,
    ];

    let tooltip_positions_text: Vec<String> = vec![
        String::from("left"),
        String::from("right"),
        String::from("above"),
        String::from("below"),
    ];

    html! {
        <div class="show-size">
            <h4>{get_size(size.clone()).to_uppercase()}</h4>
            {
                tooltip_palette.into_iter().map(|_| {
                    let button = html! {
                        <Container direction=Direction::Row wrap=Wrap::Wrap >
                            {tooltip_positions_text.clone().into_iter().enumerate().map(|(i,position)| {
                                html!{
                                    <Item layouts=vec![ItemLayout::ItXs(12), ItemLayout::ItM(6), ItemLayout::ItL(3)]>
                                        <Tooltip
                                            key=format!("tooltip-{}-{}-{}", index_style, index_size, tooltip_positions_text[i])
                                            tooltip_palette=tooltip_palette_enum[index].clone()
                                            tooltip_style=tooltip_style.clone()
                                            tooltip_position=tooltip_positions[i].clone()
                                            tooltip_size=size.clone()
                                            content=html!{<span>{"Info"}</span>}
                                            class_name="tooltip-page"
                                        >
                                            <div class="tooltip-content">{format!("Tooltip {}", position)}</div>
                                        </Tooltip>
                                    </Item>
                                }
                            }).collect::<Html>()}
                        </Container>
                    };
                    index += 1;
                    button
                }).collect::<Html>()
            }
        </div>
    }
}
