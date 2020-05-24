use crate::styles::{get_pallete, get_size, Palette, Size};
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
    #[prop_or(Palette::Standard)]
    pub input_style: Palette,
    #[prop_or(Size::Medium)]
    pub input_size: Size,
    #[prop_or(Callback::noop())]
    pub oninput_signal: Callback<InputData>,
    #[prop_or(Callback::noop())]
    pub onblur_signal: Callback<FocusEvent>,
    #[prop_or(Callback::noop())]
    pub onkeypress_signal: Callback<KeyboardEvent>,
    #[prop_or(Callback::noop())]
    pub onchange_signal: Callback<ChangeData>,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub class_name: String,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub accept: String,
    #[prop_or_default]
    pub alt: String,
    #[prop_or_default]
    pub autofocus: bool,
    #[prop_or_default]
    pub autocomplete: bool,
    #[prop_or_default]
    pub list: String,
    #[prop_or_default]
    pub min: u16,
    #[prop_or_default]
    pub max: u16,
    #[prop_or_default]
    pub minlength: u16,
    #[prop_or(1000)]
    pub maxlength: u16,
    #[prop_or_default]
    pub pattern: String,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub multiple: bool,
    #[prop_or_default]
    pub underline: bool,
    #[prop_or_default]
    pub capture: String,
    #[prop_or_default]
    pub step: i16,
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
            <>
                <input
                    id=self.props.id
                    class=format!(
                        "form-input {} {} {} {} {}",
                        self.props.class_name,
                        get_pallete(self.props.input_style.clone()),
                        get_size(self.props.input_size.clone()),
                        if self.props.error_state { "error" } else { "" },
                        if self.props.underline { "underline" } else { "" }
                    )
                    type=get_type(self.props.input_type.clone())
                    oninput=self.link.callback(|input_data| Msg::Input(input_data))
                    checked=self.props.checked
                    onblur=self.link.callback(|focus_event| Msg::Blur(focus_event))
                    onkeypress=self.link.callback(|keyboard_event| Msg::KeyPressed(keyboard_event))
                    onchange=self.link.callback(|change_data| Msg::Changed(change_data))
                    value=self.props.value
                    name=self.props.name
                    required=self.props.required
                    readonly=self.props.readonly
                    disabled=self.props.disabled
                    multiple=self.props.multiple
                    placeholder=self.props.placeholder
                    pattern=self.props.pattern
                    min=self.props.min
                    minlength=self.props.minlength
                    max=self.props.max
                    maxlength=self.props.maxlength
                    alt=self.props.alt
                    accept=self.props.accept
                    capture=self.props.capture
                    autofocus=self.props.autofocus
                    autocomplete=self.props.autocomplete
                    step=self.props.step
                    list=self.props.list
                />
                {get_error_message(self.props.error_state, self.props.error_message.clone())}
            </>
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
        InputType::Tel => "tel".to_string(),
        InputType::Text => "text".to_string(),
        InputType::Time => "time".to_string(),
        InputType::Url => "url".to_string(),
        InputType::Week => "week".to_string(),
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
fn should_create_form_input() {
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
        name: "input-test".to_string(),
        input_style: Palette::Standard,
        input_size: Size::Medium,
        placeholder: "test input".to_string(),
        required: false,
        autocomplete: false,
        autofocus: false,
        multiple: false,
        alt: "input test".to_string(),
        pattern: "".to_string(),
        min: 0,
        max: 0,
        maxlength: 100,
        minlength: 0,
        readonly: false,
        underline: false,
        disabled: false,
        step: 1,
        accept: "".to_string(),
        capture: "".to_string(),
        list: "".to_string(),
    };

    let form_input: App<FormInput> = App::new();

    form_input.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let form_input_element = utils::document()
        .get_element_by_id("form-input-id-test")
        .unwrap();

    assert_eq!(form_input_element.tag_name(), "INPUT");
}
