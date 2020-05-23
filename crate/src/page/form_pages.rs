use yew::prelude::*;
use yew_styles::forms::{
    form_group::{FormGroup, Orientation},
    form_input::{FormInput, InputType},
    form_label::FormLabel,
};
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::{Palette, Size};

pub struct FormPage {
    pub link: ComponentLink<Self>,
    pub value: Vec<String>,
}

pub enum Msg {
    Input(String, usize),
}

impl Component for FormPage {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        FormPage {
            link,
            value: vec!["".to_string(); 50],
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(value, index) => {
                self.value[index] = value;
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
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h1>{"Form Component"}</h1>
                </Item>

                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h2>{"Form input types"}</h2>
                </Item>

                {get_form_inputs(self)}
            </>
        }
    }
}

fn get_form_inputs(form_page: &FormPage) -> Html {
    html! {
        <Container wrap=Wrap::Wrap direction=Direction::Row>
            <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                <FormGroup orientation=Orientation::Horizontal>
                    <FormLabel
                        text="standard input"
                    />
                    <FormInput
                        input_type=InputType::Text
                        value=form_page.value[0].clone()
                        input_style=Palette::Standard
                        input_size=Size::Medium
                        id="form-input-test"
                        oninput_signal = form_page.link.callback(|e: InputData| Msg::Input(e.value, 0))
                        placeholder="test"
                        underline=false
                    />
                    <div>{format!("Value: {}", form_page.value[0].clone())}</div>
                </FormGroup>
            </Item>
            <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                <FormGroup orientation=Orientation::Horizontal>
                    <FormLabel
                        text="underline input"
                    />
                    <FormInput
                        input_type=InputType::Text
                        value=form_page.value[1].clone()
                        oninput_signal = form_page.link.callback(|e: InputData| Msg::Input(e.value, 1))
                        input_style=Palette::Standard
                        input_size=Size::Medium
                        id="form-input-test"
                        placeholder="test"
                        underline=true
                    />
                    <div>{format!("Value: {}", form_page.value[1].clone())}</div>
                </FormGroup>
            </Item>
            <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                <FormGroup orientation=Orientation::Horizontal>
                    <FormLabel
                        text="Success input type"
                    />
                    <FormInput
                        input_type=InputType::Text
                        value=form_page.value[1].clone()
                        oninput_signal = form_page.link.callback(|e: InputData| Msg::Input(e.value, 2))
                        input_style=Palette::Success
                        input_size=Size::Medium
                        id="form-input-test"
                        placeholder="test"
                        underline=false
                    />
                    <div>{format!("Value: {}", form_page.value[2].clone())}</div>
                </FormGroup>
            </Item>
        </Container>
    }
}
