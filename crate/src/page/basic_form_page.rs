use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{HtmlOptionElement, HtmlSelectElement};
use yew::prelude::*;
use yew::utils;
use yew_styles::forms::{
    form_component::Form,
    form_group::{FormGroup, Orientation},
    form_input::{FormInput, InputType},
    form_label::FormLabel,
    form_select::FormSelect,
    form_submit::FormSubmit,
    form_textarea::FormTextArea,
};
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::{Palette, Style};

#[derive(Clone)]
struct Fields {
    first_name: String,
    last_name: String,
    email: String,
    specialty: String,
    skills: Vec<String>,
    cover_letter: String,
}

pub struct BasicFormPage {
    link: ComponentLink<Self>,
    fields: HashMap<String, String>,
    skills: Vec<String>,
    empty_fields: Vec<String>,
    result: Option<Fields>,
}

pub enum Msg {
    FirstName(String),
    LastName(String),
    Email(String),
    Specialty(String),
    Skills(Vec<String>),
    CoverLetter(String),
    Submit,
}

impl Component for BasicFormPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fields: HashMap::new(),
            skills: vec![],
            empty_fields: vec![],
            result: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FirstName(first_name) => {
                self.fields.insert("first_name".to_string(), first_name);
            }
            Msg::LastName(last_name) => {
                self.fields.insert("last_name".to_string(), last_name);
            }
            Msg::Email(email) => {
                self.fields.insert("email".to_string(), email);
            }
            Msg::Specialty(specialty) => {
                self.fields.insert("specialty".to_string(), specialty);
            }
            Msg::Skills(skills) => {
                self.skills = skills;
            }
            Msg::CoverLetter(cover_letter) => {
                self.fields.insert("cover_letter".to_string(), cover_letter);
            }
            Msg::Submit => {
                self.empty_fields = get_empty_fields(self.fields.clone());

                if self.empty_fields.is_empty() {
                    self.result = Some(Fields {
                        first_name: self.fields.get("first_name").unwrap().to_string(),
                        last_name: self.fields.get("last_name").unwrap().to_string(),
                        email: self.fields.get("email").unwrap().to_string(),
                        specialty: self.fields.get("specialty").unwrap().to_string(),
                        skills: self.skills.clone(),
                        cover_letter: self.fields.get("cover_letter").unwrap().to_string(),
                    });
                    self.fields.drain();

                    self.skills = vec![];

                    set_default_selected("specialty");
                    remove_all_selected("skills");
                }
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Container wrap=Wrap::Wrap direction=Direction::Row>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h1>{"Basic Form"}</h1>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <Form onsubmit_signal=self.link.callback(|e| Msg::Submit)>
                        <Container wrap=Wrap::Wrap direction=Direction::Row>
                            <Item layouts=vec!(ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                                <FormGroup orientation=Orientation::Horizontal>
                                    <FormLabel text="First name: "/>
                                    <FormInput
                                        value=match self.fields.get("first_name") {
                                            Some(value) => value,
                                            None => ""
                                        }
                                        error_state=self.empty_fields.iter().any(|field| field == "first_name")
                                        error_message="First name field is required"
                                        input_type=InputType::Text
                                        oninput_signal=self.link.callback(|e: InputData| Msg::FirstName(e.value))
                                    />
                                </FormGroup>
                                <FormGroup orientation=Orientation::Horizontal>
                                    <FormLabel text="Last name: "/>
                                    <FormInput
                                        value=match self.fields.get("last_name") {
                                            Some(value) => value,
                                            None => ""
                                        }
                                        error_state=self.empty_fields.iter().any(|field| field == "last_name")
                                        error_message="Last name field is required"
                                        input_type=InputType::Text
                                        oninput_signal=self.link.callback(|e: InputData| Msg::LastName(e.value))
                                    />
                                </FormGroup>
                                <FormGroup orientation=Orientation::Horizontal>
                                    <FormLabel text="Email: "/>
                                    <FormInput
                                        value=match self.fields.get("email") {
                                            Some(value) => value,
                                            None => ""
                                        }
                                        error_state=self.empty_fields.iter().any(|field| field == "email")
                                        error_message="Email field is required"
                                        input_type=InputType::Email
                                        oninput_signal=self.link.callback(|e: InputData| Msg::Email(e.value))
                                    />
                                </FormGroup>
                            </Item>
                            <Item layouts=vec!(ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                                <FormGroup orientation=Orientation::Vertical>
                                    <FormLabel text="Specialty:"/>
                                    <FormSelect
                                        id="specialty"
                                        error_state=self.empty_fields.iter().any(|field| field == "specialty")
                                        error_message="Select specialty is required"
                                        onchange_signal=self.link.callback(|e: ChangeData| {
                                            match e {
                                                ChangeData::Select(element) => {
                                                    let value = element.value();
                                                    Msg::Specialty(value)
                                                },
                                                _ => unreachable!()
                                            }
                                        })
                                        options=html!{
                                            <>
                                            <option value="" disabled=true>{"Choose specialty"}</option>
                                                <option value="frontend">{"Frontend"}</option>
                                                <option value="backend">{"Backend"}</option>
                                            </>
                                        }
                                    />
                                </FormGroup>
                                <FormGroup orientation=Orientation::Vertical>
                                    <FormLabel text="Skills:"/>
                                    <FormSelect
                                        id="skills"
                                        multiple=true
                                        onchange_signal=self.link.callback(|e: ChangeData| {
                                            match e {
                                                ChangeData::Select(element) => {
                                                    let mut values = vec![];
                                                    let options = element.options();

                                                    for i in 0..options.length() {
                                                        let option = options
                                                            .get_with_index(i)
                                                            .unwrap()
                                                            .dyn_into::<HtmlOptionElement>()
                                                            .unwrap();
                                                        if option.selected() {
                                                            values.push(option.value());
                                                        }
                                                    }
                                                    Msg::Skills(values)
                                                },
                                                _ => unreachable!()
                                            }
                                        })
                                        options=html!{
                                            <>
                                                <option value="yew">{"Yew.rs"}</option>
                                                <option value="rustwasm">{"Rustwasm"}</option>
                                                <option value="rust">{"Rust"}</option>
                                                <option value="warp">{"Warp"}</option>
                                                <option value="tokio">{"Tokio"}</option>
                                            </>
                                        }
                                    />
                                </FormGroup>
                            </Item>
                            <Item layouts=vec!(ItemLayout::ItXs(12))>
                                <FormGroup orientation=Orientation::Vertical>
                                    <FormLabel text="Cover letter:"/>
                                    <FormTextArea
                                        value=match self.fields.get("cover_letter") {
                                            Some(value) => value,
                                            None => ""
                                        }
                                        error_state=self.empty_fields.iter().any(|field| field == "cover_letter")
                                        error_message="cover letter is required"
                                        oninput_signal=self.link.callback(|e: InputData| Msg::CoverLetter(e.value))/>
                                </FormGroup>
                            </Item>
                            <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItM(3))>
                                <FormGroup>
                                <FormSubmit
                                    value="Submit application"
                                    submit_type=Palette::Success
                                    submit_style=Style::Outline
                                />
                                </FormGroup>
                            </Item>
                        </Container>
                    </Form>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    {get_result(self.result.clone())}
                </Item>
            </Container>
        }
    }
}

