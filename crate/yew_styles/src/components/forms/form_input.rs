use super::error_message::get_error_message;
use crate::styles::colors::get_styles;
use crate::styles::helpers::{get_common_form_styles, get_palette, get_size, Palette, Size};
use gloo::utils;
use stylist::{css, StyleSource, YieldStyle};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::start_app_with_props;

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
/// use yew_styles::styles::helpers::{Palette, Size};
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
///                 input_type=InputType::Text
///                 input_palette=Palette::Standard
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
    props: Props,
}

/// Different type inputs supported. You can find more information [here](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input)
#[derive(Clone, PartialEq)]
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

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// The input type. Default `InputType::Text`
    #[prop_or(InputType::Text)]
    pub input_type: InputType,
    /// The input style according with the purpose. Default `Palette::Standard`
    #[prop_or(Palette::Standard)]
    pub input_palette: Palette,
    /// The size of the input. Default `Size::Medium`
    #[prop_or(Size::Medium)]
    pub input_size: Size,
    /// Signal to emit the event input
    #[prop_or(Callback::noop())]
    pub oninput_signal: Callback<InputEvent>,
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
    /// General property to get the ref of the component
    #[prop_or_default]
    pub code_ref: NodeRef,
    /// General property to add keys
    #[prop_or_default]
    pub key: String,
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
    /// Automatically focus the form control when the page is loaded. Default `false`
    #[prop_or(false)]
    pub autofocus: bool,
    /// Hint for form autofill feature. . Default `false`
    #[prop_or(false)]
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
    /// Maximum length (number of characters) of value. Default `1000`
    #[prop_or(1000)]
    pub maxlength: u16,
    /// Pattern the value must match to be valid. Default `"[\\s\\S]*".to_string()`
    #[prop_or("[\\s\\S]*".to_string())]
    pub pattern: String,
    /// The value is not editable. Default `false`
    #[prop_or(false)]
    pub readonly: bool,
    /// A value is required or must be check for the form to be submittable. Default `false`
    #[prop_or(false)]
    pub required: bool,
    /// Whether the form control is disabled. Default `false`
    #[prop_or(false)]
    pub disabled: bool,
    /// Underline style instead of box, like Material. Default `false`
    #[prop_or(false)]
    pub underline: bool,
    /// Incremental values that are valid
    #[prop_or_default]
    pub step: i16,
    /// Error state for validation. Default `false`
    #[prop_or(false)]
    pub error_state: bool,
    /// Show error message when error_state is true
    #[prop_or_default]
    pub error_message: String,
    /// Set css styles directly in the component
    #[prop_or(css!(""))]
    pub styles: StyleSource<'static>,
}

#[derive(Debug)]
pub enum Msg {
    Input(InputEvent),
    Blur(FocusEvent),
    KeyPressed(KeyboardEvent),
}

impl YieldStyle for FormInput {
    fn style_from(&self) -> StyleSource<'static> {
        let styles = get_styles();
        let color = styles
            .get("outline")
            .unwrap()
            .iter()
            .find(|pallete| pallete.name == get_palette(self.props.input_palette.clone()))
            .unwrap();

        get_common_form_styles(color)
    }
}

impl Component for FormInput {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: *ctx.props(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props = *ctx.props();

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            input_palette,
            input_size,
            input_type,
            oninput_signal,
            onblur_signal,
            onkeydown_signal,
            placeholder,
            checked,
            code_ref,
            key,
            class_name,
            id,
            name,
            alt,
            autofocus,
            autocomplete,
            list,
            min,
            max,
            minlength,
            maxlength,
            pattern,
            readonly,
            required,
            disabled,
            underline,
            step,
            error_state,
            error_message,
            styles,
        } = &ctx.props();

        html! {
            <>
                <input
                    id={id.clone()}
                    class={classes!(
                        self.style(),
                        get_size(input_size.clone()),
                        if *underline { "underline" } else { "" },
                        class_name.clone(),
                        styles.clone(),
                    )}
                    key={key.clone()}
                    ref={code_ref.clone()}
                    type={get_type(input_type.clone())}
                    oninput={ctx.link().callback(Msg::Input)}
                    checked={*checked}
                    onblur={ctx.link().callback(Msg::Blur)}
                    onkeydown={ctx.link().callback(Msg::KeyPressed)}
                    name={name.clone()}
                    required={*required}
                    readonly={*readonly}
                    disabled={*disabled}
                    placeholder={placeholder.clone()}
                    pattern={pattern.clone()}
                    min={min.to_string()}
                    minlength={minlength.to_string()}
                    max={max.to_string()}
                    maxlength={maxlength.to_string()}
                    alt={alt.clone()}
                    autofocus={*autofocus}
                    autocomplete={autocomplete.to_string()}
                    step={step.to_string()}
                    list={list.clone()}
                />
                {get_error_message(*error_state, error_message.clone())}
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
        key: "".to_string(),
        code_ref: NodeRef::default(),
        id: "form-input-id-test".to_string(),
        class_name: "form-input-class-test".to_string(),
        input_type: InputType::Text,
        oninput_signal: Callback::noop(),
        onblur_signal: Callback::noop(),
        onkeydown_signal: Callback::noop(),
        checked: false,
        error_message: "invalid input".to_string(),
        error_state: false,
        name: "input-test".to_string(),
        input_palette: Palette::Standard,
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
        styles: css!("background-color: #918d94;"),
    };

    start_app_with_props::<FormInput>(props);

    let form_input_element = utils::document()
        .get_element_by_id("form-input-id-test")
        .unwrap();

    assert_eq!(form_input_element.tag_name(), "INPUT");
}
