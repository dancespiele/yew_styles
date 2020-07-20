use lipsum::lipsum;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;
use yew::utils;
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
                <h1>{"Text Component"}</h1>
                <h2>{"Features required"}</h2>
                <span><code>{"text"}</code></span>

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
