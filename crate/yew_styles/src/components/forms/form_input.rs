use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

pub struct FormInput {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub onsignal: Callback<()>,
    pub value: String,
    #[prop_or_default]
    pub class_name: String,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub input_id: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub error_state: bool,
    #[prop_or_default]
    pub error_message: String,
}

#[derive(Debug)]
pub enum Msg {
    Input,
}

impl Component for FormInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input => {
                self.props.onsignal.emit(());
            }
        };

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class=format!("form-input {}", self.props.class_name) id=self.props.id>
                {get_label(self.props.label.clone())}
                <input
                    id=self.props.input_id
                    class=if self.props.error_state { "error" } else { "" }
                    oninput=self.link.callback(|_| Msg::Input)
                    value=self.props.value
                />
                {get_error_message(self.props.error_state, self.props.error_message.clone())}
            </div>
        }
    }
}

fn get_label(label: String) -> Html {
    if !label.is_empty() {
        html! {
            <label>{label}</label>
        }
    } else {
        html! {}
    }
}

fn get_error_message(error_state: bool, error_message: String) -> Html {
    if error_state {
        html! {<span class="form-input-error">{error_message}</span>}
    } else {
        html! {}
    }
}
