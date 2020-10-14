use super::error_message::get_error_message;
use crate::styles::{get_pallete, get_size, Palette, Size};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

/// # Form Textearea
///
/// ## Features required
///
/// forms
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_styles::forms::form_textarea::FormTextArea;
/// use yew_styles::styles::{Palette, Size};
///
/// pub struct FormTextAreaExample {
///     pub link: ComponentLink<Self>,
///     pub value: String,
/// }
///
/// pub enum Msg {
///     Input(String),
/// }
///
/// impl Component for FormTextAreaExample {
///     type Message = Msg;
///     type Properties = ();
///     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
///         FormTextAreaExample {
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
///             <FormTextArea placeholder="write here"
///                 textarea_size=Size::Small
///                 textarea_style=Palette::Info
///                 oninput_signal=form_page.link.callback(|e: InputData| Msg::Input(e.value))
///             />
///         }
///     }
/// ```
pub struct FormTextArea {
    link: ComponentLink<Self>,
    props: Props,
}

/// Type of wraps. You can find more information [here](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
#[derive(Clone, PartialEq)]
pub enum WrapText {
    Hard,
    Soft,
    Off,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
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
    /// Content to be appear in the form control when the form control is empty
    #[prop_or_default]
    pub placeholder: String,
    /// The input style according with the purpose. Default `Palette::Standard`
    #[prop_or(Palette::Standard)]
    pub textarea_style: Palette,
    /// The size of the input. Default `Size::Medium`
    #[prop_or(Size::Medium)]
    pub textarea_size: Size,
    /// Maximum length (number of characters) of value. Default `1000`
    #[prop_or(1000)]
    pub maxlength: u32,
    /// Minimum length (number of characters) of value
    #[prop_or_default]
    pub minlength: u16,
    /// Whether the form control is disabled. Default `false`
    #[prop_or(false)]
    pub disabled: bool,
    /// The name of the textarea
    #[prop_or_default]
    pub name: String,
    /// The value is not editable. Default `false`
    #[prop_or(false)]
    pub readonly: bool,
    /// A value is required or must be check for the form to be submittable. Default `false`
    #[prop_or(false)]
    pub required: bool,
    /// Automatically focus the form control when the page is loaded. Default `false`
    #[prop_or(false)]
    pub autofocus: bool,
    /// Hint for form autofill feature. Default `false`
    #[prop_or(false)]
    pub autocomplete: bool,
    /// The visible width of the text control
    #[prop_or_default]
    pub cols: u16,
    /// The number of visible text lines for the control
    #[prop_or_default]
    pub rows: u16,
    /// Specifies whether the "textarea"
    /// is subject to spell checking by the underlying browser/OS. Default `false`
    #[prop_or(false)]
    pub spellcheck: bool,
    /// Signal to emit the event input
    #[prop_or(Callback::noop())]
    pub oninput_signal: Callback<InputData>,
    /// Signal to emit the event blur
    #[prop_or(Callback::noop())]
    pub onblur_signal: Callback<FocusEvent>,
    /// Signal to emit the event keypress
    #[prop_or(Callback::noop())]
    pub onkeydown_signal: Callback<KeyboardEvent>,
    /// Error state for validation. Default `false`
    #[prop_or(false)]
    pub error_state: bool,
    /// Show error message when error_state is true
    #[prop_or_default]
    pub error_message: String,
    /// Indicates how the control wraps text. Default `WrapText::Soft`
    #[prop_or(WrapText::Soft)]
    pub wrap: WrapText,
}

#[derive(Debug)]
pub enum Msg {
    Input(InputData),
    Blur(FocusEvent),
    KeyPressed(KeyboardEvent),
}

impl Component for FormTextArea {
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
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <textarea
                    id=self.props.id
                    class=format!("form-textarea {} {} {}",
                    self.props.class_name,
                    get_pallete(self.props.textarea_style.clone()),
                    get_size(self.props.textarea_size.clone()))
                    key=self.props.key.clone()
                    ref=self.props.code_ref.clone()
                    oninput=self.link.callback(|input_data| Msg::Input(input_data))
                    onblur=self.link.callback(|focus_event| Msg::Blur(focus_event))
                    onkeydown=self.link.callback(|keyboard_event| Msg::KeyPressed(keyboard_event))
                    name=self.props.name
                    autocomplete=self.props.autocomplete
                    autofocus=self.props.autofocus
                    required=self.props.required
                    readonly=self.props.readonly
                    disabled=self.props.disabled
                    rows=self.props.rows
                    placeholder=self.props.placeholder
                    cols=self.props.cols
                    spellcheck=self.props.spellcheck
                    minlength=self.props.minlength
                    maxlength=self.props.maxlength
                    warp=get_wrap(self.props.wrap.clone())
                />
                {get_error_message(self.props.error_state, self.props.error_message.clone())}
            </>
        }
    }
}

fn get_wrap(wrap_text: WrapText) -> String {
    match wrap_text {
        WrapText::Hard => "hard".to_string(),
        WrapText::Off => "soft".to_string(),
        WrapText::Soft => "off".to_string(),
    }
}

#[wasm_bindgen_test]
fn should_create_form_textarea() {
    let props = Props {
        id: "form-input-id-test".to_string(),
        key: "".to_string(),
        code_ref: NodeRef::default(),
        class_name: "form-input-class-test".to_string(),
        oninput_signal: Callback::noop(),
        onblur_signal: Callback::noop(),
        onkeydown_signal: Callback::noop(),
        error_message: "invalid input".to_string(),
        error_state: false,
        name: "input-test".to_string(),
        textarea_style: Palette::Standard,
        textarea_size: Size::Medium,
        placeholder: "test input".to_string(),
        required: false,
        autocomplete: false,
        autofocus: false,
        maxlength: 100,
        minlength: 0,
        readonly: false,
        disabled: false,
        cols: 20,
        rows: 10,
        spellcheck: true,
        wrap: WrapText::Hard,
    };

    let form_textarea: App<FormTextArea> = App::new();

    form_textarea.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let form_textarea_element = utils::document()
        .get_element_by_id("form-input-id-test")
        .unwrap();

    assert_eq!(form_textarea_element.tag_name(), "TEXTAREA");
}
