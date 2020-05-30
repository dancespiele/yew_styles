use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

pub struct Form {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    /// signal to emit the event submit.
    #[prop_or(Callback::noop())]
    pub onsubmit_signal: Callback<Event>,
    pub children: Children,
    #[prop_or_default]
    pub action: String,
    #[prop_or(Method::Post)]
    pub method: Method,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub class_name: String,
    #[prop_or_default]
    pub id: String,
}

#[derive(Clone)]
pub enum Method {
    Post,
    Get,
    Dialog,
}

pub enum Msg {
    Submitted(Event),
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
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <form
                onsubmit=self.link.callback(|e: Event| Msg::Submitted(e))
                action=self.props.action
                method=get_method(self.props.method.clone())
                name=self.props.name
                class=format!("form {}", self.props.class_name)
                id=format!("{}", self.props.id)
            >
                { self.props.children.render() }
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
        class_name: "form-test".to_string(),
        id: "form-test-id".to_string(),
        onsubmit_signal: onsubmit,
        method: Method::Post,
        action: "".to_string(),
        name: "form-test".to_string(),
        children: Children::new(vec![html! {<input/>}]),
    };

    let event = Event::new("Submit").unwrap();

    props.onsubmit_signal.emit(event);

    let form_element = utils::document().get_element_by_id("form").unwrap();

    assert_eq!(
        form_element.text_content().unwrap(),
        "form submitted".to_string()
    );
}
