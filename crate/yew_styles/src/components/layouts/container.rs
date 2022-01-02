use stylist::{css, StyleSource, YieldStyle};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

/// # Container component
///
/// ## Features required
///
/// layouts
///
/// ## Example
///
/// The layouts in yew styles is base in flexbox
/// you can fine more information about the properties options
/// [here](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Flexible_Box_Layout/Basic_Concepts_of_Flexbox)
///
/// ```rust
/// use yew::prelude::*;
/// use yew_styles::{
///     layouts::{
///         container::{Wrap, Direction},
///         item::{ItenLayout, AlignSelf}
///     }
/// };
///
/// pub struct App {
///   link: ComponentLink<Self>,
/// }
///
/// pub enum Msg {
///   Clicked(String),
/// }
/// #[derive(Clone, Properties)]
/// pub struct Props {}
///
/// impl Component for App {
///     type Message = Msg;
///     type Properties = Props;
///
///     fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
///         App {
///             link
///         }
///     }
///
///     fn update(&mut self, msg: Self::Message) -> ShouldRender {
///         false
///     }
///
///     fn view(&self) -> Html {
///        html! {
///          <Container direction=Direction::Row wrap=Wrap::Wrap class_name="align-item">
///                <Item name="align" layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexStart>
///                    <h3>{"start"}</h3>
///                </Item>
///                <Item name="align" layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::Center>
///                    <h3>{"center"}</h3>
///                </Item>
///                <Item name="align" layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexEnd>
///                    <h3>{"end"}</h3>
///                </Item>
///           </Container>
///        }
///     }
/// }
/// ```
pub struct Container {
    props: Props,
}

#[derive(Clone, Copy)]
struct ContainerModel;

/// Which direction are placing the items
#[derive(Clone, PartialEq)]
pub enum Direction {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

/// Set a wrap for the items
#[derive(Clone, PartialEq)]
pub enum Wrap {
    Nowrap,
    Wrap,
    WrapReverse,
}

#[derive(Clone, PartialEq)]
pub enum Mode {
    SafeMode,
    UnsafeMode,
    NoMode,
}

/// Set how will be justified the content
#[derive(Clone, PartialEq)]
pub enum JustifyContent {
    FlexStart(Mode),
    FlexEnd(Mode),
    Center(Mode),
    SpaceBetween(Mode),
    SpaceAround(Mode),
    SpaceEvenly(Mode),
    Start(Mode),
    End(Mode),
    Left(Mode),
    Rigth(Mode),
}

/// Set how will be aligned the items
#[derive(Clone, PartialEq)]
pub enum AlignItems {
    Stretch(Mode),
    FlexStart(Mode),
    FlexEnd(Mode),
    Center(Mode),
    Baseline(Mode),
    FirstBaseline(Mode),
    LastBaseline(Mode),
    Start(Mode),
    End(Mode),
    SelfStart(Mode),
    SelfEnd(Mode),
}

/// set how will be aligned the content
#[derive(Clone, PartialEq)]
pub enum AlignContent {
    FlexStart(Mode),
    FlexEnd(Mode),
    Center(Mode),
    SpaceBetween(Mode),
    SpaceAround(Mode),
    SpaceEvenly(Mode),
    Stretch(Mode),
    Start(Mode),
    End(Mode),
    Baseline(Mode),
    FirstBaseline(Mode),
    LastBaseline(Mode),
}

pub enum Msg {}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// Which direction are placing the items. Required
    pub direction: Direction,
    /// Set a wrap for the items. Required
    pub wrap: Wrap,
    /// Set how will be justified the content. Default `JustifyContent::FlexStart(Mode::NoMode)`
    #[prop_or(JustifyContent::FlexStart(Mode::NoMode))]
    pub justify_content: JustifyContent,
    /// Set how will be aligned the content. Default `AlignContent::Stretch(Mode::NoMode)`
    #[prop_or(AlignContent::Stretch(Mode::NoMode))]
    pub align_content: AlignContent,
    /// Set how will be aligned the items. Default `AlignItems::Stretch(Mode::NoMode)`
    #[prop_or(AlignItems::Stretch(Mode::NoMode))]
    pub align_items: AlignItems,
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
    #[prop_or(css!(""))]
    pub styles: StyleSource<'static>,
    pub children: Children,
}

impl YieldStyle for Container {
    fn style_from(&self) -> StyleSource<'static> {
        css!(
            r#"
                display: flex;
                flex-flow: ${flow};
                justify-content: ${justify_content};
                align-items: ${align_items};
                align-content: ${align_content};
            "#,
            flow = get_flow(self.props.direction.clone(), self.props.wrap.clone()),
            justify_content = get_justify_content(self.props.justify_content.clone()),
            align_items = get_align_items(self.props.align_items.clone()),
            align_content = get_align_content(self.props.align_content.clone()),
        )
    }
}

impl Component for Container {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Container { props }
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
            <div class=classes!(self.style(), self.props.class_name.clone(), self.props.styles.clone())
                id=self.props.id.to_string()
                key=self.props.key.clone()
                ref=self.props.code_ref.clone()
            >
                {self.props.children.clone()}
            </div>
        }
    }
}

