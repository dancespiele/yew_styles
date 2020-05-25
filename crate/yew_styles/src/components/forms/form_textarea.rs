use crate::styles::{get_pallete, get_size, Palette, Size};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

pub struct FormTextArea {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone)]
pub enum WrapText {
    Hard,
    Soft,
    Off,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class_name: String,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or(Palette::Standard)]
    pub textarea_style: Palette,
    #[prop_or(Size::Medium)]
    pub textarea_size: Size,
    #[prop_or(1000)]
    pub maxlength: u32,
    #[prop_or_default]
    pub minlength: u16,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub autofocus: bool,
    #[prop_or_default]
    pub autocomplete: bool,
    #[prop_or_default]
    pub cols: u16,
    #[prop_or_default]
    pub rows: u16,
    #[prop_or_default]
    pub spellcheck: bool,
    #[prop_or(Callback::noop())]
    pub oninput_signal: Callback<InputData>,
    #[prop_or(Callback::noop())]
    pub onblur_signal: Callback<FocusEvent>,
    #[prop_or(Callback::noop())]
    pub onkeypress_signal: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub error_state: bool,
    #[prop_or_default]
    pub error_message: String,
    #[prop_or(WrapText::Soft)]
    pub wrap: WrapText,
}

#[derive(Debug)]
pub enum Msg {
    Input(InputData),
    Blur(FocusEvent),
    KeyPressed(KeyboardEvent),
}

impl Component for FormTextArea {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(input_data) => {
                self.props.oninput_signal.emit(input_data);
            }
            Msg::Blur(focus_event) => {
                self.props.onblur_signal.emit(focus_event);
            }
            Msg::KeyPressed(keyboard_event) => {
                self.props.onkeypress_signal.emit(keyboard_event);
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
                <textarea
                    id=self.props.id
                    class=format!("form-textarea {} {} {} {}",
                    self.props.class_name,
                    get_pallete(self.props.textarea_style.clone()),
                    get_size(self.props.textarea_size.clone()),
                    if self.props.error_state { "form-error" } else { "" })
                    oninput=self.link.callback(|input_data| Msg::Input(input_data))
                    onblur=self.link.callback(|focus_event| Msg::Blur(focus_event))
                    onkeypress=self.link.callback(|keyboard_event| Msg::KeyPressed(keyboard_event))
                    name=self.props.name
                    autocomplete=self.props.autocomplete
                    autofocus=self.props.autofocus
                    required=self.props.required
                    readonly=self.props.readonly
                    disabled=self.props.disabled
                    rows=self.props.rows
                    placeholder=self.props.placeholder
                    cols=self.props.cols
                    spellcheck=self.props.spellcheck
                    minlength=self.props.minlength
                    maxlength=self.props.maxlength
                    warp=get_wrap(self.props.wrap.clone())
                />
                {get_error_message(self.props.error_state, self.props.error_message.clone())}
            </>
        }
    }
}

fn get_error_message(error_state: bool, error_message: String) -> Html {
    if error_state {
        html! {<span class="form-input-error">{error_message}</span>}
    } else {
        html! {}
    }
}

fn get_wrap(wrap_text: WrapText) -> String {
    match wrap_text {
        WrapText::Hard => "hard".to_string(),
        WrapText::Off => "soft".to_string(),
        WrapText::Soft => "off".to_string(),
    }
}

#[wasm_bindgen_test]
fn should_create_form_textarea() {
    let props = Props {
        id: "form-input-id-test".to_string(),
        class_name: "form-input-class-test".to_string(),
        oninput_signal: Callback::noop(),
        onblur_signal: Callback::noop(),
        onkeypress_signal: Callback::noop(),
        error_message: "invalid input".to_string(),
        error_state: false,
        name: "input-test".to_string(),
        textarea_style: Palette::Standard,
        textarea_size: Size::Medium,
        placeholder: "test input".to_string(),
        required: false,
        autocomplete: false,
        autofocus: false,
        maxlength: 100,
        minlength: 0,
        readonly: false,
        disabled: false,
        cols: 20,
        rows: 10,
        spellcheck: true,
        wrap: WrapText::Hard,
    };

    let form_textarea: App<FormTextArea> = App::new();

    form_textarea.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let form_textarea_element = utils::document()
        .get_element_by_id("form-input-id-test")
        .unwrap();

    assert_eq!(form_textarea_element.tag_name(), "TEXTAREA");
}
