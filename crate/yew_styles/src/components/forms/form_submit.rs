use crate::styles::colors::get_styles;
use crate::styles::helpers::{
    get_palette, get_palette_style, get_size, get_style, Palette, Size, Style,
};
use stylist::{css, StyleSource, YieldStyle};
use yew::prelude::*;

/// # Form Submit
///
/// ## Features required
///
/// forms
///
/// see example in Form
pub struct FormSubmit {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// Text of submit. Required
    pub value: String,
    /// Type submit style. Default `Palette::Standard`
    #[prop_or(Palette::Standard)]
    pub submit_palette: Palette,
    /// the submit style according with the purpose. Default `Style::Regular`
    #[prop_or(Style::Regular)]
    pub submit_style: Style,
    /// the size of the submit. Default `Size::Medium`
    #[prop_or(Size::Medium)]
    pub size: Size,
    /// Whether the form control is disabled. Default `false`
    #[prop_or(false)]
    pub disabled: bool,
    /// General property to get the ref of the component
    #[prop_or_default]
    pub code_ref: NodeRef,
    /// General property to add keys
    #[prop_or_default]
    pub key: String,
    /// general property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// general property to add custom id
    #[prop_or_default]
    pub id: String,
    #[prop_or(css!(""))]
    pub styles: StyleSource<'static>,
}

impl YieldStyle for FormSubmit {
    fn style_from(&self) -> StyleSource<'static> {
        let styles = get_styles();
        let style = get_style(self.props.submit_style.clone());
        let color = styles
            .get(style.as_str())
            .unwrap()
            .iter()
            .find(|palette| palette.name == get_palette(self.props.submit_palette.clone()))
            .unwrap();

        css!(
            r#"
                padding: 5px 10px;
                border: none;
                border-radius: 4px;
                cursor: pointer;
                font-size: 18px;

                ${palette}

                &.small {
                    font-size: 12px
                }

                &.big {
                    font-size: 26px
                }
            "#,
            palette = get_palette_style(color, !self.props.disabled)
        )
    }
}

impl Component for FormSubmit {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: *ctx.props(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props = *ctx.props();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            value,
            submit_palette,
            submit_style,
            size,
            disabled,
            code_ref,
            key,
            class_name,
            id,
            styles,
        } = &ctx.props();

        html! {
            <input
                type={"submit"}
                key={key.clone()}
                ref={code_ref.clone()}
                class={classes!(
                    self.style(),
                    get_palette(submit_palette.clone()),
                    get_size(size.clone()),
                class_name.clone())}
                disabled={*disabled}
                id={id.clone()}
                value={value.clone()}
            />
        }
    }
}

// #[wasm_bindgen_test]
// fn should_create_form_submit() {
//     let props = Props {
//         value: "submit".to_string(),
//         disabled: false,
//         key: "".to_string(),
//         code_ref: NodeRef::default(),
//         id: "result".to_string(),
//         class_name: "form-submit-test".to_string(),
//         submit_style: Style::Regular,
//         submit_palette: Palette::Standard,
//         size: Size::Medium,
//         styles: css!("background-color: #918d94;"),
//     };

//     let form_submit: App<FormSubmit> = App::new();

//     form_submit.mount_with_props(
//         utils::document().get_element_by_id("output").unwrap(),
//         props,
//     );

//     let form_submit_element = utils::document().get_element_by_id("result").unwrap();

//     assert_eq!(form_submit_element.tag_name(), "INPUT");
// }
