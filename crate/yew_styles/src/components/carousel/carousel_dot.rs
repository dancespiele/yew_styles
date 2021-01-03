use crate::styles::{get_palette, Palette};
use yew::prelude::*;
use yew_assets::object_assets::{ObjectAssets, ObjectIcon};

pub struct CarouselDot {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub onclick_signal: Callback<MouseEvent>,
    #[prop_or(Palette::Standard)]
    pub carousel_dot_palette: Palette,
    #[prop_or(false)]
    pub active: bool,
    #[prop_or_default]
    pub class_name: String,
    #[prop_or_default]
    pub id: String,
}

pub enum Msg {
    DotClicked(MouseEvent),
}

impl Component for CarouselDot {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DotClicked(mouse_event) => {
                self.props.onclick_signal.emit(mouse_event);
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
            <div
                class=(
                    "carousel-dot",
                    self.props.class_name.clone(),
                    get_palette(self.props.carousel_dot_palette.clone()),
                    if self.props.active {
                        "active"
                    } else {
                        ""
                    },
                    self.props.class_name.clone(),
                )
                id={self.props.id.clone()}
                onclick=self.link.callback(|e| Msg::DotClicked(e))
            >
                <ObjectAssets size=("12".to_string(), "12".to_string()) icon=ObjectIcon::Circle class_name="carousel-dot-assets"/>
            </div>
        }
    }
}
