use crate::container::{Container, Direction, Wrap};
use crate::palette::{BuildPalette, Palettes};
use crate::utils::{create_style, DefaultCallback};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Fixed {
    Top,
    Bottom,
}

pub enum Msg {
    Selected,
}

struct Navbar {
    link: ComponentLink<Self>,
    props: Props,
}

struct NavbarModel;

#[derive(Clone, Properties)]
struct Props {
    #[prop_or(Palettes::Standard)]
    pub navbar_type: Palettes,
    #[prop_or_default]
    pub navbar_styles: String,
    #[prop_or(Fixed::Top)]
    pub fixed: Fixed,
    #[prop_or(DefaultCallback {
        callback: Callback::noop(),
    })]
    pub onsignal: DefaultCallback<Callback<()>>,
    pub children: Children,
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Navbar { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Selected => {
                self.props.onsignal.callback.emit(());
            }
        };

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="navbar">
                <Container direction=Direction::Row, wrap=Wrap::Wrap>
                    {self.props.children.render()}
                </Container>
            </div>
        }
    }
}

impl NavbarModel {
    fn set_fixed(fixed: Fixed) {
        create_style(
            String::from("position"),
            String::from("fixed"),
            String::from("navbar"),
        );
        create_style(
            if fixed == Fixed::Top {
                String::from("top")
            } else {
                String::from("bottom")
            },
            String::from("0"),
            String::from("navbar"),
        );
    }
}
