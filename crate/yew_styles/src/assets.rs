use yew::prelude::*;

/// List of icons
#[derive(Clone)]
pub enum Icon {
    Menu,
    Plus,
    Edit,
}

pub enum Msg {}

/// # Assets
///
/// Add a svg Icon
pub struct Assets {
    pub props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    /// List of Icons
    pub asset: Icon,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
}

impl Component for Assets {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Assets { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        get_icon(
            self.props.asset.clone(),
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
    }
}

fn get_menu(class_name: String, id: String) -> Html {
    html! {
        <svg
            class=class_name
            id=id
            xmlns="http://www.w3.org/2000/svg"
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
