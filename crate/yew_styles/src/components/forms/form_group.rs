use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

pub struct FormGroup {
    props: Props,
}

#[derive(Clone)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or(Orientation::Vertical)]
    pub orientation: Orientation,
    #[prop_or_default]
    pub class_name: String,
    #[prop_or_default]
    pub id: String,
    pub children: Children,
}

impl Component for FormGroup {
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
            <div
                class=format!("form-group {} {}", self.props.class_name, get_orientation(self.props.orientation.clone()))
                id=self.props.id
                >
                {self.props.children.render()}
            </div>
        }
    }
}

fn get_orientation(orientation: Orientation) -> String {
    match orientation {
        Orientation::Horizontal => "horizontal".to_string(),
        Orientation::Vertical => "vertical".to_string(),
    }
}

#[wasm_bindgen_test]
fn should_create_form_group_horizontal_oriented() {
    let props = Props {
        id: "form-group-test-id".to_string(),
        class_name: "form-group-test-class".to_string(),
        orientation: Orientation::Horizontal,
        children: Children::new(vec![html! {
            <input id="input-child"/>
        }]),
    };

    let form_group: App<FormGroup> = App::new();
    form_group.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let form_group_h_element = utils::document()
        .get_elements_by_class_name("horizontal")
        .get_with_index(0)
        .unwrap();
    let form_group_v_element = utils::document()
        .get_elements_by_class_name("vertical")
        .get_with_index(0);

    assert_eq!(form_group_h_element.tag_name(), "DIV");
    assert_eq!(form_group_v_element, None);
}

#[wasm_bindgen_test]
fn should_create_form_group_vertical_oriented() {
    let props = Props {
        id: "form-group-test-id".to_string(),
        class_name: "form-group-test-class".to_string(),
        orientation: Orientation::Vertical,
        children: Children::new(vec![html! {
            <input id="input-child"/>
        }]),
    };

    let form_group: App<FormGroup> = App::new();
    form_group.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let form_group_h_element = utils::document()
        .get_elements_by_class_name("horizontal")
        .get_with_index(0);
    let form_group_v_element = utils::document()
        .get_elements_by_class_name("vertical")
        .get_with_index(0)
        .unwrap();

    assert_eq!(form_group_h_element, None);
    assert_eq!(form_group_v_element.tag_name(), "DIV");
}
