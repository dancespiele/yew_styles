use crate::styles::{get_size, Size};
use yew::prelude::*;

pub struct Carousel {
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

impl Component for Carousel {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
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
            <div class=("carousel-container", get_size(self.props.carousel_size.clone()), self.props.class_name.clone())>
                {self.props.children.clone()}
            </div>
        }
    }
}
