use yew::prelude::*;
use yew_assets::controller_assets::{ControllerAssets, ControllerIcon};

pub struct CarouselControls {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub prev_signal: Callback<MouseEvent>,
    pub next_signal: Callback<MouseEvent>,
    #[prop_or_default]
    pub class_name: String,
    #[prop_or_default]
    pub id: String,
}

pub enum Msg {
    PrevClicked(MouseEvent),
    NextClicked(MouseEvent),
}

impl Component for CarouselControls {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
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
            <div class="carousel-control">
                <div
                    class="carousel-control-left-container"
                    onclick=self.link.callback(|e| Msg::PrevClicked(e))>
                    <ControllerAssets
                        size=("50".to_string(),"50".to_string())
                        class_name="carousel-control-left"
                        icon=ControllerIcon::ChevronLeft
                    />
                </div>
                <div
                    class="carousel-control-right-container"
                    onclick=self.link.callback(|e| Msg::NextClicked(e))
                >
                    <ControllerAssets
                        size=("50".to_string(),"50".to_string())
                        class_name="carousel-control-right"
                        icon=ControllerIcon::ChevronRight/>
                </div>
            </div>
        }
    }
}
