use crate::styles::{get_size, Size};
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

#[derive(Clone, Properties)]
pub struct Props {
    /// Different options to select. Required
    pub options: Html,
    pub onchange_signal: Callback<ChangeData>,
    /// Whether or not the selector should be disabled.
    #[prop_or_default]
    pub disabled: bool,
    /// The size of the select
    #[prop_or(Size::Medium)]
    pub select_size: Size,
    /// The name of the input
    #[prop_or_default]
    pub name: String,
    /// A value is required or must be check for the form to be submittable
    #[prop_or_default]
    pub required: bool,
    /// Whether to allow multiple values
    #[prop_or_default]
    pub multiple: bool,
    /// If the control is presented as a scrolling list box,
    /// this attribute represents the number of rows in the
    /// list that should be visible at one time
    #[prop_or_default]
    pub size: u16,
    /// Automatically focus the form control when the page is loaded
    #[prop_or_default]
    pub autofocus: bool,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// Error state for validation
    #[prop_or_default]
    pub error_state: bool,
    /// show error message when error_state is true.
    #[prop_or_default]
    pub error_message: String,
    /// general property to add custom id
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
                self.props.onchange_signal.emit(value);
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
            <>
                <select
                    class=format!("form-select {} {}",
                        get_size(self.props.select_size.clone()),
                        self.props.class_name,
                    )
                    id=self.props.id
                    disabled=self.props.disabled,
                    name=self.props.name,
                    autofocus=self.props.autofocus,
                    required=self.props.required,
                    multiple=self.props.multiple,
                    size=self.props.size,
                    onchange=self.link.callback(Msg::Selected)
                >

                    {self.props.options.clone()}
                </select>
                {get_error_message(self.props.error_state, self.props.error_message.clone())}
            </>
        }
    }
}

fn get_error_message(error_state: bool, error_message: String) -> Html {
    if error_state {
        html! {<span class="form-error">{error_message}</span>}
    } else {
        html! {}
    }
}

#[wasm_bindgen_test]
fn should_create_form_select() {
    let props = Props {
        onchange_signal: Callback::noop(),
        id: "form-select-id-test".to_string(),
        class_name: "form-select-class-test".to_string(),
        disabled: false,
        autofocus: false,
        required: false,
        select_size: Size::Medium,
        size: 0,
        name: "options".to_string(),
        error_message: "".to_string(),
        error_state: false,
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
