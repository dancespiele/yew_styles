use crate::styles::{get_size, Size};
use yew::prelude::*;
use yew::{utils, App};
use yew_assets::controller_assets::{ControllerAssets, ControllerIcon};
use yew_assets::object_assets::{ObjectAssets, ObjectIcon};

pub struct Carousel {
    link: ComponentLink<Self>,
    props: Props,
    images_total: u32,
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
    pub children: Children,
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
        Self {
            link,
            props,
            images_total: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PrevClicked(mouse_event) => {
                self.props.prev_signal.emit(mouse_event);
            }
            Msg::NextClicked(mouse_event) => {
                self.props.next_signal.emit(mouse_event);
            }
            Msg::DotClicked(mouse_event) => {
                self.props.dot_signal.emit(mouse_event);
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            return true;
        }

        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.images_total = utils::document()
                .get_elements_by_class_name("carousel-image")
                .length();
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class=("carousel-container", get_size(self.props.carousel_size.clone()))>
                {get_controls(self.props.controls, self.link.clone())}
                <div class="carousel-images">{self.props.children.clone()}</div>
                {get_dots_controls(self.props.dot_controls, self.images_total, self.link.clone())}
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

fn get_dots_controls(dot_controls: bool, images_total: u32, link: ComponentLink<Carousel>) -> Html {
    if dot_controls && images_total > 0 {
        let mut dots = vec![];
        for _i in 0..images_total {
            dots.push(html! {
                <div
                    class="carousel-dot-container"
                    onclick=link.callback(|e| Msg::DotClicked(e))
                >
                    <ObjectAssets icon=ObjectIcon::Circle class_name="carousel-dot"/>
                </div>
            });
        }

        dots.into_iter().collect::<Html>()
    } else {
        html! {}
    }
}
