use super::error_message::get_error_message;
use crate::styles::colors::{darker, get_styles};
use crate::styles::helpers::{get_iteractions, get_palette, get_size, Palette, Size};
use stylist::{css, StyleSource, YieldStyle};
use yew::prelude::*;

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
/// use yew_styles::styles::helpers::{Palette, Size};
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
    pub oninput_signal: Callback<InputEvent>,
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

impl YieldStyle for FormTextArea {
    fn style_from(&self) -> StyleSource<'static> {
        let styles = get_styles();
        let color = styles
            .get("outline")
            .unwrap()
            .iter()
            .find(|pallete| pallete.name == get_palette(self.props.textarea_style.clone()))
            .unwrap();

        css!(
            r#"
                padding: 5px;
                height: 100px;
                box-sizing: border-box;
                border-radius: 5px;
                width: 100%;
                border: 1px solid ${border_color};
                ${iteractions}
    
                &.hidden {
                    display: none;
                }
    
                &::-webkit-input-placeholder {
                    color: ${color};
                }
    
                &:-moz-placeholder {
                    color: ${color};
                }
    
                &::-moz-placeholder {
                    color: ${color};
                }
    
                &:-ms-input-placeholder{
                    color: ${color};
                }
    
                &.small {
                    height: 50px;
                }
    
                &.big {
                    height: 250px;
                }
    
                &.underline {
                    border-radius: 2px;
                    border-top: 0;
                    border-left: 0;
                    border-right: 0;
                    border-bottom: 2px solid ${border_color};
                }
    
                &.underline:focus{
                    border-bottom-color: ${focus_color};
                }
                &.underline:hover{
                    border-bottom-color: ${hover_color};
                }
                &.underline:active{
                    border-bottom-color: ${active_color};
                }
            "#,
            border_color = color.border_color.clone(),
            color = color.color.clone(),
            iteractions = get_iteractions(
                "border-color",
                color.border_color.clone(),
                -10.0,
                -20.0,
                -30.0
            ),
            focus_color = darker(&color.border_color, -10.0),
            hover_color = darker(&color.border_color, -20.0),
            active_color = darker(&color.border_color, -30.0)
        )
    }
}

impl Component for FormTextArea {
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
            id,
            code_ref,
            key,
            class_name,
            placeholder,
            textarea_style,
            textarea_size,
            maxlength,
            minlength,
            disabled,
            name,
            readonly,
            required,
            autofocus,
            autocomplete,
            cols,
            rows,
            spellcheck,
            oninput_signal,
            onblur_signal,
            onkeydown_signal,
            error_state,
            error_message,
            wrap,
            styles,
        } = &ctx.props();

        html! {
            <>
                <textarea
                    id={id.clone()}
                    class={classes!(
                        self.style(),
                        get_size(textarea_size.clone()),
                        class_name.clone(),
                        styles.clone()
                    )}
                    key={key.clone()}
                    ref={code_ref.clone()}
                    oninput={ctx.link().callback(Msg::Input)}
                    onblur={ctx.link().callback(Msg::Blur)}
                    onkeydown={ctx.link().callback(Msg::KeyPressed)}
                    name={name.clone()}
                    autocomplete={autocomplete.to_string()}
                    autofocus={*autofocus}
                    required={*required}
                    readonly={*readonly}
                    disabled={*disabled}
                    rows={rows.to_string()}
                    placeholder={placeholder.clone()}
                    cols={cols.to_string()}
                    spellcheck={spellcheck.to_string()}
                    minlength={minlength.to_string()}
                    maxlength={maxlength.to_string()}
                    warp={get_wrap(wrap.clone())}
                />
                {get_error_message(*error_state, error_message.clone())}
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

// #[wasm_bindgen_test]
// fn should_create_form_textarea() {
//     let props = Props {
//         id: "form-input-id-test".to_string(),
//         key: "".to_string(),
//         code_ref: NodeRef::default(),
//         class_name: "form-input-class-test".to_string(),
//         styles: css!("background-color: #918d94;"),
//         oninput_signal: Callback::noop(),
//         onblur_signal: Callback::noop(),
//         onkeydown_signal: Callback::noop(),
//         error_message: "invalid input".to_string(),
//         error_state: false,
//         name: "input-test".to_string(),
//         textarea_style: Palette::Standard,
//         textarea_size: Size::Medium,
//         placeholder: "test input".to_string(),
//         required: false,
//         autocomplete: false,
//         autofocus: false,
//         maxlength: 100,
//         minlength: 0,
//         readonly: false,
//         disabled: false,
//         cols: 20,
//         rows: 10,
//         spellcheck: true,
//         wrap: WrapText::Hard,
//     };

//     start_app::<FormTextArea>();

//     form_textarea.mount_with_props(
//         utils::document().get_element_by_id("output").unwrap(),
//         props,
//     );

//     let form_textarea_element = utils::document()
//         .get_element_by_id("form-input-id-test")
//         .unwrap();

//     assert_eq!(form_textarea_element.tag_name(), "TEXTAREA");
// }
