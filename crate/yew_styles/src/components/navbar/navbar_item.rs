use yew::prelude::*;

pub enum Msg {
    Clicked,
}

pub struct NavbarItem {
    link: ComponentLink<Self>,
    props: Props,
}

pub struct NavbarItemModel;

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class_name: String,
    #[prop_or(Callback::noop())]
    pub onsignal: Callback<()>,
    pub children: Children,
}

impl Component for NavbarItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NavbarItem { link, props }
    }

    fn mounted(&mut self) -> ShouldRender {
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.props.onsignal.emit(());
            }
        }

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;

        true
    }

    fn view(&self) -> Html {
        html! {
            <div
                class=format!("navbar-item {}", self.props.class_name)
                onclick=self.link.callback(|_| Msg::Clicked)
            >
                {self.props.children.render()}
            </div>
        }
    }
}
