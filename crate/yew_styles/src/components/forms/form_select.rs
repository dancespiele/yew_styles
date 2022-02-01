use super::error_message::get_error_message;
use crate::styles::helpers::{get_size, Size};
use stylist::{css, StyleSource, YieldStyle};
use yew::prelude::*;

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
/// use yew_styles::styles::helpers::{Size};
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
pub struct FormSelect;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// Different options to select. Required
    pub options: Html,
    pub onchange_signal: Callback<Event>,
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
    Selected(Event),
}

impl YieldStyle for FormSelect {
    fn style_from(&self) -> StyleSource<'static> {
        css!(
            r#"
            padding: 3px;
            width: 100%;

            &.small {
                padding: 0;
            }
        
            &.big{
                padding: 5px;
            }
        "#
        )
    }
}

impl Component for FormSelect {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Selected(value) => {
                ctx.props().onchange_signal.emit(value);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            options,
            onchange_signal,
            disabled,
            select_size,
            name,
            required,
            multiple,
            size,
            autofocus,
            code_ref,
            key,
            class_name,
            error_state,
            id,
            error_message,
            styles,
        } = &ctx.props();

        html! {
            <>
                <select
                    class={classes!(
                        self.style(),
                        get_size(select_size.clone()),
                        class_name.clone(),
                        styles.clone()
                    )}
                    id={id.clone()}
                    key={key.clone()}
                    ref={code_ref.clone()}
                    disabled={*disabled}
                    name={name.clone()}
                    autofocus={*autofocus}
                    required={*required}
                    multiple={*multiple}
                    size={size.to_string()}
                    onchange={ctx.link().callback(Msg::Selected)}
                >
                    {options.clone()}
                </select>
                {get_error_message(*error_state, error_message.clone())}
            </>
        }
    }
}

// #[wasm_bindgen_test]
// fn should_create_form_select() {
//     let props = Props {
//         onchange_signal: Callback::noop(),
//         id: "form-select-id-test".to_string(),
//         class_name: "form-select-class-test".to_string(),
//         key: "".to_string(),
//         code_ref: NodeRef::default(),
//         disabled: false,
//         autofocus: false,
//         required: false,
//         select_size: Size::Medium,
//         size: 0,
//         name: "options".to_string(),
//         error_message: "".to_string(),
//         error_state: false,
//         multiple: false,
//         styles: css!("background-color: #918d94;"),
//         options: html! {
//             <>
//                 <option value="value-1" selected=true>{"option 1"}</option>
//                 <option value="value-2">{"option 2"}</option>
//                 <option value="value-3">{"option 3"}</option>
//                 <option value="value-4" id="result">{"option 4"}</option>
//             </>
//         },
//     };

//     start_app::<FormSelect>();
//     form_select.mount_with_props(
//         utils::document().get_element_by_id("output").unwrap(),
//         props,
//     );

//     let form_select_element = utils::document().get_element_by_id("result").unwrap();

//     assert_eq!(
//         form_select_element.text_content().unwrap(),
//         "option 4".to_string()
//     );
// }
