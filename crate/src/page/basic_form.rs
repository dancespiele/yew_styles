use wasm_bindgen::JsCast;
use web_sys::HtmlOptionElement;
use yew::prelude::*;
use yew_styles::forms::{
    form_component::Form,
    form_group::{FormGroup, Orientation},
    form_input::{FormInput, InputType},
    form_label::FormLabel,
    form_select::FormSelect,
    form_textarea::FormTextArea,
};
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::{Palette, Size, Style};

#[derive(Clone)]
struct Fields {
    first_name: String,
    last_name: String,
    email: String,
    developer: String,
    skills: Vec<String>,
    cover_letter: String,
}

pub struct BasicForm {
    link: ComponentLink<Self>,
    fields: Fields,
}

pub enum Msg {
    FirstName(String),
    LastName(String),
    Email(String),
    Developer(String),
    Skills(Vec<String>),
    CoverLetter(String),
}

impl Component for BasicForm {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fields: Fields {
                first_name: "".to_string(),
                last_name: "".to_string(),
                email: "".to_string(),
                developer: "".to_string(),
                skills: vec![],
                cover_letter: "".to_string(),
            },
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
            Msg::Developer(developer) => {
                self.fields.developer = developer;
            }
            Msg::Skills(skills) => {
                self.fields.skills = skills;
            }
            Msg::CoverLetter(cover_letter) => {
                self.fields.cover_letter = cover_letter;
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Form>
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
                                            Msg::Developer(value)
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
                </Container>
            </Form>
        }
    }
}
