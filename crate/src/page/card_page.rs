use super::highlighters::get_card;
use inflector::Inflector;
use lipsum::lipsum;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;
use yew::utils;
use yew_prism::Prism;
use yew_styles::card::Card;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::helpers::{get_palette, get_size, get_style, Palette, Size, Style};

pub enum Msg {
    Dragged(DragEvent),
    DraggedOver(DragEvent),
    Dropped(DragEvent),
}

pub struct CardPage {
    link: ComponentLink<Self>,
}

impl Component for CardPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        CardPage { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Dragged(drag_event) => {
                let target_id = drag_event
                    .target()
                    .unwrap()
                    .dyn_into::<Element>()
                    .unwrap()
                    .id();

                drag_event
                    .data_transfer()
                    .unwrap()
                    .set_data("application/card-component", &target_id)
                    .unwrap();

                drag_event.data_transfer().unwrap().set_drop_effect("move");
            }
            Msg::DraggedOver(drag_event) => {
                drag_event.prevent_default();

                drag_event.data_transfer().unwrap().set_drop_effect("move");
            }

            Msg::Dropped(drag_event) => {
                drag_event.prevent_default();

                let data = drag_event
                    .data_transfer()
                    .unwrap()
                    .get_data("application/card-component")
                    .unwrap();

                let target_element = drag_event.target().unwrap().dyn_into::<Element>().unwrap();

                if target_element
                    .append_child(&utils::document().get_element_by_id(&data).unwrap())
                    .is_ok()
                {
                    target_element
                        .append_child(&utils::document().get_element_by_id(&data).unwrap())
                        .unwrap();
                }
            }
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{"Card Component"}</h1>

                <h2>{"Features required"}</h2>
                <span><code>{"card"}</code></span>

                <h2>{"Code example"}</h2>
                    <Prism
                        code=get_card()
                        language="rust"
                    />

                <h2>{"Propeties"}</h2>
                <ul>
                    <li><b>{"card_palette: "}</b>{"type card purpose style. Options included in "}<code>{"Pallete"}</code>{". Default "}<code>{"Standard"}</code>{"."}</li>
                    <li><b>{"card_size: "}</b>{"three diffent card standard sizes. Options included in "}<code>{"Size"}</code>{". Default "}<code>{"Medium"}</code>{"."}</li>
                    <li><b>{"card_style: "}</b>{"card styles. Options included in "}<code>{"Style"}</code>{". Default "}<code>{"Regular"}</code>{"."}</li>
                    <li><b>{"onclick_signal: "}</b>{"click event for card."}</li>
                    <li><b>{"ondrag_signal: "}</b>{"a dragged item (element or text selection) is dragged."}</li>
                    <li><b>{"ondragend_signal: "}</b>{"a drag operation ends."}</li>
                    <li><b>{"ondragenter_signal: "}</b>{"a dragged item enters a valid drop target."}</li>
                    <li><b>{"ondragexit_signal: "}</b>{"an element is no longer the drag operation's immediate selection target."}</li>
                    <li><b>{"ondragleave_signal: "}</b>{"a dragged item leaves a valid drop target."}</li>
                    <li><b>{"ondragover_signal: "}</b>{"a dragged item is being dragged over a valid drop target, every few hundred milliseconds."}</li>
                    <li><b>{"ondragstart_signal: "}</b>{"the user starts dragging an item."}</li>
                    <li><b>{"ondrop_signal: "}</b>{"an item is dropped on a valid drop target."}</li>
                    <li><b>{"draggable: "}</b>{"if the item is draggable. Default "}<code>{"false"}</code>{"."}</li>
                    <li><b>{"header: "}</b>{"header content of the card. Default "}<code>{"None"}</code>{"."}</li>
                    <li><b>{"header_size: "}</b>{"the size of the header card based in Flexbox. Default "}<code>{"4"}</code>{"."}</li>
                    <li><b>{"body: "}</b>{"body content of the card. Default "}<code>{"None"}</code>{"."}</li>
                    <li><b>{"body_size: "}</b>{"the size of the body card based in Flexbox. Default "}<code>{"6"}</code>{"."}</li>
                    <li><b>{"footer: "}</b>{"footer content of the card. Default "}<code>{"None"}</code>{"."}</li>
                    <li><b>{"footer_size: "}</b>{"the size of the footer card based in Flexbox. Default "}<code>{"2"}</code>{"."}</li>
                    <li><b>{"single_content"}</b>{"Without split in parts, only a single content. Default"}<code>{"None"}</code>{"."}</li>
                    <li><b>{"interaction_effect: "}</b>{"if hove, focus, active effects are enable. Default "}<code>{"true"}</code>{"."}</li>
                    <li><b>{"key: "}</b>{"general property to add keys."}</li>
                    <li><b>{"code_ref: "}</b>{"general property to get the ref of the component."}</li>
                    <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                    <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>
                    <li><b>{"styles: "}</b>{"use stylist-rs to write styles in the component. Example: "}<code>{"css!(\"background-color: #918d94;\")"}</code></li>
                </ul>

