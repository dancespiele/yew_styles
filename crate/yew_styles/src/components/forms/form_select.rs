use super::error_message::get_error_message;
use crate::styles::{get_size, Size};
use stylist::{css, StyleSource};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App, ChangeData};

/// # Form Select
///
/// ## Features required
///
/// forms
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_styles::forms::form_select::FormSelect;
/// use yew_styles::styles::{Size};
///
/// pub struct FormSelectExample {
///     pub link: ComponentLink<Self>,
///     pub value: String,
/// }
///
/// pub enum Msg {
///     Select(String),
/// }
///
/// impl Component for FormSelectExample {
///     type Message = Msg;
///     type Properties = ();
///     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
///         FormSelectExample {
///             link,
///             value: "".to_string(),
///         }
///     }
///     fn update(&mut self, msg: Self::Message) -> ShouldRender {
///         match msg {
///             Msg::Select(value) => {
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
///             <FormSelect
///                 select_size=Size::Medium
///                 onchange_signal = form_page.link.callback(|e: ChangeData|
///                     match e {
///                         ChangeData::Select(element) => {
///                             let value = element.value();
///                             Msg::Select(value)
///                         },
///                         _ => unreachable!(),
///                     }
///                 )
///                 options=html!{
///                     <>
///                         <option value="" disabled=true>{"Select library"}</option>
///                         <option value="yew">{"Yew"}</option>
///                         <option value="yew_styles">{"Yew Styles"}</option>
///                         <option value="yew_prism">{"Yew prism"}</option>
///                     </>
///                 }
///             />
///         }
///     }
/// ```
pub struct FormSelect {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// Different options to select. Required
    pub options: Html,
    pub onchange_signal: Callback<ChangeData>,
    /// Whether or not the selector should be disabled.
    #[prop_or_default]
    pub disabled: bool,
    /// The size of the select. Default `Size::Medium`
    #[prop_or(Size::Medium)]
    pub select_size: Size,
    /// The name of the input
    #[prop_or_default]
    pub name: String,
    /// A value is required or must be check for the form to be submittable. Default `false`
    #[prop_or(false)]
    pub required: bool,
    /// Whether to allow multiple values. Default `false`
    #[prop_or(false)]
    pub multiple: bool,
    /// If the control is presented as a scrolling list box,
    /// this attribute represents the number of rows in the
    /// list that should be visible at one time
    #[prop_or_default]
    pub size: u16,
    /// Automatically focus the form control when the page is loaded. Default `false`
    #[prop_or(false)]
    pub autofocus: bool,
    /// General property to get the ref of the component
    #[prop_or_default]
    pub code_ref: NodeRef,
    /// General property to add keys
    #[prop_or_default]
    pub key: String,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// Error state for validation. Default `false`
    #[prop_or(false)]
    pub error_state: bool,
    /// show error message when error_state is true.
    #[prop_or_default]
    pub error_message: String,
    /// general property to add custom id
    #[prop_or_default]
    pub id: String,
    /// Set css styles directly in the component
    #[prop_or(css!(""))]
    pub styles: StyleSource<'static>,
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
                self.props.onchange_signal.emit(value);
            }
        }
        true
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
            <>
                <select
                    class=classes!(
                        "form-select",
                        get_size(self.props.select_size.clone()),
                        self.props.class_name.clone(),
                        self.props.styles.clone()
                    )
                    id=self.props.id.clone()
                    key=self.props.key.clone()
                    ref=self.props.code_ref.clone()
                    disabled=self.props.disabled
                    name=self.props.name.clone()
                    autofocus=self.props.autofocus
                    required=self.props.required
                    multiple=self.props.multiple
                    size=self.props.size.to_string()
                    onchange=self.link.callback(Msg::Selected)
                >
                    {self.props.options.clone()}
                </select>
                {get_error_message(self.props.error_state, self.props.error_message.clone())}
            </>
        }
    }
}

#[wasm_bindgen_test]
fn should_create_form_select() {
    let props = Props {
        onchange_signal: Callback::noop(),
        id: "form-select-id-test".to_string(),
        class_name: "form-select-class-test".to_string(),
        key: "".to_string(),
        code_ref: NodeRef::default(),
        disabled: false,
        autofocus: false,
        required: false,
        select_size: Size::Medium,
        size: 0,
        name: "options".to_string(),
        error_message: "".to_string(),
        error_state: false,
        multiple: false,
        styles: css!("background-color: #918d94;"),
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
