use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

/// # Form
///
/// ## Features required
///
/// forms
///
/// ## Example
///
/// ```rust
/// use std::collections::HashMap;
/// use wasm_bindgen::JsCast;
/// use web_sys::{HtmlOptionElement, HtmlSelectElement};
/// use yew::prelude::*;
/// use yew::utils;
/// use yew_styles::forms::{
///     form_component::Form,
///     form_group::{FormGroup, Orientation},
///     form_input::{FormInput, InputType},
///     form_label::FormLabel,
///     form_select::FormSelect,
///     form_submit::FormSubmit,
///     form_textarea::FormTextArea,
/// };
/// use yew_styles::layouts::{
///     container::{Container, Direction, Wrap},
///     item::{Item, ItemLayout},
/// };
/// use yew_styles::styles::{Palette, Style};
///
/// #[derive(Clone)]
/// struct Fields {
///     first_name: String,
///     last_name: String,
///     email: String,
///     specialty: String,
///     skills: Vec<String>,
///     cover_letter: String,
/// }
///
/// pub struct BasicFormPage {
///     link: ComponentLink<Self>,
///     fields: HashMap<String, String>,
///     skills: Vec<String>,
///     empty_fields: Vec<String>,
///     result: Option<Fields>,
/// }
///
/// pub enum Msg {
///     FirstName(String),
///     LastName(String),
///     Email(String),
///     Specialty(String),
///     Skills(Vec<String>),
///     CoverLetter(String),
///     Submit,
/// }
///
/// impl Component for BasicFormPage {
///     type Message = Msg;
///     type Properties = ();
///
///     fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
///         Self {
///             link,
///             fields: HashMap::new(),
///             skills: vec![],
///             empty_fields: vec![],
///             result: None,
///         }
///     }
///
///     fn update(&mut self, msg: Self::Message) -> ShouldRender {
///         match msg {
///             Msg::FirstName(first_name) => {
///                 self.fields.insert("first_name".to_string(), first_name);
///             }
///             Msg::LastName(last_name) => {
///                 self.fields.insert("last_name".to_string(), last_name);
///             }
///             Msg::Email(email) => {
///                 self.fields.insert("email".to_string(), email);
///             }
///             Msg::Specialty(specialty) => {
///                 self.fields.insert("specialty".to_string(), specialty);
///             }
///             Msg::Skills(skills) => {
///                 self.skills = skills;
///             }
///             Msg::CoverLetter(cover_letter) => {
///                 self.fields.insert("cover_letter".to_string(), cover_letter);
///             }
///             Msg::Submit => {
///                 self.empty_fields = get_empty_fields(self.fields.clone());
///
///                 if self.empty_fields.is_empty() {
///                     self.result = Some(Fields {
///                         first_name: self.fields.get("first_name").unwrap().to_string(),
///                         last_name: self.fields.get("last_name").unwrap().to_string(),
///                         email: self.fields.get("email").unwrap().to_string(),
///                         specialty: self.fields.get("specialty").unwrap().to_string(),
///                         skills: self.skills.clone(),
///                         cover_letter: self.fields.get("cover_letter").unwrap().to_string(),
///                     });
///                     self.fields.drain();
///
///                     self.skills = vec![];
///                     remove_input_values();
///
///                     set_default_selected("specialty");
///                     remove_all_selected("skills");
///                 }
///             }
///         }
///
///         true
///     }
///
///     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
///         false
///     }
///
///     fn view(&self) -> Html {
///         html! {
///             <Container wrap=Wrap::Wrap direction=Direction::Row>
///                 <Item layouts=vec!(ItemLayout::ItXs(12))>
///                     <h1>{"Basic Form"}</h1>
///                 </Item>
///                 <Item layouts=vec!(ItemLayout::ItXs(12))>
///                     <Form onsubmit_signal=self.link.callback(|e| Msg::Submit)>
///                         <Container wrap=Wrap::Wrap direction=Direction::Row>
///                             <Item layouts=vec!(ItemLayout::ItM(6), ItemLayout::ItXs(12))>
///                                 <FormGroup orientation=Orientation::Horizontal>
///                                     <FormLabel text="First name: "/>
///                                     <FormInput
///                                         error_state=self.empty_fields.iter().any(|field| field == "first_name")
///                                         error_message="First name field is required"
///                                         input_type=InputType::Text
///                                         oninput_signal=self.link.callback(|e: InputData| Msg::FirstName(e.value))
///                                     />
///                                 </FormGroup>
///                                 <FormGroup orientation=Orientation::Horizontal>
///                                     <FormLabel text="Last name: "/>
///                                     <FormInput
///                                         error_state=self.empty_fields.iter().any(|field| field == "last_name")
///                                         error_message="Last name field is required"
///                                         input_type=InputType::Text
///                                         oninput_signal=self.link.callback(|e: InputData| Msg::LastName(e.value))
///                                     />
///                                 </FormGroup>
///                                 <FormGroup orientation=Orientation::Horizontal>
///                                     <FormLabel text="Email: "/>
///                                     <FormInput
///                                         error_state=self.empty_fields.iter().any(|field| field == "email")
///                                         error_message="Email field is required"
///                                         input_type=InputType::Email
///                                         oninput_signal=self.link.callback(|e: InputData| Msg::Email(e.value))
///                                     />
///                                 </FormGroup>
///                             </Item>
///                             <Item layouts=vec!(ItemLayout::ItM(6), ItemLayout::ItXs(12))>
///                                 <FormGroup orientation=Orientation::Vertical>
///                                     <FormLabel text="Specialty:"/>
///                                     <FormSelect
///                                         id="specialty"
///                                         error_state=self.empty_fields.iter().any(|field| field == "specialty")
///                                         error_message="Select specialty is required"
///                                         onchange_signal=self.link.callback(|e: ChangeData| {
///                                             match e {
///                                                 ChangeData::Select(element) => {
///                                                     let value = element.value();
///                                                     Msg::Specialty(value)
///                                                 },
///                                                 _ => unreachable!()
///                                             }
///                                         })
///                                         options=html!{
///                                             <>
///                                             <option value="" disabled=true>{"Choose specialty"}</option>
///                                                 <option value="frontend">{"Frontend"}</option>
///                                                 <option value="backend">{"Backend"}</option>
///                                             </>
///                                         }
///                                     />
///                                 </FormGroup>
///                                 <FormGroup orientation=Orientation::Vertical>
///                                     <FormLabel text="Skills:"/>
///                                     <FormSelect
///                                         id="skills"
///                                         multiple=true
///                                         onchange_signal=self.link.callback(|e: ChangeData| {
///                                             match e {
///                                                 ChangeData::Select(element) => {
///                                                     let mut values = vec![];
///                                                     let options = element.options();
///
///                                                     for i in 0..options.length() {
///                                                         let option = options
///                                                             .get_with_index(i)
///                                                             .unwrap()
///                                                             .dyn_into::<HtmlOptionElement>()
///                                                             .unwrap();
///                                                         if option.selected() {
///                                                             values.push(option.value());
///                                                         }
///                                                     }
///                                                     Msg::Skills(values)
///                                                 },
///                                                 _ => unreachable!()
///                                             }
///                                         })
///                                         options=html!{
///                                             <>
///                                                 <option value="yew">{"Yew.rs"}</option>
///                                                 <option value="rustwasm">{"Rustwasm"}</option>
///                                                 <option value="rust">{"Rust"}</option>
///                                                 <option value="warp">{"Warp"}</option>
///                                                 <option value="tokio">{"Tokio"}</option>
///                                             </>
///                                         }
///                                     />
///                                 </FormGroup>
///                             </Item>
///                             <Item layouts=vec!(ItemLayout::ItXs(12))>
///                                 <FormGroup orientation=Orientation::Vertical>
///                                     <FormLabel text="Cover letter:"/>
///                                     <FormTextArea
///                                         error_state=self.empty_fields.iter().any(|field| field == "cover_letter")
///                                         error_message="cover letter is required"
///                                         oninput_signal=self.link.callback(|e: InputData| Msg::CoverLetter(e.value))/>
///                                 </FormGroup>
///                             </Item>
///                             <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItM(3))>
///                                 <FormGroup>
///                                 <FormSubmit
///                                     value="Submit application"
///                                     submit_palette=Palette::Success
///                                     submit_style=Style::Outline
///                                 />
///                                 </FormGroup>
///                             </Item>
///                         </Container>
///                     </Form>
///                 </Item>
///                 <Item layouts=vec!(ItemLayout::ItXs(12))>
///                     {get_result(self.result.clone())}
///                 </Item>
///             </Container>
///         }
///     }
/// }
///
/// fn get_result(result: Option<Fields>) -> Html {
///     if let Some(form) = result {
///         html! {
///             <Container wrap=Wrap::Wrap direction=Direction::Row>
///                 <Item layouts=vec!(ItemLayout::ItXs(12))>
///                     <p><b>{"First name: "}</b>{form.first_name.clone()}</p>
///                     <p><b>{"last name: "}</b>{form.last_name.clone()}</p>
///                     <p><b>{"email: "}</b>{form.email.clone()}</p>
///                     <p><b>{"Specialty: "}</b>{form.specialty.clone()}</p>
///                     <p><b>{"Skills: "}</b>{form.skills.join(", ")}</p>
///                     <p><b>{"Cover letter: "}</b>{form.cover_letter}</p>
///                 </Item>
///             </Container>
///         }
///     } else {
///         html! {}
///     }
/// }
///
/// fn get_empty_fields(fields: HashMap<String, String>) -> Vec<String> {
///     let total_fields = vec![
///         "first_name",
///         "last_name",
///         "email",
///         "specialty",
///         "cover_letter",
///     ];
///     let mut empty_fields: Vec<String> = vec![];
///
///     for field in total_fields {
///         let is_filled = fields
///             .iter()
///             .any(|(key, value)| key == field && !value.is_empty());
///         if !is_filled {
///             empty_fields.push(field.to_string());
///         }
///     }
///
///     empty_fields
/// }
///
/// fn remove_all_selected(select: &str) {
///     let specialty_form_element = utils::document()
///         .get_element_by_id(select)
///         .unwrap()
///         .dyn_into::<HtmlSelectElement>()
///         .unwrap();
///     let specialty_options = specialty_form_element.options();
///
///     for i in 0..specialty_options.length() {
///         let option = specialty_options
///             .get_with_index(i)
///             .unwrap()
///             .dyn_into::<HtmlOptionElement>()
///             .unwrap();
///
///         option.set_selected(false);
///     }
/// }
///
/// fn remove_input_values() {
///     let input_ids = vec!["first-name", "last-name", "email"];
///     
///     for id in input_ids {
///         let input_from_element = utils::document()
///             .get_element_by_id(id)
///             .unwrap()
///             .dyn_into::<HtmlInputElement>()
///             .unwrap();
///     
///         input_from_element.set_value("");
///     }
///     
///     let textarea = utils::document()
///         .get_element_by_id("cover-letter")
///         .unwrap()
///         .dyn_into::<HtmlTextAreaElement>()
///         .unwrap();
///     
///     textarea.set_value("");
/// }
///
/// fn set_default_selected(select: &str) {
///     let specialty_form_element = utils::document()
///         .get_element_by_id(select)
///         .unwrap()
///         .dyn_into::<HtmlSelectElement>()
///         .unwrap();
///     let specialty_options = specialty_form_element.options();
///
///     let option = specialty_options
///         .get_with_index(0)
///         .unwrap()
///         .dyn_into::<HtmlOptionElement>()
///         .unwrap();
///
///     option.set_selected(true);
/// }
/// ```
pub struct Form {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// Signal to emit the event submit. Default
    #[prop_or(Callback::noop())]
    pub onsubmit_signal: Callback<FocusEvent>,
    pub children: Children,
    /// The URL that processes the form submission
    #[prop_or_default]
    pub action: String,
    /// The HTTP method to submit the form. Default `Method::Post`
    #[prop_or(Method::Post)]
    pub method: Method,
    /// The name of the form
    #[prop_or_default]
    pub name: String,
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
}

