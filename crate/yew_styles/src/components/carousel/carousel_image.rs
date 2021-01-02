use yew::prelude::*;

pub struct CarouselImage {
    props: Props,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub img_src: String,
    #[prop_or_default]
    pub class_name: String,
    #[prop_or_default]
    pub id: String,
}

impl Component for CarouselImage {
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
            <div class="carousel-image carousel-fade">
                <img src=self.props.img_src/>
            </div>
        }
    }
}
