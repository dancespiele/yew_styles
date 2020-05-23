use crate::styles::{get_size, Size};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App, ChangeData};

pub struct FormSelect {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub options: Html,
    pub on_change_signal: Callback<ChangeData>,
    /// Whether or not the selector should be disabled.
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or(Size::Medium)]
    pub select_size: Size,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub multiple: bool,
    #[prop_or_default]
    pub size: u16,
    #[prop_or_default]
    pub autofocus: bool,
    #[prop_or_default]
    pub class_name: String,
    #[prop_or_default]
    pub id: String,
}

pub enum Msg {
    Selected(ChangeData),
}

impl Component for FormSelect {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Selected(value) => {
                self.props.on_change_signal.emit(value);
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <select
                class=format!("form-select {} {}", get_size(self.props.select_size.clone()), self.props.class_name)
                disabled=self.props.disabled,
                name=self.props.name,
                autofocus=self.props.autofocus,
                required=self.props.required,
                multiple=self.props.multiple,
                size=self.props.size,
                onchange=self.link.callback(|change_data| Msg::Selected(change_data))
            >

            {self.props.options.clone()}
            </select>
        }
    }
}

#[wasm_bindgen_test]
fn should_create_form_select() {
    let props = Props {
        on_change_signal: Callback::noop(),
        id: "form-select-id-test".to_string(),
        class_name: "form-select-class-test".to_string(),
        disabled: false,
        autofocus: false,
        required: false,
        select_size: Size::Medium,
        size: 0,
        name: "options".to_string(),
        multiple: false,
        options: html! {
            <>
                <option value="value-1" selected=true>{"option 1"}</option>
                <option value="value-2">{"option 2"}</option>
                <option value="value-3">{"option 3"}</option>
                <option value="value-4" id="result">{"option 4"}</option>
            </>
        },
    };

    let form_select: App<FormSelect> = App::new();
    form_select.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let form_select_element = utils::document().get_element_by_id("result").unwrap();

    assert_eq!(
        form_select_element.text_content().unwrap(),
        "option 4".to_string()
    );
}
