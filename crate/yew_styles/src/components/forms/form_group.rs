use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

/// # Form Group
///
/// ## Features required
///
/// forms
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_styles::forms::{
///     form_label::FormLabel,
///     form_group::{FormGroup, Orientation},
///     form_textarea::FormTextArea,
/// };
/// use yew_styles::styles::{Palette, Size};
///
/// pub struct FormGroupExample {
///     pub link: ComponentLink<Self>,
///     pub value: String,
/// }
///
/// pub enum Msg {
///     Input(String),
/// }
///
/// impl Component for FormGroupExample {
///     type Message = Msg;
///     type Properties = ();
///     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
///         FormGroupExample {
///             link,
///             value: "".to_string(),
///         }
///     }
///     fn update(&mut self, msg: Self::Message) -> ShouldRender {
///         match msg {
///             Msg::Input(value) => {
///                 self.value = value;
///             }
///         }
///         true
///     }
///     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
///         false
///     }
///
///     fn view(&self) -> Html {
///         html!{
///             <FormGroup orientation=Orientation::Vertical>
///                 <FormLabel
///                 text="Info small textarea"
///                 />
///                 <FormTextArea placeholder="write here"
///                     value=form_page.value.clone()
///                     textarea_size=Size::Small
///                     textarea_style=Palette::Info
///                     oninput_signal=form_page.link.callback(|e: InputData| Msg::Input(e.value))
///                 />
///             </FormGroup>
///         }
///     }
/// ```
pub struct FormGroup {
    props: Props,
}

/// Orientation type
#[derive(Clone, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// In which orientation will show the inputs, select and labels. Default `Orientation::Vertical`
    #[prop_or(Orientation::Vertical)]
    pub orientation: Orientation,
    /// General property to get the ref of the component
    #[prop_or_default]
    pub code_ref: NodeRef,
    /// General property to add keys
    #[prop_or_default]
    pub key: String,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
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
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div
                class=format!("form-group {} {}", get_orientation(self.props.orientation.clone()), self.props.class_name)
                id=self.props.id
                key=self.props.key.clone()
                ref=self.props.code_ref.clone()
                >
                {self.props.children.clone()}
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
        key: "".to_string(),
        code_ref: NodeRef::default(),
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
        key: "".to_string(),
        code_ref: NodeRef::default(),
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
