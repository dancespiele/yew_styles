use super::error_message::get_error_message;
use crate::styles::{get_pallete, get_size, Palette, Size};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

pub struct FormFile {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    /// Current value of the form control. Required
    pub value: String,
    /// One or more unique file type specifiers describing file types to allow
    pub accept: Vec<String>,
    /// The input style according with the purpose
    #[prop_or(Palette::Standard)]
    pub input_type: Palette,
    /// The size of the input
    #[prop_or(Size::Medium)]
    pub input_size: Size,
    /// Signal to emit the event change
    pub onchange_signal: Callback<ChangeData>,
    /// Media capture input method in file upload controls
    #[prop_or_default]
    pub capture: String,
    /// Whether to allow multiple values
    #[prop_or_default]
    pub multiple: bool,
    /// Hide the file input element
    #[prop_or(false)]
    pub hidden: bool,
    /// Underline style instead of box, like Material
    #[prop_or_default]
    pub underline: bool,
    /// Error state for validation
    #[prop_or_default]
    pub error_state: bool,
    /// Show error message when error_state is true
    #[prop_or_default]
    pub error_message: String,
    /// Alt attribute for the image type
    #[prop_or_default]
    pub alt: String,
    /// Automatically focus the form control when the page is loaded
    #[prop_or_default]
    pub autofocus: bool,
    /// The name of the input
    #[prop_or_default]
    pub name: String,
    /// A value is required or must be check for the form to be submittable
    #[prop_or(false)]
    pub required: bool,
    /// The value is not editable
    #[prop_or_default]
    pub readonly: bool,
    /// Whether the form control is disabled
    #[prop_or_default]
    pub disabled: bool,
    /// Content to be appear in the form control when the form control is empty
    #[prop_or_default]
    pub placeholder: String,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
}

#[derive(Debug)]
pub enum Msg {
    Changed(ChangeData),
}

impl Component for FormFile {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Changed(changed_data) => {
                self.props.onchange_signal.emit(changed_data);
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
                    type="file"
                    id=self.props.id
                    class=format!(
                        "form-file {} {} {} {} {}",
                        self.props.class_name,
                        get_pallete(self.props.input_type.clone()),
                        get_size(self.props.input_size.clone()),
                        if self.props.underline { "underline" } else { "" },
                        if self.props.hidden { "hidden" } else { "" }
                    )
                    onchange=self.link.callback(Msg::Changed)
                    multiple=self.props.multiple
                    value=self.props.value
                    name=self.props.name
                    alt=self.props.alt
                    accept=self.props.accept.join(", ")
                    capture=self.props.capture
                    required=self.props.required
                    readonly=self.props.readonly
                    disabled=self.props.disabled
                    autofocus=self.props.autofocus
                    placeholder=self.props.placeholder
                />
                {get_error_message(self.props.error_state, self.props.error_message.clone())}
            </>
        }
    }
}

#[wasm_bindgen_test]
fn should_create_form_input() {
    let props = Props {
        id: "form-input-id-test".to_string(),
        class_name: "form-input-class-test".to_string(),
        value: "".to_string(),
        onchange_signal: Callback::noop(),
        error_message: "invalid input".to_string(),
        error_state: false,
        name: "input-test".to_string(),
        input_type: Palette::Standard,
        input_size: Size::Medium,
        placeholder: "test input".to_string(),
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

    let form_input: App<FormFile> = App::new();

    form_input.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let form_input_element = utils::document()
        .get_element_by_id("form-input-id-test")
        .unwrap();

    assert_eq!(form_input_element.tag_name(), "INPUT");
}