fn get_flow(direction: Direction, wrap: Wrap) -> String {
    let direction = match direction {
        Direction::Row => "row".to_string(),
        Direction::RowReverse => "row-reverse".to_string(),
        Direction::Column => "column".to_string(),
        Direction::ColumnReverse => "column-reverse".to_string(),
    };

    let wrap = match wrap {
        Wrap::Nowrap => "nowrap".to_string(),
        Wrap::Wrap => "wrap".to_string(),
        Wrap::WrapReverse => "wrap-reverse".to_string(),
    };

    format!("{} {}", direction, wrap)
}

fn get_mode(mode: Mode) -> String {
    match mode {
        Mode::NoMode => "".to_string(),
        Mode::SafeMode => " safe".to_string(),
        Mode::UnsafeMode => " unsafe".to_string(),
    }
}

fn get_justify_content(justify_content: JustifyContent) -> String {
    match justify_content {
        JustifyContent::FlexStart(mode) => format!("flex-start{}", get_mode(mode)),
        JustifyContent::FlexEnd(mode) => format!("flex-end{}", get_mode(mode)),
        JustifyContent::Start(mode) => format!("start{}", get_mode(mode)),
        JustifyContent::End(mode) => format!("end{}", get_mode(mode)),
        JustifyContent::Left(mode) => format!("left{}", get_mode(mode)),
        JustifyContent::Center(mode) => format!("center{}", get_mode(mode)),
        JustifyContent::Rigth(mode) => format!("right{}", get_mode(mode)),
        JustifyContent::SpaceAround(mode) => format!("space-around{}", get_mode(mode)),
        JustifyContent::SpaceBetween(mode) => format!("space-between{}", get_mode(mode)),
        JustifyContent::SpaceEvenly(mode) => format!("evenly{}", get_mode(mode))
    }
}

fn get_align_content(align_content: AlignContent) -> String {
    match align_content {
        AlignContent::Stretch(mode) => format!("stretch{}", get_mode(mode)),
        AlignContent::FlexStart(mode) => format!("flex-start{}", get_mode(mode)),
        AlignContent::FlexEnd(mode) => format!("flex-end{}", get_mode(mode)),
        AlignContent::Start(mode) => format!("start{}", get_mode(mode)),
        AlignContent::End(mode) => format!("end{}", get_mode(mode)),
        AlignContent::Center(mode) => format!("center{}", get_mode(mode)),
        AlignContent::Baseline(mode) => format!("baseline{}", get_mode(mode)),
        AlignContent::FirstBaseline(mode) => format!("first-baseline{}", get_mode(mode)),
        AlignContent::LastBaseline(mode) => format!("last-baseline{}", get_mode(mode)),
        AlignContent::SpaceAround(mode) => format!("space-around{}", get_mode(mode)),
        AlignContent::SpaceBetween(mode) => format!("space-between{}", get_mode(mode)),
        AlignContent::SpaceEvenly(mode) => format!("evenly{}", get_mode(mode)),
    }
}

fn get_align_items(align_items: AlignItems) -> String {
    match align_items {
        AlignItems::Stretch(mode) => format!("stretch{}", get_mode(mode)),
        AlignItems::Baseline(mode) => format!("baseline{}", get_mode(mode)),
        AlignItems::Start(mode) => format!("start{}", get_mode(mode)),
        AlignItems::End(mode) => format!("end{}", get_mode(mode)),
        AlignItems::FlexStart(mode) => format!("start{}", get_mode(mode)),
        AlignItems::FlexEnd(mode) => format!("flex-end{}", get_mode(mode)),
        AlignItems::FirstBaseline(mode) => format!("first-baseline{}", get_mode(mode)),
        AlignItems::LastBaseline(mode) => format!("last-baseline{}", get_mode(mode)),
        AlignItems::SelfStart(mode) => format!("self-start{}", get_mode(mode)),
        AlignItems::SelfEnd(mode) => format!("self-end{}", get_mode(mode)),
        AlignItems::Center(mode) => format!("center{}", get_mode(mode)),
    }
}

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn should_create_a_container() {
    let props_container = Props {
        direction: Direction::Row,
        wrap: Wrap::Wrap,
        justify_content: JustifyContent::Center(Mode::NoMode),
        align_content: AlignContent::Center(Mode::NoMode),
        align_items: AlignItems::Center(Mode::NoMode),
        key: "".to_string(),
        code_ref: NodeRef::default(),
        class_name: String::from("layout-test"),
        styles: css!("color: red;"),
        id: String::from("layout-id-test"),
        children: Children::new(vec![html! {
            <div id="container">{"Container"}</div>
        }]),
    };

    let container: App<Container> = App::new();
    container.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props_container,
    );

    let container_element = utils::document().get_element_by_id("container").unwrap();

    assert_eq!(
        container_element.text_content().unwrap(),
        "Container".to_string()
    );
}
