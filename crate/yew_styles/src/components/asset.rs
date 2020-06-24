use yew::prelude::*;

/// List of icons
#[derive(Clone)]
pub enum Icon {
    Menu,
    Plus,
    Edit,
    X,
}

pub enum Msg {}

/// # Asset
///
/// Add a svg Icon
pub struct Asset {
    pub props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    /// List of Icons
    pub icon: Icon,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
}

impl Component for Asset {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Asset { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        get_icon(
            self.props.icon.clone(),
            self.props.class_name.clone(),
            self.props.id.clone(),
        )
    }
}

fn get_icon(icon: Icon, class_name: String, id: String) -> Html {
    match icon {
        Icon::Menu => get_menu(class_name, id),
        Icon::Edit => get_edit(class_name, id),
        Icon::Plus => get_plus(class_name, id),
        Icon::X => get_x(class_name, id),
    }
}

fn get_menu(class_name: String, id: String) -> Html {
    html! {
        <svg
            class=class_name
            id=id
            xmlns="http://www.w3.org/2000/svg"
            width="30" height="30" viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round">
            <line x1="3" y1="12" x2="21" y2="12">
            </line>
            <line x1="3" y1="6" x2="21" y2="6">
            </line>
            <line x1="3" y1="18" x2="21" y2="18">
            </line>
        </svg>
    }
}

fn get_edit(class_name: String, id: String) -> Html {
    html! {
        <svg
            id=id
            class=class_name
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round">
                <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7">
                </path>
                <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z">
                </path>
        </svg>
    }
}

fn get_plus(class_name: String, id: String) -> Html {
    html! {
        <svg
            id=id
            class=class_name
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round">
                <line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12">
                </line>
        </svg>
    }
}

fn get_x(class_name: String, id: String) -> Html {
    html! {
        <svg
            class=class_name
            id=id
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none" stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
    }
}
