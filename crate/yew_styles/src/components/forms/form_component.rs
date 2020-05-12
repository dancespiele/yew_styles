use wasm_bindgen_test::*;
use web_sys::window;
use yew::prelude::*;

pub struct Form {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    // get all the values from the form
    pub onsignal: Callback<()>,
    pub children: Children,
    pub class_name: String,
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
    let body = window().unwrap().document().unwrap().body().unwrap();

    let element = window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    element.set_text_content(Some("fill the form"));
    element.set_id("form");

    body.append_child(&element).unwrap();

    let onsubmit = Callback::from(|_| {
        let content = window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("form")
            .unwrap();

        content.set_text_content(Some("form submitted"));
    });

    let props = Props {
        class_name: "form-test".to_string(),
        id: "form-test-id".to_string(),
        onsignal: onsubmit,
        children: Children::new(vec![html! {<input/>}]),
    };

    let link = ComponentLink::new();

    let form = Form::create(props, link.clone());

    let form_vnode = form.render();

    let vsnode_expected = html! {
        <form
            onsubmit=link.callback(|_| Msg::Submitted)
            class="form form-test"
            id="form-test-id"
        >
            <>
                <input/>
            </>
        </form>
    };
    assert_eq!(form_vnode, vsnode_expected);
}
