use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

/// # Form Label
///
/// ## Features required
///
/// forms
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_styles::forms::{
///     form_label::FormLabel,
///     form_group::{FormGroup, Orientation},
///     form_textarea::FormTextArea,
/// };
/// use yew_styles::styles::{Palette, Size};
///
/// pub struct FormLabelExample {
///     pub link: ComponentLink<Self>,
///     pub value: String,
/// }
///
/// pub enum Msg {
///     Input(String),
/// }
///
/// impl Component for FormLabelExample {
///     type Message = Msg;
///     type Properties = ();
///     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
///         FormLabelExample {
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
///             <FormGroup orientation=Orientation::Vertical>
///                 <FormLabel
///                 text="Info small textarea"
///                 />
///                 <FormTextArea placeholder="write here"
///                     value=form_page.value.clone()
///                     textarea_size=Size::Small
///                     textarea_style=Palette::Info
///                     oninput_signal=form_page.link.callback(|e: InputData| Msg::Input(e.value))
///                 />
///             </FormGroup>
///         }
///     }
/// ```
pub struct FormLabel {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// text of the label. Required
    pub text: String,
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
    /// The id of a labelable form-related element in the same document as the <label> element
    #[prop_or_default]
    pub label_for: String,
}

impl Component for FormLabel {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
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
            <label
                class=format!("form-label {}", self.props.class_name)
                id=self.props.id
                key=self.props.key.clone()
                ref=self.props.code_ref.clone()
                for=self.props.label_for
            >{self.props.text.clone()}</label>
        }
    }
}

#[wasm_bindgen_test]
fn should_create_form_label() {
    let props = Props {
        key: "".to_string(),
        code_ref: NodeRef::default(),
        class_name: "form-label-class-test".to_string(),
        id: "form-label-id-test".to_string(),
        label_for: "label-form".to_string(),
        text: "label text".to_string(),
    };

    let form_label: App<FormLabel> = App::new();

    form_label.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let form_label_element = utils::document()
        .get_element_by_id("form-label-id-test")
        .unwrap();

    assert_eq!(
        form_label_element.text_content().unwrap(),
        "label text".to_string()
    )
}
