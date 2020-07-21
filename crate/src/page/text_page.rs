use super::highlighters::{get_alert_text, get_paragraph_text, get_plain_text, get_tag_text};
use lipsum::lipsum;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;
use yew::utils;
use yew_prism::Prism;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::{Palette, Size, Style};
use yew_styles::text::{Text, TextType};

pub struct TextPage {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Dragged(DragEvent),
    DraggedOver(DragEvent),
    Dropped(DragEvent),
}

impl Component for TextPage {
    type Properties = ();
    type Message = Msg;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TextPage { link }
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
                    .set_data("application/text-component", &target_id)
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
                    .get_data("application/text-component")
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
                <h1>{"Text Component"}</h1>
                <h2>{"Features required"}</h2>
                <span><code>{"text"}</code></span>

                <h2>{"Code example"}</h2>
                <h3>{"Plain text"}</h3>
                <Prism
                    code=get_plain_text()
                    language="rust"
                />
                <h3>{"Paragraph text"}</h3>
                <Prism
                    code=get_paragraph_text()
                    language="rust"
                />
                <h3>{"Alert text"}</h3>
                <Prism
                    code=get_alert_text()
                    language="rust"
                />
                <h3>{"Tag text"}</h3>
                <Prism
                    code=get_tag_text()
                    language="rust"
                />

                <h2>{"Properties"}</h2>
                <ul>
                    <li><b>{"text: "}</b>{"text to show. Required."}</li>
                    <li><b>{"input_type: "}</b>{"the text type. Options included in "}<code>{"TextType"}</code>{". Required."}</li>
                    <li><b>{"text_palette: "}</b>{"type text purpose style. Options included in "}<code>{"Pallete"}</code>{". Default "}<code>{"Standard"}</code>{". Only alert and tag types"}</li>
                    <li><b>{"text_size: "}</b>{"three diffent text standard sizes. Options included in "}<code>{"Size"}</code>{". Default "}<code>{"Medium"}</code>{". Only alert and tag types"}</li>
                    <li><b>{"text_style: "}</b>{"text styles. Options included in "}<code>{"Style"}</code>{". Default "}<code>{"Regular"}</code>{". Only alert and tag types"}</li>
                    <li><b>{"removable: "}</b>{"if the tag can be deleted. Default "}<code>{"false"}</code>{". Only for tag text type."}</li>
                    <li><b>{"onclick_signal: "}</b>{"click event for text. Only for tag type."}</li>
                    <li><b>{"ondelete_signal: "}</b>{"click event for deleting tag text. Only for tag type with removable set to true."}</li>
                    <li><b>{"ondrag_signal: "}</b>{"a dragged item (element or text selection) is dragged. Only for tag text type."}</li>
                    <li><b>{"ondragend_signal: "}</b>{"a drag operation ends. Only for tag text type."}</li>
                    <li><b>{"ondragenter_signal: "}</b>{"a dragged item enters a valid drop target. Only for tag text type."}</li>
                    <li><b>{"ondragexit_signal: "}</b>{"an element is no longer the drag operation's immediate selection target. Only for tag text type."}</li>
                    <li><b>{"ondragleave_signal: "}</b>{"a dragged item leaves a valid drop target. Only for tag text type."}</li>
                    <li><b>{"ondragover_signal: "}</b>{"a dragged item is being dragged over a valid drop target, every few hundred milliseconds. Only for tag text type."}</li>
                    <li><b>{"ondragstart_signal: "}</b>{"the user starts dragging an item. Only for tag text type."}</li>
                    <li><b>{"ondrop_signal: "}</b>{"an item is dropped on a valid drop target. Only for tag text type."}</li>
                    <li><b>{"draggable: "}</b>{"if the item is draggable. Default "}<code>{"false"}</code>{". Only for tag text type."}</li>
                </ul>

                <h2>{"Visual examples"}</h2>

                <h3>{"Plain text"}</h3>
                <Text
                    text_type=TextType::Plain
                    text_size=Size::Medium
                    text=lipsum(8)
                />

                <h3>{"Paragraph"}</h3>
                <Text
                    text_type=TextType::Paragraph
                    text_size=Size::Small
                    text=lipsum(200)
                />
                <h3>{"Alert text"}</h3>
                <Container wrap = Wrap::Wrap direction = Direction::Row>
                    {get_text(TextType::Alert, 10, 10)}
                </Container>
                <h3>{"Tag text"}</h3>
                <Container wrap = Wrap::Wrap direction = Direction::Row>
                    {get_text(TextType::Tag, 1, 1)}
                </Container>
                <h3>{"Tag text removable"}</h3>
                <Text
                    text_type=TextType::Tag
                    text_size=Size::Medium
                    text=lipsum(1).replace(".", "")
                    text_style=Style::Outline
                    text_palette=Palette::Secondary
                    removable=true
                />
                <h3>{"Tag draggable with interaction effect"}</h3>
                <Container wrap = Wrap::Wrap direction = Direction::Row>
                    <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                        <div
                            ondrop=self.link.callback(|e| Msg::Dropped(e))
                            ondragover=self.link.callback(|e| Msg::DraggedOver(e))
                            class="tag-box"
                        >
                            {get_draggable_tags(self.link.clone())}
                        </div>
                    </Item>
                    <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                        <div ondrop=self.link.callback(Msg::Dropped)
                            ondragover=self.link.callback(Msg::DraggedOver)
                            class="tag-box">
                        </div>
                    </Item>
                </Container>
            </>
        }
    }
}

fn get_text(text_type: TextType, words: usize, layout_size: i8) -> Html {
    let styles = vec![Style::Regular, Style::Outline, Style::Light];
    let palette = vec![Palette::Success, Palette::Warning, Palette::Danger];

    styles
        .into_iter()
        .map(|style| {
            palette
                .clone()
                .into_iter()
                .map(|item_palette| {
                    html! {
                        <Item layouts=vec![ItemLayout::ItXs(layout_size)] class_name="alert-example">
                            <Text
                                text_type=text_type.clone()
                                text_size=Size::Medium
                                text=lipsum(words).replace(".", "")
                                text_style=style.clone()
                                text_palette=item_palette
                            />
                        </Item>
                    }
                })
                .collect::<Html>()
        })
        .collect::<Html>()
}

fn get_draggable_tags(link: ComponentLink<TextPage>) -> Html {
    let styles = vec![Style::Regular, Style::Outline, Style::Light];
    let palette = vec![Palette::Success, Palette::Warning, Palette::Danger];
    let mut index = 0;

    styles
        .into_iter()
        .map(|style| {
            palette
                .clone()
                .into_iter()
                .map(|item_palette| {
                    let text_view = html! {
                        <Text
                            class_name="draggable-tag"
                            id=format!("tag-{}", index)
                            draggable=true
                            ondragstart_signal=link.callback(Msg::Dragged)
                            text_type=TextType::Tag
                            text_size=Size::Medium
                            text=lipsum(1).replace(".", "")
                            text_style=style.clone()
                            text_palette=item_palette
                            interaction_effect= true
                        />
                    };

                    index += 1;

                    text_view
                })
                .collect::<Html>()
        })
        .collect::<Html>()
}
