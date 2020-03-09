use crate::container::{Container, Direction, Wrap};
use crate::palette::{BuildPalette, Palettes};
use crate::utils::create_style;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Fixed {
    Top,
    Bottom,
}

pub enum Msg {}

pub struct Navbar {
    pub props: NavbarProps,
}

struct NavbarModel;

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or(Palettes::Standard)]
    pub navbar_type: Palettes,
    #[prop_or_default]
    pub navbar_styles: String,
    #[prop_or(Fixed::Top)]
    pub fixed: Fixed,
    pub children: Children,
}

#[derive(Clone)]
pub struct NavbarProps {
    pub navbar_type: String,
    pub navbar_styles: String,
    pub fixed: Fixed,
    pub children: Children,
}

impl From<Props> for NavbarProps {
    fn from(props: Props) -> Self {
        NavbarProps {
            navbar_type: BuildPalette::new(props.navbar_type),
            navbar_styles: props.navbar_styles,
            fixed: props.fixed,
            children: props.children,
        }
    }
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Navbar {
            props: NavbarProps::from(props),
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        NavbarModel.init(self.props.clone());

        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        NavbarModel.init(self.props.clone());
        self.props = NavbarProps::from(props);
        true
    }

    fn view(&self) -> Html {
        html! {
            <div
                class=format!("navbar {} {}", self.props.navbar_type, self.props.navbar_styles)
            >
                <Container direction=Direction::Row, wrap=Wrap::Wrap>
                    {self.props.children.render()}
                </Container>
            </div>
        }
    }
}

impl NavbarModel {
    fn init(self, props: NavbarProps) {
        self.set_fixed(props.fixed);
    }

    fn set_fixed(self, fixed: Fixed) {
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
