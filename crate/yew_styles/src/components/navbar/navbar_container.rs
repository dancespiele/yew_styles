use crate::container::{Container, Direction, JustifyContent, Mode, Wrap};
use yew::prelude::*;

use uuid::Uuid;

pub enum Msg {
    Clicked,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or(JustifyContent::FlexStart(Mode::NoMode))]
    pub justify_content: JustifyContent,
    pub children: Children,
}

pub struct NavbarContainer {
    pub props: Props,
}

impl Component for NavbarContainer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        NavbarContainer { props: props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Container
                class_name="navbar-container"
                direction=Direction::Row
                wrap=Wrap::Wrap
                justify_content=self.props.justify_content.clone()
                name=format!("{}", Uuid::new_v4())>
                    {self.props.children.render()}
            </Container>
        }
    }
}
