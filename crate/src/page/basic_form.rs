use std::ops::Index;
use wasm_bindgen::JsCast;
use web_sys::HtmlOptionElement;
use yew::prelude::*;
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

pub struct BasicForm {
    link: ComponentLink<Self>,
    fields: Fields,
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

enum FieldIndex {
    FirstName,
    LastName,
    Email,
    Specialty,
    Skills,
    CoverLetter,
    Empty,
}

impl Index<FieldIndex> for Fields {
    type Output = String;

    fn index(&self, fields_name: FieldIndex) -> &Self::Output {
        match fields_name {
            FieldIndex::FirstName => &self.first_name,
            FieldIndex::LastName => &self.last_name,
            FieldIndex::Email => &self.email,
            FieldIndex::Specialty => &self.specialty,
            FieldIndex::Skills => &self.skills.clone().join(", "),
            FieldIndex::CoverLetter => &self.cover_letter,
            FieldIndex::Empty => &"".to_string(),
        }
    }
}

impl Fields {
    fn new() -> Self {
        Fields {
            first_name: "".to_string(),
            last_name: "".to_string(),
            email: "".to_string(),
            specialty: "".to_string(),
            skills: vec![],
            cover_letter: "".to_string(),
        }
    }

    fn get_field_index(&self, field_name: &str) -> FieldIndex {
        match field_name {
            "first_name" => FieldIndex::FirstName,
            "last_name" => FieldIndex::LastName,
            "email" => FieldIndex::Email,
            "specialty" => FieldIndex::Specialty,
            "skills" => FieldIndex::Skills,
            "cover_letter" => FieldIndex::CoverLetter,
            _ => FieldIndex::Empty,
        }
    }

    fn checkEmptyField(&self) -> Vec<String> {
        let fields = vec![
            "first_name",
            "last_name",
            "email",
            "specialty",
            "skills",
            "cover_letter",
        ];
        let mut empty_fields: Vec<String> = vec![];

        for field in fields {
            let field_index = self.get_field_index(field);
            if self[field_index].is_empty() {
                empty_fields.push(field.to_string());
            }
        }

        empty_fields
    }
}

impl Component for BasicForm {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fields: Fields::new(),
            result: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FirstName(first_name) => {
                self.fields.first_name = first_name;
            }
            Msg::LastName(last_name) => {
                self.fields.last_name = last_name;
            }
            Msg::Email(email) => {
                self.fields.email = email;
            }
            Msg::Specialty(specialty) => {
                self.fields.specialty = specialty;
            }
            Msg::Skills(skills) => {
                self.fields.skills = skills;
            }
            Msg::CoverLetter(cover_letter) => {
                self.fields.cover_letter = cover_letter;
            }
            Msg::Submit => {
                self.result = Some(self.fields.clone());
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <Form onsubmit_signal=self.link.callback(|e| Msg::Submit)>
                <Container wrap=Wrap::Wrap direction=Direction::Row>
                    <Item layouts=vec!(ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                        <FormGroup orientation=Orientation::Horizontal>
                            <FormLabel text="First name: "/>
                            <FormInput
                                value=self.fields.first_name.clone()
                                input_type=InputType::Text
                                oninput_signal=self.link.callback(|e: InputData| Msg::FirstName(e.value))
                            />
                        </FormGroup>
                        <FormGroup orientation=Orientation::Horizontal>
                            <FormLabel text="Last name: "/>
                            <FormInput
                                value=self.fields.last_name.clone()
                                input_type=InputType::Text
                                oninput_signal=self.link.callback(|e: InputData| Msg::LastName(e.value))
                            />
                        </FormGroup>
                        <FormGroup orientation=Orientation::Horizontal>
                            <FormLabel text="Email: "/>
                            <FormInput
                                value=self.fields.email.clone()
                                input_type=InputType::Email
                                oninput_signal=self.link.callback(|e: InputData| Msg::Email(e.value))
                            />
                        </FormGroup>
                    </Item>
                    <Item layouts=vec!(ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                        <FormGroup orientation=Orientation::Vertical>
                            <FormLabel text="Specialty:"/>
                            <FormSelect
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
            {get_result(self.result.clone())}
            </>
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
                    <p><b>{"Skills: "}</b>{form.skills.clone().into_iter().enumerate().map(|(index, skill)| {
                        format!("{}{}", skill, if index == form.skills.len() -1 { "" } else {", "})
                    }).collect::<Html>()}</p>
                    <p><b>{"Cover letter: "}</b>{form.cover_letter}</p>
                </Item>
            </Container>
        }
    } else {
        html! {}
    }
}

// struct Data {
//     required: f64
//     optional: Option<f64>
//   }
//   struct DataBuilder {
//     required: Option<f64>
//     optional: Option<f64>
//   }
//   impl DataBuilder {
//     fn build(self) -> Result<Data,(Self, &'static str)> {
//         if self.required.is_none() {
//             return Err(self, "field require missing")
//         }
//         Data {
//             required: self.required.unwrap(), // we checked before!
//             optional: self.optional // Option ok here
//         }
//     }
//   }