#[derive(Clone, PartialEq)]
pub enum Method {
    Post,
    Get,
    Dialog,
}

pub enum Msg {
    Submitted(FocusEvent),
}

impl Component for Form {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Form { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submitted(value) => {
                value.prevent_default();
                self.props.onsubmit_signal.emit(value);
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
            <form
                onsubmit=self.link.callback(Msg::Submitted)
                action=self.props.action.clone()
                method=get_method(self.props.method.clone())
                name=self.props.name.clone()
                key=self.props.key.clone()
                ref=self.props.code_ref.clone()
                class=format!("form {}", self.props.class_name)
                id=self.props.id.to_string()
            >
                { self.props.children.clone() }
            </form>
        }
    }
}

fn get_method(method: Method) -> String {
    match method {
        Method::Get => "get".to_string(),
        Method::Post => "post".to_string(),
        Method::Dialog => "dialog".to_string(),
    }
}

#[wasm_bindgen_test]
fn should_create_form_component() {
    let props = Props {
        key: "".to_string(),
        code_ref: NodeRef::default(),
        class_name: "form-test".to_string(),
        id: "form-test-id".to_string(),
        onsubmit_signal: Callback::noop(),
        method: Method::Post,
        action: "".to_string(),
        name: "form-test".to_string(),
        children: Children::new(vec![html! {<input id="result"/>}]),
    };

    let form_component: App<Form> = App::new();

    form_component.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let form_element = utils::document().get_element_by_id("result").unwrap();

    assert_eq!(form_element.tag_name(), "INPUT");
}

#[wasm_bindgen_test]
fn should_submit_the_form() {
    let body = utils::document().body().unwrap();

    let element = utils::document().create_element("div").unwrap();
    element.set_text_content(Some("fill the form"));
    element.set_id("form");

    body.append_child(&element).unwrap();

    let onsubmit = Callback::from(|_| {
        let content = utils::document().get_element_by_id("form").unwrap();

        content.set_text_content(Some("form submitted"));
    });

    let props = Props {
        key: "".to_string(),
        code_ref: NodeRef::default(),
        class_name: "form-test".to_string(),
        id: "form-test-id".to_string(),
        onsubmit_signal: onsubmit,
        method: Method::Post,
        action: "".to_string(),
        name: "form-test".to_string(),
        children: Children::new(vec![html! {<input/>}]),
    };

    let focus_event = FocusEvent::new("Submit").unwrap();

    props.onsubmit_signal.emit(focus_event);

    let form_element = utils::document().get_element_by_id("form").unwrap();

    assert_eq!(
        form_element.text_content().unwrap(),
        "form submitted".to_string()
    );
}
