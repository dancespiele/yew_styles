use yew::prelude::*;

pub struct NavbarDropdown {
    props: Props,
    show: bool,
    link: ComponentLink<Self>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    /// clickeable content to show the dropdown. Required
    pub main_content: Html,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    #[prop_or(false)]
    pub active: bool,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    pub children: Children,
}

pub enum Msg {
    ShowDropdown,
    HideDropdown,
}

impl Component for NavbarDropdown {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            show: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowDropdown => {
                self.show = true;
            }
            Msg::HideDropdown => {
                self.show = false;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            return true;
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div
                class=("navbar-dropdown", if self.props.active {
                    "active"
                } else {
                    ""
                }, self.props.class_name.clone())
                id=self.props.id
                onmouseover=self.link.callback(|_| Msg::ShowDropdown)
                onmouseleave=self.link.callback(|_| Msg::HideDropdown)
                onclick=self.link.callback(|_| Msg::HideDropdown)
                >
                <div class="main-content">{self.props.main_content.clone()}</div>
                {get_items(self.show, self.props.children.clone())}
            </div>
        }
    }
}

fn get_items(show: bool, children: Children) -> Html {
    if show {
        html! {
            <ul>
                {children.clone()}
            </ul>
        }
    } else {
        html! {}
    }
}
