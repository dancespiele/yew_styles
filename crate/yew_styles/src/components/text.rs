use crate::styles::{get_pallete, get_size, get_style, Palette, Size, Style};
use yew::prelude::*;

pub struct Text {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone)]
pub enum TextType {
    Plain,
    Parragraph,
    Alert,
    Tag,
}

#[derive(Clone, Properties)]
pub struct Props {
    /// Text to show
    pub text: String,
    /// How is showing the text
    pub text_content_type: TextType,
    /// if hove, focus, active effects are enable, only for tag type
    #[prop_or(true)]
    pub interaction_effect: bool,
    /// A dragged item (element or text selection) is dragged, only for tag type
    #[prop_or(Callback::noop())]
    pub ondrag_signal: Callback<DragEvent>,
    /// A drag operation ends, only for tag type
    #[prop_or(Callback::noop())]
    pub ondragend_signal: Callback<DragEvent>,
    /// A dragged item enters a valid drop target, only for tag type
    #[prop_or(Callback::noop())]
    pub ondragenter_signal: Callback<DragEvent>,
    /// An element is no longer the drag operation's immediate selection target, only for tag type
    #[prop_or(Callback::noop())]
    pub ondragexit_signal: Callback<DragEvent>,
    /// A dragged item leaves a valid drop target, only for tag type
    #[prop_or(Callback::noop())]
    pub ondragleave_signal: Callback<DragEvent>,
    /// A dragged item is being dragged over a valid drop target
    /// Every few hundred milliseconds, only for tag type
    #[prop_or(Callback::noop())]
    pub ondragover_signal: Callback<DragEvent>,
    /// The user starts dragging an item, only for tag type
    #[prop_or(Callback::noop())]
    pub ondragstart_signal: Callback<DragEvent>,
    /// An item is dropped on a valid drop target, only for tag type
    #[prop_or(Callback::noop())]
    pub ondrop_signal: Callback<DragEvent>,
    /// Click event only for text tag type
    #[prop_or(Callback::noop())]
    pub onclick_signal: Callback<MouseEvent>,
    /// Click event only for text tag type with removable in true
    #[prop_or(Callback::noop())]
    pub ondelete_signal: Callback<MouseEvent>,
    /// If the item is draggable, only for tag type
    #[prop_or(false)]
    pub draggable: bool,
    /// If the tag can be seleted
    #[prop_or(false)]
    pub removable: bool,
    /// Type text purpose style, only for tag type
    #[prop_or(Palette::Standard)]
    pub text_type: Palette,
    /// Text styles, only for tag type
    #[prop_or(Style::Regular)]
    pub text_style: Style,
    /// Three diffent text standard sizes
    #[prop_or(Size::Medium)]
    pub text_size: Size,
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

impl Component for Text {
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
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {}
    }
}
