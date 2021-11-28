use crate::styles::{get_palette, get_size, get_style, Palette, Position, Size, Style};
use stylist::{css, StyleSource};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

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
/// use yew_styles::styles::{Palette, Position, Size, Style};
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
    link: ComponentLink<Self>,
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            show_tooltip: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TargetOver => self.show_tooltip = true,
            Msg::TargetLeave => self.show_tooltip = false,
        };

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            return true;
        }
        false
    }

    fn view(&self) -> Html {
        let tooltip = html! {
            <div
                id=self.props.id.clone()
                key=self.props.key.clone()
                ref=self.props.code_ref.clone()
                class=classes!(
                    "tooltip",
                    get_position(self.props.tooltip_position.clone()),
                    get_palette(self.props.tooltip_palette.clone()),
                    get_style(self.props.tooltip_style.clone()),
                    get_size(self.props.tooltip_size.clone()),
                    self.props.class_name.clone(),
                    self.props.styles.clone()
                )
            >
             {self.props.content.clone()}
            </div>
        };

        html! {
            <div class="tooltip-container"
                onmouseover = self.link.callback(|_| Msg::TargetOver)
                onmouseleave = self.link.callback(|_| Msg::TargetLeave)
            >
                {if self.show_tooltip {
                    tooltip
                }else {
                    html!{}
                }}
                {self.props.children.clone()}
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
    let tooltip_props = Props {
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

    let tooltip: App<Tooltip> = App::new();

    tooltip.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        tooltip_props,
    );

    let tooltip_element = utils::document()
        .get_elements_by_class_name("tooltip-container")
        .get_with_index(0)
        .unwrap();

    let child = tooltip_element.first_element_child().unwrap();

    assert_eq!(child.id(), "result".to_string());
}
