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
use yew_styles::styles::{get_pallete, get_size, get_style, Palette, Size, Style};

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

                target_element
                    .append_child(&utils::document().get_element_by_id(&data).unwrap())
                    .unwrap();
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
                <h2>{"Code example"}</h2>

                <Prism
                    code=get_card()
                    language="rust"
                />

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
                        <div ondrop=self.link.callback(|e| Msg::Dropped(e))
                            ondragover=self.link.callback(|e| Msg::DraggedOver(e))
                            class="box">
                            {"Box 1"}
                            <Card
                                card_style=Style::Light
                                card_type=Palette::Success
                                draggable=true
                                ondragstart_signal=self.link.callback(|e| Msg::Dragged(e))
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
                            <div ondrop=self.link.callback(|e| Msg::Dropped(e))
                                ondragover=self.link.callback(|e| Msg::DraggedOver(e))
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
                        card_type=card_type.clone()
                        header=Some(html!{<div class="image">
                            {get_pallete(card_type.clone()).to_pascal_case()}
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