fn get_result(result: Option<Fields>) -> Html {
    if let Some(form) = result {
        html! {
            <Container wrap=Wrap::Wrap direction=Direction::Row>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <p><b>{"First name: "}</b>{form.first_name.clone()}</p>
                    <p><b>{"last name: "}</b>{form.last_name.clone()}</p>
                    <p><b>{"email: "}</b>{form.email.clone()}</p>
                    <p><b>{"Specialty: "}</b>{form.specialty.clone()}</p>
                    <p><b>{"Skills: "}</b>{form.skills.join(", ")}</p>
                    <p><b>{"Cover letter: "}</b>{form.cover_letter}</p>
                </Item>
            </Container>
        }
    } else {
        html! {}
    }
}

fn get_empty_fields(fields: HashMap<String, String>) -> Vec<String> {
    let total_fields = vec![
        "first_name",
        "last_name",
        "email",
        "specialty",
        "cover_letter",
    ];
    let mut empty_fields: Vec<String> = vec![];

    for field in total_fields {
        let is_filled = fields
            .iter()
            .any(|(key, value)| key == field && !value.is_empty());
        if !is_filled {
            empty_fields.push(field.to_string());
        }
    }

    empty_fields
}

fn remove_all_selected(select: &str) {
    let specialty_form_element = utils::document()
        .get_element_by_id(select)
        .unwrap()
        .dyn_into::<HtmlSelectElement>()
        .unwrap();
    let specialty_options = specialty_form_element.options();

    for i in 0..specialty_options.length() {
        let option = specialty_options
            .get_with_index(i)
            .unwrap()
            .dyn_into::<HtmlOptionElement>()
            .unwrap();

        option.set_selected(false);
    }
}

fn set_default_selected(select: &str) {
    let specialty_form_element = utils::document()
        .get_element_by_id(select)
        .unwrap()
        .dyn_into::<HtmlSelectElement>()
        .unwrap();
    let specialty_options = specialty_form_element.options();

    let option = specialty_options
        .get_with_index(0)
        .unwrap()
        .dyn_into::<HtmlOptionElement>()
        .unwrap();

    option.set_selected(true);
}
