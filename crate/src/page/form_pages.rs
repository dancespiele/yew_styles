use super::highlighters::{input_code, select_code, textarea_code};
use crate::app::AppRouter;
use wasm_bindgen::JsCast;
use web_sys::HtmlOptionElement;
use yew::prelude::*;
use yew_prism::Prism;
use yew_router::prelude::*;
use yew_styles::forms::{
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
use yew_styles::styles::{Palette, Size};
pub struct FormPage {
    pub link: ComponentLink<Self>,
    pub value: Vec<String>,
    pub multiple_values: Vec<String>,
}

pub enum Msg {
    Input(String, usize),
    Select(String, usize),
    MultipleSelect(Vec<String>),
}

impl Component for FormPage {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        FormPage {
            link,
            value: vec!["".to_string(); 6],
            multiple_values: vec![],
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(value, index) => {
                self.value[index] = value;
            }
            Msg::Select(value, index) => {
                self.value[index] = value;
            }
            Msg::MultipleSelect(values) => {
                self.multiple_values = values;
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
                    <h2>{"Form"}</h2>
                    <ul>
                        <li><b>{"onsubmit_signal: "}</b>{"signal to emit the event submit."}</li>
                        <li><b>{"action: "}</b>{"the URL that processes the form submission."}</li>
                        <li><b>{"method: "}</b>{"the HTTP method to submit the form. Options included in "}<code>{"Method"}</code>
                            {". Default "}<code>{"Post"}</code>{"."}</li>
                        <li><b>{"name: "}</b>{"the name of the form."}</li>
                        <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                        <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>
                    </ul>

                    <p><b>{"Note: "}</b>{"the component triggers automatically "}
                        <code>{"prevent_default"}</code>{" once that the submit event is actined."}</p>
                    <p>{"The code example is in "}<RouterAnchor<AppRouter> route=AppRouter::BasicFormPath>{"Basic Form page"}</RouterAnchor<AppRouter>></p>
                </Item>

                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h2>{"Form group"}</h2>
                    <ul>
                        <li><b>{"method: "}</b>{"in which orientation will show the inputs, select and labels. Options included in "}
                            <code>{"Orientation"}</code>{". Default "}<code>{"Vertical"}</code>{"."}</li>
                        <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                        <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>
                    </ul>

                    <p>{"The code example is in "}<RouterAnchor<AppRouter> route=AppRouter::BasicFormPath>{"Basic Form page"}</RouterAnchor<AppRouter>></p>
                </Item>

                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h2>{"Form input types"}</h2>
                    <Prism
                        code=input_code()
                        language="rust"
                    />
                    <ul>
                        <li><b>{"value: "}</b>{"current value of the form control. Required."}</li>
                        <li><b>{"input_type: "}</b>{"the input type. Options included in "}<code>{"InputType"}</code>
                            {". Default "}<code>{"Text"}</code>{"."}</li>
                        <li><b>{"name: "}</b>{"the name of the input."}</li>
                        <li><b>{"input_style: "}</b>{"the input style according with the purpose. Options included in "}<code>{"Palette"}</code>
                            {". Default "}<code>{"Standard"}</code>{"."}</li>
                        <li><b>{"input_size: "}</b>{"the size of the input. Options included in "}<code>{"Size"}</code>
                            {". Default "}<code>{"Medium"}</code>{"."}</li>
                        <li><b>{"oninput_signal: "}</b>{"signal to emit the event input."}</li>
                        <li><b>{"onblur_signal: "}</b>{"signal to emit the event blur."}</li>
                        <li><b>{"onkeypress_signal: "}</b>{"signal to emit the event keypress."}</li>
                        <li><b>{"onchange_signal: "}</b>{"signal to emit the event change."}</li>
                        <li><b>{"placeholder: "}</b>{"content to be appear in the form control when the form control is empty."}</li>
                        <li><b>{"checked: "}</b>{"whether the command or control is checked."}</li>
                        <li><b>{"accept: "}</b>{"hint for expected file type in file upload controls."}</li>
                        <li><b>{"alt: "}</b>{"alt attribute for the image type. Required for accessibiltiy."}</li>
                        <li><b>{"autofocus: "}</b>{"automatically focus the form control when the page is loaded."}</li>
                        <li><b>{"autocomplete: "}</b>{"hint for form autofill feature."}</li>
                        <li><b>{"list: "}</b>{"value of the id attribute of the"}<code>{"<datalist>"}</code>{" of autocomplete options."}</li>
                        <li><b>{"min: "}</b>{"minimum value."}</li>
                        <li><b>{"max: "}</b>{"maximum value."}</li>
                        <li><b>{"minlength: "}</b>{"minimum length (number of characters) of value"}</li>
                        <li><b>{"maxlength: "}</b>{"maximum length (number of characters) of value. Default 1000"}</li>
                        <li><b>{"pattern: "}</b>{"pattern the value must match to be valid."}</li>
                        <li><b>{"readonly: "}</b>{"boolean. The value is not editable."}</li>
                        <li><b>{"required: "}</b>{"boolean. A value is required or must be check for the form to be submittable."}</li>
                        <li><b>{"disabled: "}</b>{"whether the form control is disabled."}</li>
                        <li><b>{"multiple: "}</b>{"boolean. Whether to allow multiple values."}</li>
                        <li><b>{"underline: "}</b>{"underline style instead of box, like Material."}</li>
                        <li><b>{"capture: "}</b>{"media capture input method in file upload controls."}</li>
                        <li><b>{"step: "}</b>{"incremental values that are valid."}</li>
                        <li><b>{"error_state: "}</b>{"error state for validation."}</li>
                        <li><b>{"error_message: "}</b>{"show error message when error_state is true."}</li>
                        <li><b>{"id: "}</b>{"general property to add custom id."}</li>
                        <li><b>{"class_name: "}</b>{"general property to add custom class styles."}</li>
                    </ul>

                    {get_form_inputs(self)}
                </Item>

                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h2>{"Form select types"}</h2>
                    <Prism
                        code=select_code()
                        language="rust"
                    />
                    <ul>
                        <li><b>{"name: "}</b>{"the name of the input."}</li>
                        <li><b>{"select_size: "}</b>{"the size of the select. Options included in "}<code>{"Size"}</code>
                            {". Default "}<code>{"Medium"}</code>{"."}</li>
                        <li><b>{"options: "}</b>{"different options to select. Required."}</li>
                        <li><b>{"onchange_signal: "}</b>{"signal to emit the event change. Required."}</li>
                        <li><b>{"autofocus: "}</b>{"automatically focus the form control when the page is loaded."}</li>
                        <li><b>{"required: "}</b>{"boolean. A value is required or must be check for the form to be submittable."}</li>
                        <li><b>{"disabled: "}</b>{"whether the form control is disabled."}</li>
                        <li><b>{"multiple: "}</b>{"boolean. Whether to allow multiple values."}</li>
                        <li><b>{"size: "}</b>{"If the control is presented as a scrolling list box,
                            this attribute represents the number of rows in the list that should be visible at one time."}</li>
                        <li><b>{"error_state: "}</b>{"error state for validation."}</li>
                        <li><b>{"error_message: "}</b>{"show error message when error_state is true."}</li>
                        <li><b>{"id: "}</b>{"general property to add custom id."}</li>
                        <li><b>{"class_name: "}</b>{"general property to add custom class styles."}</li>
                    </ul>
                    {get_select_form(self)}
                </Item>

                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h2>{"Form textarea types"}</h2>
                    <Prism
                        code=textarea_code()
                        language="rust"
                    />

                    <ul>
                        <li><b>{"value: "}</b>{"current value of the form control. Required."}</li>
                        <li><b>{"textarea_type: "}</b>{"the textarea type. Options included in "}<code>{"InputType"}</code>
                            {". Default "}<code>{"Text"}</code>{"."}</li>
                        <li><b>{"name: "}</b>{"the name of the textarea."}</li>
                        <li><b>{"textarea_style: "}</b>{"the input style according with the purpose. Options included in "}<code>{"Palette"}</code>
                            {". Default "}<code>{"Standard"}</code>{"."}</li>
                        <li><b>{"textarea_size: "}</b>{"the size of the input. Options included in "}<code>{"Size"}</code>
                            {". Default "}<code>{"Medium"}</code>{"."}</li>
                        <li><b>{"oninput_signal: "}</b>{"signal to emit the event input."}</li>
                        <li><b>{"onblur_signal: "}</b>{"signal to emit the event blur."}</li>
                        <li><b>{"onkeypress_signal: "}</b>{"signal to emit the event keypress."}</li>
                        <li><b>{"placeholder: "}</b>{"content to be appear in the form control when the form control is empty."}</li>
                        <li><b>{"autofocus: "}</b>{"automatically focus the form control when the page is loaded."}</li>
                        <li><b>{"autocomplete: "}</b>{"hint for form autofill feature."}</li>
                        <li><b>{"minlength: "}</b>{"minimum length (number of characters) of value"}</li>
                        <li><b>{"maxlength: "}</b>{"maximum length (number of characters) of value. Default 1000"}</li>
                        <li><b>{"readonly: "}</b>{"boolean. The value is not editable."}</li>
                        <li><b>{"required: "}</b>{"boolean. A value is required or must be check for the form to be submittable."}</li>
                        <li><b>{"disabled: "}</b>{"whether the form control is disabled."}</li>
                        <li><b>{"wrap: "}</b>{"indicates how the control wraps text. Options included in "}<code>{"WrapText"}</code>
                            {". Default "}<code>{"Soft"}</code>{"."}</li>
                        <li><b>{"cols: "}</b>{"the visible width of the text control."}</li>
                        <li><b>{"rows: "}</b>{"the number of visible text lines for the control."}</li>
                        <li><b>{"spellcheck: "}</b>{"specifies whether the "}<code>{"<textarea>"}</code>
                            {" is subject to spell checking by the underlying browser/OS."}</li>
                        <li><b>{"error_state: "}</b>{"error state for validation."}</li>
                        <li><b>{"error_message: "}</b>{"show error message when error_state is true."}</li>
                        <li><b>{"id: "}</b>{"general property to add custom id."}</li>
                        <li><b>{"class_name: "}</b>{"general property to add custom class styles."}</li>
                    </ul>
                    {get_textarea(self)}
                </Item>
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
                        value=form_page.value[2].clone()
                        input_style=Palette::Success
                        oninput_signal = form_page.link.callback(|e: InputData| Msg::Input(e.value, 2))
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

fn get_select_form(form_page: &FormPage) -> Html {
    html! {
        <Container wrap = Wrap::Wrap direction = Direction::Row>
            <Item layouts=vec!(ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                <FormGroup>
                    <FormLabel
                        text="Standard select"
                    />
                    <FormSelect
                        select_size=Size::Medium
                        onchange_signal = form_page.link.callback(|e: ChangeData|
                            match e {
                                ChangeData::Select(element) => {
                                    let value = element.value();
                                    Msg::Select(value, 3)
                                },
                                _ => unreachable!(),
                            }
                        )
                        options=html!{
                            <>
                                <option value="" disabled=true>{"Select library"}</option>
                                <option value="yew">{"Yew"}</option>
                                <option value="yew_styles">{"Yew Styles"}</option>
                                <option value="yew_prism">{"Yew prism"}</option>
                            </>
                        }
                    />
                    <div>{format!("Value: {}", form_page.value[3].clone())}</div>
                </FormGroup>
            </Item>
            <Item layouts=vec!(ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                <FormGroup>
                    <FormLabel
                        text="Multiple select"
                    />
                    <FormSelect
                        select_size=Size::Medium
                        multiple=true
                        onchange_signal = form_page.link.callback(|e: ChangeData| {
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
                                    Msg::MultipleSelect(values)
                                },
                                _ => unreachable!(),
                            }
                        })
                        options=html!{
                            <>
                                <option value="" disabled=true>{"Select multiple library"}</option>
                                <option value="yew">{"Yew"}</option>
                                <option value="yew_styles">{"Yew Styles"}</option>
                                <option value="yew_prism">{"Yew prism"}</option>
                            </>
                        }
                    />
                    <div>{format!("Value: {:#?}", form_page.multiple_values.clone())}</div>
                </FormGroup>
            </Item>
        </Container>
    }
}

fn get_textarea(form_page: &FormPage) -> Html {
    html! {
        <Container wrap = Wrap::Wrap direction = Direction::Row>
            <Item layouts=vec!(ItemLayout::ItL(6), ItemLayout::ItXs(12))>
                <FormGroup orientation=Orientation::Vertical>
                        <FormLabel
                            text="Standard textarea"
                        />
                        <FormTextArea placeholder="write here"
                            value=form_page.value[4].clone()
                            textarea_size=Size::Medium
                            oninput_signal=form_page.link.callback(|e: InputData| Msg::Input(e.value, 4))
                        />

                    <div>{format!("Value: {}", form_page.value[4].clone())}</div>
                </FormGroup>
            </Item>
            <Item layouts=vec!(ItemLayout::ItL(6), ItemLayout::ItXs(12))>
                <FormGroup orientation=Orientation::Vertical>
                        <FormLabel
                            text="Info small textarea"
                        />
                        <FormTextArea placeholder="write here"
                            value=form_page.value[5].clone()
                            textarea_size=Size::Small
                            textarea_style=Palette::Info
                            oninput_signal=form_page.link.callback(|e: InputData| Msg::Input(e.value, 5))
                        />

                    <div>{format!("Value: {}", form_page.value[5].clone())}</div>
                </FormGroup>
            </Item>
        </Container>
    }
}
