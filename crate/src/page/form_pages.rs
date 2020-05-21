use yew::prelude::*;
use yew_styles::forms::form_input::{FormInput, InputType};

pub struct WriteInput {
    pub link: ComponentLink<Self>,
    pub value: String,
}

pub enum MsgWriteInput {
    Input(String),
}

impl Component for WriteInput {
    type Message = MsgWriteInput;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        WriteInput {
            link,
            value: String::from(""),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            MsgWriteInput::Input(value) => {
                self.value = value;
            }
        };
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <div>
                <FormInput
                    input_type=InputType::Text
                    value=self.value.clone()
                    input_id="form-input-test"
                    oninput_signal=self.link.callback(|e: InputData| MsgWriteInput::Input(e.value))
                />
                <span id="result">{self.value.clone()}</span>
            </div>
        }
    }
}

