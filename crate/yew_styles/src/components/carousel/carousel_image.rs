use yew::prelude::*;

pub struct CarouselImage {
    props: Props,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    /// Url image path
    pub img_src: String,
    #[prop_or(false)]
    /// Show the image if it is active
    pub active: bool,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    /// General property to get the ref of the component
    #[prop_or_default]
    pub code_ref: NodeRef,
    /// General property to add keys
    #[prop_or_default]
    pub key: String,
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
            <div class=("carousel-image carousel-fade", if self.props.active {
                "active"
            } else {
                ""
            })
                ref=self.props.code_ref.clone()
                id=self.props.id
            >
                <img src=self.props.img_src/>
            </div>
        }
    }
}
