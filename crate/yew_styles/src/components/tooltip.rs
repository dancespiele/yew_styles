use crate::styles::helpers::{get_palette, get_size, get_style, Palette, Position, Size, Style};
use gloo::utils;
use stylist::{css, StyleSource};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::start_app_with_props;

/// # Tooltip component
///
/// ## Features required
///
/// tooltip
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_styles::styles::helpers::{Palette, Position, Size, Style};
/// use yew_styles::tooltip::Tooltip;
///
/// pub struct TooltipPage;
///
/// impl Component for TooltipPage {
///     type Message = ();
///     type Properties = ();
///
///     fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
///         Self
///     }
///
///     fn update(&mut self, _msg: Self::Message) -> ShouldRender {
///         false
///     }
///
///     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
///         false
///     }
///
///     fn view(&self) -> Html {
///         html! {
///             <>
///                 <Tooltip
///                     tooltip_palette=Palette::Standard
///                     tooltip_style=Style::Outline
///                     tooltip_position=Position::Above
///                     tooltip_size=Size::Small
///                     content=html!{<span>{"Info"}</span>}
///                     class_name="tooltip-page"
///                     >
///                     <div class="tooltip-content">{"Tooltip above"}</div>
///                 </Tooltip>
///             </>
///         }
///     }
/// }
/// ```
pub struct Tooltip {
    props: Props,
    show_tooltip: bool,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    /// Tooltip palette. Default `Palette::Standard`
    #[prop_or(Palette::Standard)]
    pub tooltip_palette: Palette,
    /// Tooltip style. Default `Style::Regular`
    #[prop_or(Style::Regular)]
    pub tooltip_style: Style,
    /// Tooltip size. Default `Size::Medium`
    #[prop_or(Size::Medium)]
    pub tooltip_size: Size,
    /// General property to add keys
    #[prop_or_default]
    pub key: String,
    /// General property to get the ref of the component
    #[prop_or_default]
    pub code_ref: NodeRef,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    /// Tooltip position over targeted element. Left, Right, Above, Below
    pub tooltip_position: Position,
    /// Show the content of tooltip
    pub content: Html,
    /// Set css styles directly in the component
    #[prop_or(css!(""))]
    pub styles: StyleSource<'static>,
    pub children: Children,
}

pub enum Msg {
    TargetOver,
    TargetLeave,
}

impl Component for Tooltip {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: *ctx.props(),
            show_tooltip: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TargetOver => self.show_tooltip = true,
            Msg::TargetLeave => self.show_tooltip = false,
        };

        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props = *ctx.props();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let tooltip = html! {
            <div
                id={ctx.props().id.clone()}
                key={ctx.props().key.clone()}
                ref={ctx.props().code_ref.clone()}
                class={classes!(
                    "tooltip",
                    get_position(ctx.props().tooltip_position.clone()),
                    get_palette(ctx.props().tooltip_palette.clone()),
                    get_style(ctx.props().tooltip_style.clone()),
                    get_size(ctx.props().tooltip_size.clone()),
                    ctx.props().class_name.clone(),
                    ctx.props().styles.clone()
                )}
            >
             {ctx.props().content.clone()}
            </div>
        };

        html! {
            <div class="tooltip-container"
                onmouseover = {ctx.link().callback(|_| Msg::TargetOver)}
                onmouseleave = {ctx.link().callback(|_| Msg::TargetLeave)}
            >
                {if self.show_tooltip {
                    tooltip
                }else {
                    html!{}
                }}
                {ctx.props().children.clone()}
            </div>
        }
    }
}

fn get_position(position: Position) -> String {
    match position {
        Position::Left => String::from("left"),
        Position::Right => String::from("right"),
        Position::Above => String::from("above"),
        Position::Below => String::from("below"),
    }
}

#[wasm_bindgen_test]
fn should_create_tooltip() {
    let props = Props {
        tooltip_palette: Palette::Clean,
        tooltip_style: Style::Regular,
        tooltip_size: Size::Medium,
        tooltip_position: Position::Above,
        content: html! {<p>{"tooltip"}</p>},
        code_ref: NodeRef::default(),
        key: String::from("dropdown-1"),
        class_name: String::from("class-test"),
        id: String::from("id-test"),
        styles: css!("color: blue;"),
        children: Children::new(vec![html! {<div id="result">{"result"}</div>}]),
    };

    start_app_with_props::<Tooltip>(props);

    let tooltip_element = utils::document()
        .get_elements_by_class_name("tooltip-container")
        .get_with_index(0)
        .unwrap();

    let child = tooltip_element.first_element_child().unwrap();

    assert_eq!(child.id(), "result".to_string());
}
