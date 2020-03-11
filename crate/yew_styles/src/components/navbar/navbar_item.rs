use crate::utils::create_style;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum AligSelf {
    Left,
    Center,
    Right,
}

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
    #[prop_or(AligSelf::Left)]
    pub side: AligSelf,
    #[prop_or_default]
    pub class_name: String,
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
        NavbarItemModel.init(self.props.clone());

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
        NavbarItemModel.init(self.props.clone());
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

impl NavbarItemModel {
    fn init(self, props: Props) {
        self.set_side(props.side);
    }

    fn set_side(self, align_self: AligSelf) {
        let value = match align_self {
            AligSelf::Left => String::from("flex-start"),
            AligSelf::Center => String::from("center"),
            AligSelf::Right => String::from("flex-end"),
        };

        create_style(
            String::from("align-self"),
            value,
            String::from("navbar-item"),
        );
    }
}
