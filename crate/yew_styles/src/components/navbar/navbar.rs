use super::navbar_container::NavbarContainer;
use super::navbar_item::NavbarItem;
use crate::container::{Direction, JustifyContent, Mode};
use crate::styles::{get_pallete, get_style, Palette, Style};
use crate::utils::create_style;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Fixed {
    None,
    Top,
    Bottom,
}

pub enum Msg {
    TroggleMenu,
}

pub struct Navbar {
    pub link: ComponentLink<Self>,
    pub props: NavbarProps,
    pub display_menu: bool,
}

struct NavbarModel;

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or(Palette::Standard)]
    pub navbar_type: Palette,
    #[prop_or(Style::Regular)]
    pub navbar_style: Style,
    #[prop_or_default]
    pub class_name: String,
    #[prop_or(Fixed::Top)]
    pub fixed: Fixed,
    #[prop_or_default]
    pub branch: Html,
    pub children: Children,
}

#[derive(Clone)]
pub struct NavbarProps {
    pub navbar_type: String,
    pub navbar_style: String,
    pub class_name: String,
    pub fixed: Fixed,
    pub branch: Html,
    pub children: Children,
}

impl From<Props> for NavbarProps {
    fn from(props: Props) -> Self {
        NavbarProps {
            navbar_type: get_pallete(props.navbar_type),
            navbar_style: get_style(props.navbar_style),
            class_name: props.class_name,
            fixed: props.fixed,
            branch: props.branch,
            children: props.children,
        }
    }
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Navbar {
            link,
            props: NavbarProps::from(props),
            display_menu: false,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        NavbarModel.init(self.props.clone());

        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TroggleMenu => {
                self.display_menu = !self.display_menu;
            }
        };

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        NavbarModel.init(self.props.clone());
        self.props = NavbarProps::from(props);
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div
                    class=format!("navbar-mobile {} {} {}", self.props.navbar_style, self.props.navbar_type, self.props.class_name)
                >
                    <div class="navbar-dropdown">
                        <NavbarContainer justify_content=JustifyContent::Start(Mode::NoMode)
                        direction=Direction::Row
                        class_name="navbar-container-mobile">
                        {get_branch(self.props.branch.clone())}
                        </NavbarContainer>
                        <NavbarContainer justify_content=JustifyContent::End(Mode::NoMode)
                            direction=Direction::Row
                            class_name="navbar-container-mobile">
                            <NavbarItem
                                onsignal=self.link.callback(move |_| Msg::TroggleMenu)
                            >
                                <div class="navbar-menu">
                                    <svg xmlns="http://www.w3.org/2000/svg"
                                        width="10mm" height="10mm"
                                        viewBox="0 0 512 512">
                                        <path id="Selección"
                                            stroke="black" stroke-width="1"
                                            d="M 97.00,64.12
                                                C 79.03,68.69 64.23,84.02 64.00,103.00
                                                    63.84,116.87 65.18,126.19 75.30,136.83
                                                    84.09,146.07 96.54,149.98 109.00,150.00
                                                    109.00,150.00 403.00,150.00 403.00,150.00
                                                    421.29,149.97 436.72,142.10 444.68,125.00
                                                    447.01,119.98 447.93,115.50 448.00,110.00
                                                    448.05,105.17 448.30,99.65 447.08,95.00
                                                    442.20,76.40 426.08,64.03 407.00,64.12
                                                    407.00,64.12 201.00,64.12 201.00,64.12
                                                    201.00,64.12 133.00,64.12 133.00,64.12
                                                    133.00,64.12 97.00,64.12 97.00,64.12 Z
                                                M 99.00,213.12
                                                C 88.99,215.69 81.18,219.22 74.18,227.01
                                                    60.29,242.50 60.29,269.50 74.18,284.99
                                                    82.99,294.80 94.06,298.98 107.00,299.00
                                                    107.00,299.00 405.00,299.00 405.00,299.00
                                                    418.47,298.98 431.05,293.87 439.47,283.00
                                                    455.95,261.72 448.50,228.23 424.00,216.80
                                                    417.73,213.87 411.83,213.01 405.00,213.12
                                                    405.00,213.12 202.00,213.12 202.00,213.12
                                                    202.00,213.12 135.00,213.12 135.00,213.12
                                                    135.00,213.12 99.00,213.12 99.00,213.12 Z
                                                M 101.00,362.11
                                                C 85.09,365.38 73.47,372.65 66.88,388.00
                                                    64.82,392.78 64.06,396.83 64.00,402.00
                                                    63.95,406.83 63.70,412.35 64.92,417.00
                                                    69.80,435.60 85.92,447.97 105.00,448.00
                                                    105.00,448.00 407.00,448.00 407.00,448.00
                                                    428.40,447.97 447.74,430.94 448.00,409.00
                                                    448.10,400.54 448.38,394.95 444.68,387.00
                                                    436.72,369.90 421.29,362.03 403.00,362.11
                                                    403.00,362.11 203.00,362.11 203.00,362.11
                                                    203.00,362.11 137.00,362.11 137.00,362.11
                                                    137.00,362.11 101.00,362.11 101.00,362.11 Z" />
                                    </svg>
                                </div>
                            </NavbarItem>
                        </NavbarContainer>
                    </div>
                    {if self.display_menu {
                        self.props.children.render()
                    } else {
                        html!{}
                    }}
                </div>

                <div
                    class=format!("navbar {} {} {}", self.props.navbar_style, self.props.navbar_type, self.props.class_name)
                >
                    {self.props.children.render()}
                </div>
            </>
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
            if fixed == Fixed::None {
                String::from("inherit")
            } else {
                String::from("fixed")
            },
            String::from("navbar"),
        );

        if fixed != Fixed::None {
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
}

fn get_branch(branch: Html) -> Html {
    if branch != html! {} {
        html! {
            <div class="branch">
                {branch}
            </div>
        }
    } else {
        html! {}
    }
}
