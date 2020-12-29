use crate::styles::Size;
use yew::prelude::*;
use yew::{utils, App};

pub struct Carousel {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or(Size::Medium)]
    pub carousel_size: Size,
    #[prop_or(true)]
    pub controls: bool,
    #[prop_or(Callback::noop())]
    pub prev_signal: Callback<MouseEvent>,
    #[prop_or(Callback::noop())]
    pub next_signal: Callback<MouseEvent>,
    #[prop_or(true)]
    pub dot_controls: bool,
    #[prop_or(Callback::noop())]
    pub dot_signal: Callback<MouseEvent>,
}

pub enum Msg {
    PrevClicked(MouseEvent),
    NextClicked(MouseEvent),
    DotClicked(MouseEvent),
}

impl Component for Carousel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {}
    }
}
