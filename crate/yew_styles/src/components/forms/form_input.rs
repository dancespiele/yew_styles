use super::error_message::get_error_message;
use crate::styles::{get_pallete, get_size, Palette, Size};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

/// # Form Input
///
/// ## Features required
///
/// forms
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_styles::forms::form_input::{FormInput, InputType};
/// use yew_styles::styles::{Palette, Size};
///
/// pub struct FormInputExample {
///     pub link: ComponentLink<Self>,
///     pub value: String,
/// }
///
/// pub enum Msg {
///     Input(String),
/// }
///
/// impl Component for FormInputExample {
///     type Message = Msg;
///     type Properties = ();
///     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
///         FormInputExample {
///             link,
///             value: "".to_string(),
///         }
///     }
///     fn update(&mut self, msg: Self::Message) -> ShouldRender {
///         match msg {
///             Msg::Input(value) => {
///                 self.value = value;
///             }
///         }
///         true
///     }
///     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
///         false
///     }
///
///     fn view(&self) -> Html {
///         html!{
///             <FormInput
///                 input_content_type=InputType::Text
///                 value=form_page.value.clone()
///                 input_type=Palette::Standard
///                 input_size=Size::Medium
///                 id="form-input-example"
///                 oninput_signal = form_page.link.callback(|e: InputData| Msg::Input(e.value))
///                 placeholder="example"
///                 underline=false
///             />
///         }
///     }
/// ```
pub struct FormInput {
    link: ComponentLink<Self>,
    props: Props,
}

/// Different type inputs supported. You can find more information [here](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input)
#[derive(Clone)]
pub enum InputType {
    Button,
    Checkbox,
    Color,
    Date,
    Datetime,
    DatetimeLocal,
    Email,
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
    /// Current value of the form control. Required
    pub value: String,
    /// The input type
    #[prop_or(InputType::Text)]
    pub input_content_type: InputType,
    /// The input style according with the purpose
    #[prop_or(Palette::Standard)]
    pub input_type: Palette,
    /// The size of the input
    #[prop_or(Size::Medium)]
    pub input_size: Size,
    /// Signal to emit the event input
    #[prop_or(Callback::noop())]
    pub oninput_signal: Callback<InputData>,
    /// Signal to emit the event blur
    #[prop_or(Callback::noop())]
    pub onblur_signal: Callback<FocusEvent>,
    /// Signal to emit the event keypress
    #[prop_or(Callback::noop())]
    pub onkeydown_signal: Callback<KeyboardEvent>,
    /// Content to be appear in the form control when the form control is empty
    #[prop_or_default]
    pub placeholder: String,
    /// Whether the command or control is checked
    #[prop_or_default]
    pub checked: bool,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    /// The name of the input
    #[prop_or_default]
    pub name: String,
    /// Alt attribute for the image type
    #[prop_or_default]
    pub alt: String,
    /// Automatically focus the form control when the page is loaded
    #[prop_or_default]
    pub autofocus: bool,
    /// Hint for form autofill feature
    #[prop_or_default]
    pub autocomplete: bool,
    /// Value of the id attribute of the "datalist" of autocomplete options
    #[prop_or_default]
    pub list: String,
    /// Minimum value
    #[prop_or_default]
    pub min: u16,
    /// Maximum value
    #[prop_or_default]
    pub max: u16,
    /// Minimum length (number of characters) of value
    #[prop_or_default]
    pub minlength: u16,
    /// Maximum length (number of characters) of value
    #[prop_or(1000)]
    pub maxlength: u16,
    /// Pattern the value must match to be valid
    #[prop_or("[\\s\\S]*".to_string())]
    pub pattern: String,
    /// The value is not editable
    #[prop_or_default]
    pub readonly: bool,
    /// A value is required or must be check for the form to be submittable
    #[prop_or(false)]
    pub required: bool,
    /// Whether the form control is disabled
    #[prop_or(false)]
    pub disabled: bool,
    /// Underline style instead of box, like Material
    #[prop_or_default]
    pub underline: bool,
    /// Incremental values that are valid
    #[prop_or_default]
    pub step: i16,
    /// Error state for validation
    #[prop_or_default]
    pub error_state: bool,
    /// Show error message when error_state is true
    #[prop_or_default]
    pub error_message: String,
}

#[derive(Debug)]
pub enum Msg {
    Input(InputData),
    Blur(FocusEvent),
    KeyPressed(KeyboardEvent),
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
                self.props.onkeydown_signal.emit(keyboard_event);
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
                        "form-input {} {} {} {}",
                        self.props.class_name,
                        get_pallete(self.props.input_type.clone()),
                        get_size(self.props.input_size.clone()),
                        if self.props.underline { "underline" } else { "" }
                    )
                    type=get_type(self.props.input_content_type.clone())
                    oninput=self.link.callback(Msg::Input)
                    checked=self.props.checked
                    onblur=self.link.callback(Msg::Blur)
                    onkeydown=self.link.callback(Msg::KeyPressed)
                    value=self.props.value
                    name=self.props.name
                    required=self.props.required
                    readonly=self.props.readonly
                    disabled=self.props.disabled
                    placeholder=self.props.placeholder
                    pattern=self.props.pattern
                    min=self.props.min
                    minlength=self.props.minlength
                    max=self.props.max
                    maxlength=self.props.maxlength
                    alt=self.props.alt
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

fn get_type(input_content_type: InputType) -> String {
    match input_content_type {
        InputType::Button => "button".to_string(),
        InputType::Checkbox => "checkbox".to_string(),
        InputType::Color => "color".to_string(),
        InputType::Date => "date".to_string(),
        InputType::Datetime => "datetime".to_string(),
        InputType::DatetimeLocal => "datetime-local".to_string(),
        InputType::Email => "email".to_string(),
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

#[wasm_bindgen_test]
fn should_create_form_input() {
    let props = Props {
        id: "form-input-id-test".to_string(),
        class_name: "form-input-class-test".to_string(),
        value: "".to_string(),
        input_content_type: InputType::Text,
        oninput_signal: Callback::noop(),
        onblur_signal: Callback::noop(),
        onkeydown_signal: Callback::noop(),
        checked: false,
        error_message: "invalid input".to_string(),
        error_state: false,
        name: "input-test".to_string(),
        input_type: Palette::Standard,
        input_size: Size::Medium,
        placeholder: "test input".to_string(),
        required: false,
        autocomplete: false,
        autofocus: false,
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