                <h2>{"Visual examples"}</h2>
                <h3>{"Sizes"}</h3>
                <Container direction=Direction::Row wrap=Wrap::Wrap>
                    {get_sizes()}
                </Container>
                <Container direction=Direction::Row wrap=Wrap::Wrap>
                    <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                        <h3>{"No interaction effect"}</h3>
                        <Card
                            card_style=Style::Outline
                            interaction_effect=false
                            header=Some(html!{<div class="image">
                                {"Image"}
                            </div>})
                            body=Some(html!{
                                <div class="content">{lipsum(10)}</div>
                            })
                            footer=Some(html!{
                                <div>{lipsum(3)}</div>
                            })
                        />
                    </Item>
                    <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                        <h3>{"Custom content"}</h3>
                        <Card
                            card_style=Style::Outline
                            header=Some(html!{<div class="image">
                                {"Image"}
                            </div>})
                            header_size=8
                            body=Some(html!{
                                <div>{lipsum(20)}</div>
                            })
                            body_size=4
                        />
                    </Item>
                    <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                        <h3>{"Single Content"}</h3>
                        <Card
                            card_style=Style::Outline
                            single_content=Some(html!{<div class="image">
                                {"Image"}
                            </div>})
                        />
                    </Item>
                </Container>
                <h3>{"Drag and drop"}</h3>
                    <Container direction=Direction::Row wrap=Wrap::Wrap>
                        <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                        <div ondrop=self.link.callback(Msg::Dropped)
                            ondragover=self.link.callback(Msg::DraggedOver)
                            class="box">
                            {"Box 1"}
                            <Card
                                class_name="card-example-draggable"
                                card_style=Style::Light
                                card_palette=Palette::Success
                                draggable=true
                                ondragstart_signal=self.link.callback(Msg::Dragged)
                                id="card-draggable"
                                header=Some(html!{<div class="image">
                                    {"Image"}
                                </div>})
                                body=Some(html!{
                                    <div class="content">{lipsum(10)}</div>
                                })
                                footer=Some(html!{
                                    <div>{lipsum(3)}</div>
                                })
                            />
                        </div>
                        </Item>
                        <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                            <div ondrop=self.link.callback(Msg::Dropped)
                                ondragover=self.link.callback(Msg::DraggedOver)
                            class="box">
                                {"Box 2"}
                            </div>
                        </Item>
                    </Container>
                <h3>{"Styles"}</h3>
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
                        card_size=size.clone()
                        header=Some(html!{<div class="image">
                            {get_size(size.clone()).to_pascal_case()}
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
        .map(|style| {
            html! {
                <>
                    <Item layouts=vec!(ItemLayout::ItXs(12))>
                        <h4>{get_style(style.clone()).to_pascal_case()}</h4>
                    </Item>
                    {get_types(style)}
                </>
            }
        })
        .collect::<Html>()
}

fn get_types(style: Style) -> Html {
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
        .map(|card_palette| {
            html! {
                <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                    <Card
                        card_style=style.clone()
                        card_palette=card_palette.clone()
                        header=Some(html!{<div class="image">
                            {get_palette(card_palette.clone()).to_pascal_case()}
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
