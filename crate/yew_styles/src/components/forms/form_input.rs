use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App, ChangeData, FocusEvent, InputData, KeyboardEvent};

pub struct FormInput {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone)]
pub enum InputType {
    Button,
    Checkbox,
    Color,
    Date,
    Datetime,
    DatetimeLocal,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Radio,
    Range,
    Reset,
    Search,
    Submit,
    Tel,
    Text,
    Time,
    Url,
    Week,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub value: String,
    pub input_type: InputType,
    #[prop_or_default(Callback::noop())]
    pub oninput_signal: Callback<InputData>,
    #[prop_or_default(Callback::noop())]
    pub onblur_signal: Callback<FocusEvent>,
    #[prop_or_default(Callback::noop())]
    pub onkeypress_signal: Callback<KeyboardEvent>,
    #[prop_or_default(Callback::noop())]
    pub onchange_signal: Callback<ChangeData>,
    #[prop_or_default]
    pub checked: bool,
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
    Input(InputData),
    Blur(FocusEvent),
    KeyPressed(KeyboardEvent),
    Changed(ChangeData),
}

impl Component for FormInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(input_data) => {
                self.props.oninput_signal.emit(input_data);
            }
            Msg::Blur(focus_event) => {
                self.props.onblur_signal.emit(focus_event);
            }
            Msg::KeyPressed(keyboard_event) => {
                self.props.onkeypress_signal.emit(keyboard_event);
            }
            Msg::Changed(change_data) => {
                self.props.onchange_signal.emit(change_data);
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
                    type=get_type(self.props.input_type.clone())
                    oninput=self.link.callback(|input_data| Msg::Input(input_data))
                    checked=self.props.checked
                    onblur=self.link.callback(|focus_event| Msg::Blur(focus_event))
                    onkeypress=self.link.callback(|keyboard_event| Msg::KeyPressed(keyboard_event))
                    onchange=self.link.callback(|change_data| Msg::Changed(change_data))
                    value=self.props.value
                />
                {get_error_message(self.props.error_state, self.props.error_message.clone())}
            </div>
        }
    }
}

fn get_type(input_type: InputType) -> String {
    match input_type {
        InputType::Button => "button".to_string(),
        InputType::Checkbox => "checkbox".to_string(),
        InputType::Color => "color".to_string(),
        InputType::Date => "date".to_string(),
        InputType::Datetime => "datetime".to_string(),
        InputType::DatetimeLocal => "datetime-local".to_string(),
        InputType::Email => "email".to_string(),
        InputType::File => "file".to_string(),
        InputType::Hidden => "hidden".to_string(),
        InputType::Image => "image".to_string(),
        InputType::Month => "month".to_string(),
        InputType::Number => "number".to_string(),
        InputType::Password => "password".to_string(),
        InputType::Radio => "radio".to_string(),
        InputType::Range => "range".to_string(),
        InputType::Reset => "reset".to_string(),
        InputType::Search => "search".to_string(),
        InputType::Submit => "submit".to_string(),
        InputType::Tel => "tel".to_string(),
        InputType::Text => "text".to_string(),
        InputType::Time => "time".to_string(),
        InputType::Url => "url".to_string(),
        InputType::Week => "week".to_string(),
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

#[wasm_bindgen_test]
fn should_create_form_input_with_label() {
    let props = Props {
        id: "form-input-id-test".to_string(),
        class_name: "form-input-class-test".to_string(),
        value: "".to_string(),
        input_type: InputType::Text,
        oninput_signal: Callback::noop(),
        onblur_signal: Callback::noop(),
        onchange_signal: Callback::noop(),
        onkeypress_signal: Callback::noop(),
        checked: false,
        error_message: "invalid input".to_string(),
        error_state: false,
        input_id: "input-test".to_string(),
        label: "Input label".to_string(),
        name: "input-test".to_string(),
    };

    let form_input: App<FormInput> = App::new();

    form_input.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let form_input_element = utils::document()
        .get_element_by_id("form-input-id-test")
        .unwrap();

    let label_element = form_input_element.query_selector("label").unwrap().unwrap();

    let input_element = form_input_element.query_selector("input").unwrap().unwrap();

    assert_eq!(label_element.tag_name(), "LABEL");
    assert_eq!(input_element.id(), "input-test");
}

#[wasm_bindgen_test]
fn should_create_form_input_without_label() {
    let props = Props {
        id: "form-input-id-test".to_string(),
        class_name: "form-input-class-test".to_string(),
        value: "".to_string(),
        input_type: InputType::Text,
        oninput_signal: Callback::noop(),
        onblur_signal: Callback::noop(),
        onchange_signal: Callback::noop(),
        onkeypress_signal: Callback::noop(),
        checked: false,
        error_message: "invalid input".to_string(),
        error_state: false,
        input_id: "input-test".to_string(),
        label: "".to_string(),
        name: "input-test".to_string(),
    };

    let form_input: App<FormInput> = App::new();

    form_input.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let form_input_element = utils::document()
        .get_element_by_id("form-input-id-test")
        .unwrap();

    let label_element = form_input_element.query_selector("label").unwrap();

    let input_element = form_input_element.query_selector("input").unwrap().unwrap();

    assert_eq!(label_element, None);
    assert_eq!(input_element.id(), "input-test");
}
