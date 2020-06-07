use crate::layouts::{
    container::{Container, Direction, JustifyContent, Mode, Wrap},
    item::{AlignSelf, Item, ItemLayout},
};
use crate::styles::{get_pallete, get_size, get_style, Palette, Size, Style};
use wasm_bindgen_test::*;
use web_sys::window;
use yew::prelude::*;
use yew::{utils, App};

pub struct Card {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or(Callback::noop())]
    pub ondrag_signal: Callback<DragEvent>,
    #[prop_or(Callback::noop())]
    pub ondragend_signal: Callback<DragEvent>,
    #[prop_or(Callback::noop())]
    pub ondragenter_signal: Callback<DragEvent>,
    #[prop_or(Callback::noop())]
    pub ondragexit_signal: Callback<DragEvent>,
    #[prop_or(Callback::noop())]
    pub ondragleave_signal: Callback<DragEvent>,
    #[prop_or(Callback::noop())]
    pub ondragover_signal: Callback<DragEvent>,
    #[prop_or(Callback::noop())]
    pub ondragstart_signal: Callback<DragEvent>,
    #[prop_or(Callback::noop())]
    pub ondrop_signal: Callback<DragEvent>,
    #[prop_or(false)]
    pub draggable: bool,
    #[prop_or(None)]
    pub header: Option<Html>,
    #[prop_or(None)]
    pub body: Option<Html>,
    #[prop_or(None)]
    pub footer: Option<Html>,
    #[prop_or(None)]
    pub single_content: Option<Html>,
    #[prop_or(Palette::Standard)]
    pub card_type: Palette,
    #[prop_or(Style::Regular)]
    pub card_style: Style,
    #[prop_or(Size::Medium)]
    pub card_size: Size,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
}

pub enum Msg {
    Draged(DragEvent),
    DragedEnd(DragEvent),
    DragedEnter(DragEvent),
    DragedExit(DragEvent),
    DragedLeave(DragEvent),
    DragedOver(DragEvent),
    DragedStart(DragEvent),
    Dropped(DragEvent),
}

impl Component for Card {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Draged(drag_event) => {
                self.props.ondrag_signal.emit(drag_event);
            }
            Msg::DragedEnd(drag_event) => {
                self.props.ondragend_signal.emit(drag_event);
            }
            Msg::DragedEnter(drag_event) => {
                self.props.ondragenter_signal.emit(drag_event);
            }
            Msg::DragedExit(drag_event) => {
                self.props.ondragexit_signal.emit(drag_event);
            }
            Msg::DragedLeave(drag_event) => {
                self.props.ondragleave_signal.emit(drag_event);
            }
            Msg::DragedOver(drag_event) => {
                self.props.ondragover_signal.emit(drag_event);
            }
            Msg::DragedStart(drag_event) => {
                self.props.ondragstart_signal.emit(drag_event);
            }
            Msg::Dropped(drag_event) => {
                self.props.ondrop_signal.emit(drag_event);
            }
        };

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div
                id=self.props.id
                class=format!(
                    "card {} {} {} {}",
                    get_pallete(self.props.card_type.clone()),
                    get_size(self.props.card_size.clone()),
                    get_style(self.props.card_style.clone()),
                    self.props.class_name.clone(),
                )
                draggable = self.props.draggable
                ondrag = self.link.callback(|e| Msg::Draged(e))
                ondragend = self.link.callback(|e| Msg::Draged(e))
                ondragenter = self.link.callback(|e| Msg::Draged(e))
                ondragexit = self.link.callback(|e| Msg::Draged(e))
                ondragleave = self.link.callback(|e| Msg::Draged(e))
                ondragover = self.link.callback(|e| Msg::Draged(e))
                ondragstart = self.link.callback(|e| Msg::Draged(e))
                ondrop = self.link.callback(|e| Msg::Draged(e))
            >
                {get_content(
                    self.props.single_content.clone(),
                    self.props.header.clone(), self.props.body.clone(),
                    self.props.footer.clone()
                )}
            </div>
        }
    }
}

fn get_content(
    single_content: Option<Html>,
    header: Option<Html>,
    body: Option<Html>,
    footer: Option<Html>,
) -> Html {
    if let Some(single_content_node) = single_content {
        html! {
            <div class="card-single-content">
                {single_content_node}
            </div>
        }
    } else {
        html! {
            <Container wrap = Wrap::Wrap direction=Direction::Column justify_content=JustifyContent::Center(Mode::NoMode)>
                <Item layouts=vec!(ItemLayout::ItXs(12)) align_self=AlignSelf::FlexStart>
                    {get_content_part(header, "card-header")}
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12)) align_self=AlignSelf::Center>
                    {get_content_part(body, "card-body")}
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12)) align_self=AlignSelf::FlexEnd>
                    {get_content_part(footer, "card-footer")}
                </Item>
            </Container>
        }
    }
}

fn get_content_part(content: Option<Html>, class_content: &str) -> Html {
    if let Some(content_node) = content {
        html! {
            <div class=class_content>
                {content_node}
            </div>
        }
    } else {
        html! {}
    }
}
