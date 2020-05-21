use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

pub struct FormLabel {
    pub props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class_name: String,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub label_for: String,
    #[prop_or_default]
    pub text: String,
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
        self.props = props;

        true
    }

    fn view(&self) -> Html {
        html! {
            <label
                class=format!("form-label {}", self.props.class_name)
                id=self.props.id
                for=self.props.label_for
            >{self.props.text.clone()}</label>
        }
    }
}

#[wasm_bindgen_test]
fn should_create_form_label() {
    let props = Props {
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
