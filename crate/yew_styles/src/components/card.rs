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
    pub ondrag_signal: Callback<()>,
    #[prop_or(Callback::noop())]
    pub ondragend_signal: Callback<()>,
    #[prop_or(Callback::noop())]
    pub ondragenter_signal: Callback<()>,
    #[prop_or(Callback::noop())]
    pub ondragexit_signal: Callback<()>,
    #[prop_or(Callback::noop())]
    pub ondragleave_signal: Callback<()>,
    #[prop_or(Callback::noop())]
    pub ondragover_signal: Callback<()>,
    #[prop_or(Callback::noop())]
    pub ondragstart_signal: Callback<()>,
    #[prop_or(Callback::noop())]
    pub ondrop_signal: Callback<()>,
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
    Draged,
    DragedEnd,
    DragedEnter,
    DragedExit,
    DragedLeave,
    DragedOver,
    DragedStart,
    Dropped,
}

impl Component for Card {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Draged => {
                self.props.ondrag_signal.emit(());
            }
            Msg::DragedEnd => {
                self.props.ondragend_signal.emit(());
            }
            Msg::DragedEnter => {
                self.props.ondragenter_signal.emit(());
            }
            Msg::DragedExit => {
                self.props.ondragexit_signal.emit(());
            }
            Msg::DragedLeave => {
                self.props.ondragleave_signal.emit(());
            }
            Msg::DragedOver => {
                self.props.ondragover_signal.emit(());
            }
            Msg::DragedStart => {
                self.props.ondragstart_signal.emit(());
            }
            Msg::Dropped => {
                self.props.ondrop_signal.emit(());
            }
        };

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
