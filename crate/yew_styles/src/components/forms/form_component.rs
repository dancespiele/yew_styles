use yew::prelude::*;

pub struct Form {
    form_ref: NodeRef,
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    // get all the values from the form
    pub on_submmit: Callback<()>,
    pub children: Children,
}

pub enum Msg {
    Submited,
}

impl Component for Form {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Form {
            form_ref: NodeRef::default(),
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {}
    }
}
