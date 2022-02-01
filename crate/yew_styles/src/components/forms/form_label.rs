use stylist::{css, StyleSource, YieldStyle};
use yew::prelude::*;

/// # Form Label
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
/// use yew_styles::styles::helpers::{Palette, Size};
///
/// pub struct FormLabelExample {
///     pub link: ComponentLink<Self>,
///     pub value: String,
/// }
///
/// pub enum Msg {
///     Input(String),
/// }
///
/// impl Component for FormLabelExample {
///     type Message = Msg;
///     type Properties = ();
///     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
///         FormLabelExample {
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
pub struct FormLabel;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// text of the label. Required
    pub text: String,
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
    /// The id of a labelable form-related element in the same document as the <label> element
    #[prop_or_default]
    pub label_for: String,
    /// Set css styles directly in the component
    #[prop_or(css!(""))]
    pub styles: StyleSource<'static>,
}

impl YieldStyle for FormLabel {
    fn style_from(&self) -> StyleSource<'static> {
        css!(
            r#"
            margin-right: 5px;
        "#
        )
    }
}

impl Component for FormLabel {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            text,
            code_ref,
            key,
            class_name,
            id,
            label_for,
            styles,
        } = &ctx.props();

        html! {
            <label
                class={classes!(self.style(), class_name, styles.clone())}
                id={id.clone()}
                key={key.clone()}
                ref={code_ref.clone()}
                for={label_for.clone()}
            >{text.clone()}</label>
        }
    }
}

// #[wasm_bindgen_test]
// fn should_create_form_label() {
//     let props = Props {
//         key: "".to_string(),
//         code_ref: NodeRef::default(),
//         class_name: "form-label-class-test".to_string(),
//         id: "form-label-id-test".to_string(),
//         label_for: "label-form".to_string(),
//         styles: css!("background-color: #918d94;"),
//         text: "label text".to_string(),
//     };

//     start_app::<FormLabel>();

//     form_label.mount_with_props(
//         utils::document().get_element_by_id("output").unwrap(),
//         props,
//     );

//     let form_label_element = utils::document()
//         .get_element_by_id("form-label-id-test")
//         .unwrap();

//     assert_eq!(
//         form_label_element.text_content().unwrap(),
//         "label text".to_string()
//     )
// }
