use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

pub struct Form {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    // get all the values from the form
    pub onsignal: Callback<()>,
    pub children: Children,
    #[prop_or_default]
    pub class_name: String,
    #[prop_or_default]
    pub id: String,
}

pub enum Msg {
    Submitted,
}

impl Component for Form {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Form { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submitted => {
                self.props.onsignal.emit(());
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
            <form
                onsubmit=self.link.callback(|_| Msg::Submitted)
                class=format!("form {}", self.props.class_name)
                id=format!("{}", self.props.id)
            >
                { self.props.children.render() }
            </form>
        }
    }
}

#[wasm_bindgen_test]
fn should_create_form_component() {
    let props = Props {
        class_name: "form-test".to_string(),
        id: "form-test-id".to_string(),
        onsignal: Callback::noop(),
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
        class_name: "form-test".to_string(),
        id: "form-test-id".to_string(),
        onsignal: onsubmit,
        children: Children::new(vec![html! {<input/>}]),
    };

    props.onsignal.emit(());

    let form_element = utils::document().get_element_by_id("form").unwrap();

    assert_eq!(
        form_element.text_content().unwrap(),
        "form submitted".to_string()
    );
}
