use super::error_message::get_error_message;
use crate::styles::colors::get_styles;
use crate::styles::helpers::{get_common_form_styles, get_palette, get_size, Palette, Size};
use gloo::utils;
use stylist::{css, StyleSource, YieldStyle};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::start_app_with_props;

pub struct FormFile {
    props: Props,
}

impl YieldStyle for FormFile {
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

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// One or more unique file type specifiers describing file types to allow. Required
    pub accept: Vec<String>,
    /// The input style according with the purpose. Default `Palette::Standard`
    #[prop_or(Palette::Standard)]
    pub input_palette: Palette,
    /// The size of the input. Default `Size::Medium`
    #[prop_or(Size::Medium)]
    pub input_size: Size,
    /// Signal to emit the event change
    pub onchange_signal: Callback<Event>,
    /// Media capture input method in file upload controls
    #[prop_or_default]
    pub capture: String,
    /// Whether to allow multiple values
    #[prop_or(false)]
    pub multiple: bool,
    /// Hide the file input element. Default `false`
    #[prop_or(false)]
    pub hidden: bool,
    /// Underline style instead of box, like Material. Default `false`
    #[prop_or(false)]
    pub underline: bool,
    /// Error state for validation. Default `false`
    #[prop_or(false)]
    pub error_state: bool,
    /// Show error message when error_state is true
    #[prop_or_default]
    pub error_message: String,
    /// Alt attribute for the image type
    #[prop_or_default]
    pub alt: String,
    /// Automatically focus the form control when the page is loaded. Default `false`
    #[prop_or(false)]
    pub autofocus: bool,
    /// The name of the input
    #[prop_or_default]
    pub name: String,
    /// A value is required or must be check for the form to be submittable. Default `false`
    #[prop_or(false)]
    pub required: bool,
    /// The value is not editable. Default `false`
    #[prop_or(false)]
    pub readonly: bool,
    /// Whether the form control is disabled. Default `false`
    #[prop_or(false)]
    pub disabled: bool,
    /// General property to get the ref of the component
    #[prop_or_default]
    pub code_ref: NodeRef,
    /// General property to add keys
    #[prop_or_default]
    pub key: String,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// Set css styles directly in the component
    #[prop_or(css!(""))]
    pub styles: StyleSource<'static>,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
}

#[derive(Debug)]
pub enum Msg {
    Changed(Event),
}

impl Component for FormFile {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: *ctx.props(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Changed(changed_data) => {
                ctx.props().onchange_signal.emit(changed_data);
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
            accept,
            input_palette,
            input_size,
            onchange_signal,
            capture,
            multiple,
            hidden,
            underline,
            error_state,
            error_message,
            alt,
            autofocus,
            name,
            required,
            readonly,
            disabled,
            code_ref,
            key,
            class_name,
            styles,
            id,
        } = &ctx.props();

        html! {
            <>
                <input
                    type="file"
                    id={id.clone()}
                    class={classes!(
                        self.style(),
                        get_size(input_size.clone()),
                        if *underline { "underline" } else { "" },
                        if *hidden { "hidden" } else { "" },
                        class_name.clone(),
                        styles.clone(),
                    )}
                    key={key.clone()}
                    ref={code_ref.clone()}
                    onchange={ctx.link().callback(Msg::Changed)}
                    multiple={*multiple}
                    name={name.clone()}
                    alt={alt.clone()}
                    accept={accept.join(", ")}
                    capture={capture.clone()}
                    required={*required}
                    readonly={*readonly}
                    disabled={*disabled}
                    autofocus={*autofocus}
                />
                {get_error_message(*error_state, error_message.clone())}
            </>
        }
    }
}

#[wasm_bindgen_test]
fn should_create_form_input() {
    let props = Props {
        key: "".to_string(),
        code_ref: NodeRef::default(),
        id: "form-input-id-test".to_string(),
        class_name: "form-input-class-test".to_string(),
        styles: css!("background-color: #918d94;"),
        onchange_signal: Callback::noop(),
        error_message: "invalid input".to_string(),
        error_state: false,
        name: "input-test".to_string(),
        input_palette: Palette::Standard,
        input_size: Size::Medium,
        required: false,
        autofocus: false,
        multiple: false,
        alt: "input test".to_string(),
        readonly: false,
        underline: false,
        disabled: false,
        accept: vec!["image/png".to_string()],
        hidden: false,
        capture: "".to_string(),
    };

    start_app_with_props::<FormFile>(props);

    let form_input_element = utils::document()
        .get_element_by_id("form-input-id-test")
        .unwrap();

    assert_eq!(form_input_element.tag_name(), "INPUT");
}
