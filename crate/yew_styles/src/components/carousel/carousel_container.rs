use crate::styles::{get_size, Size};
use yew::prelude::*;
use yew_assets::controller_assets::{ControllerAssets, ControllerIcon};

pub struct Carousel {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties, PartialEq)]
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
    #[prop_or_default]
    pub dot_index: u32,
    #[prop_or_default]
    pub class_name: String,
    #[prop_or_default]
    pub id: String,
    pub children: Children,
}

pub enum Msg {
    PrevClicked(MouseEvent),
    NextClicked(MouseEvent),
}

impl Component for Carousel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PrevClicked(mouse_event) => {
                self.props.prev_signal.emit(mouse_event);
            }
            Msg::NextClicked(mouse_event) => {
                self.props.next_signal.emit(mouse_event);
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            return true;
        }

        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=("carousel-container", get_size(self.props.carousel_size.clone()))>
                {self.props.children.clone()}
                {get_controls(self.props.controls, self.link.clone())}
            </div>
        }
    }
}

fn get_controls(controls: bool, link: ComponentLink<Carousel>) -> Html {
    if controls {
        html! {
            <div class="carousel-control">
                <div
                    class="carousel-control-left-container"
                    onclick=link.callback(|e| Msg::PrevClicked(e))>
                    <ControllerAssets
                        class_name="carousel-control-left"
                        icon=ControllerIcon::ChevronLeft
                    />
                </div>
                <div
                    class="carousel-control-right-container"
                    onclick=link.callback(|e| Msg::NextClicked(e))
                >
                    <ControllerAssets class_name="carousel-control-right" icon=ControllerIcon::ChevronRight/>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}
