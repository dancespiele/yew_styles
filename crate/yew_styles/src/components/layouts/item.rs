use gloo::utils;
use stylist::{css, StyleSource, YieldStyle};
use wasm_bindgen_test::*;
use web_sys::window;
use yew::prelude::*;
use yew::start_app;

/// Percent of the layout that will take the item.
#[derive(Clone, PartialEq)]
pub enum ItemLayout {
    ItXs(i8),
    ItS(i8),
    ItM(i8),
    ItL(i8),
    ItXl(i8),
}

/// Align the item itself
#[derive(Clone, PartialEq)]
pub enum AlignSelf {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

pub enum Msg {
    Clicked(MouseEvent),
}

/// # Item component
///
/// The layouts in yew styles is base in flexbox
/// you can fine more information about the properties options
/// [here](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Flexible_Box_Layout/Basic_Concepts_of_Flexbox)
///
/// ## Features required
///
/// layouts
///
/// ## Example
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
pub struct Item {
    props: Props,
}

#[derive(Clone, PartialEq)]
struct ItemProps {
    layouts_classes: String,
    class_name: String,
    styles: StyleSource<'static>,
}

#[derive(Clone, Copy)]
struct ItemModel;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    /// Percent of the layout that will take the item. Required
    pub layouts: Vec<ItemLayout>,
    /// Align the item itself. Default `AlignSelf::Auto`
    #[prop_or(AlignSelf::Auto)]
    pub align_self: AlignSelf,
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
    /// Click event for the item
    #[prop_or(Callback::noop())]
    pub onclick_signal: Callback<MouseEvent>,
    /// Set css styles directly in the component
    #[prop_or(css!(""))]
    pub styles: StyleSource<'static>,
    pub children: Children,
}

impl YieldStyle for Item {
    fn style_from(&self) -> StyleSource<'static> {
        format!(
            r#"
                align-self: {};

                {}
            "#,
            get_item_align_self(self.props.align_self.clone()),
            get_layout_screen(self.props.layouts.clone()),
        )
        .into()
    }
}

impl Component for Item {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Item {
            props: *ctx.props(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(mouse_event) => {
                ctx.props().onclick_signal.emit(mouse_event);
            }
        };

        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props = *ctx.props();

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            layouts,
            align_self,
            code_ref,
            key,
            class_name,
            id,
            onclick_signal,
            styles,
            children,
        } = &ctx.props();

        html! {
            <div
                class={classes!(self.style(), get_layout_classes(layouts.clone()), class_name.clone(), styles.clone())}
                key={key.clone()}
                ref={code_ref.clone()}
                onclick={ctx.link().callback(Msg::Clicked)}
            >
                {children.clone()}
            </div>
        }
    }
}

fn get_item_align_self(align_self: AlignSelf) -> String {
    match align_self {
        AlignSelf::Auto => "auto".to_string(),
        AlignSelf::Baseline => "baseline".to_string(),
        AlignSelf::Center => "center".to_string(),
        AlignSelf::FlexStart => "flex-start".to_string(),
        AlignSelf::FlexEnd => "flex-end".to_string(),
        AlignSelf::Stretch => "stretch".to_string(),
    }
}

fn get_layout(screen: &str, size: i8) -> String {
    let calc_with = 100.0 / (12.0 / size as f32);

    format!(
        r#"
            &.it-{}-{} {{
                flex-basis: {}%;
            }}
        "#,
        screen, size, calc_with
    )
}

fn get_layout_classes(layouts_prop: Vec<ItemLayout>) -> String {
    let mut layouts = layouts_prop
        .into_iter()
        .map(get_layout_class)
        .collect::<String>();

    layouts.truncate(layouts.len() - 1);
    layouts
}

fn get_layout_class(item_layout: ItemLayout) -> String {
    match item_layout {
        ItemLayout::ItXs(size) => format!("it-xs-{} ", size),
        ItemLayout::ItS(size) => format!("it-s-{} ", size),
        ItemLayout::ItM(size) => format!("it-m-{} ", size),
        ItemLayout::ItL(size) => format!("it-l-{} ", size),
        ItemLayout::ItXl(size) => format!("it-xl-{} ", size),
    }
}

pub fn get_layout_screen(screens: Vec<ItemLayout>) -> String {
    let mut screens_order: Vec<Option<String>> = vec![None; 5];

    screens.to_vec().into_iter().for_each(|l| match l {
        ItemLayout::ItXs(size) => {
            screens_order[0] = Some(get_layout("xs", size));
        }
        ItemLayout::ItS(size) => {
            screens_order[1] = Some(format!(
                r#"
                        @media (min-width: 576px){{
                            {}
                        }}
                    "#,
                get_layout("s", size)
            ));
        }
        ItemLayout::ItM(size) => {
            screens_order[2] = Some(format!(
                r#"
                        @media (min-width: 768px){{
                            {}
                        }}
                    "#,
                get_layout("m", size)
            ));
        }
        ItemLayout::ItL(size) => {
            screens_order[3] = Some(format!(
                r#"
                        @media (min-width: 992px){{
                            {}
                        }}
                    "#,
                get_layout("l", size)
            ));
        }
        ItemLayout::ItXl(size) => {
            screens_order[4] = Some(format!(
                r#"
                        @media (min-width: 1200px){{
                            {}
                        }}
                    "#,
                get_layout("xl", size)
            ));
        }
    });

    screens_order
        .to_vec()
        .into_iter()
        .fold(String::from(""), |mut acc, s| {
            if let Some(screen) = s {
                acc.push_str(&screen);
            }

            acc
        })
}

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn should_create_item() {
    impl Default for Props {
        fn default() -> Props {
            Props {
                layouts: vec![ItemLayout::ItXs(12)],
                align_self: AlignSelf::Center,
                key: "".to_string(),
                code_ref: NodeRef::default(),
                class_name: "item-test".to_string(),
                id: "item-id-test".to_string(),
                onclick_signal: Callback::noop(),
                styles: css!("background-color: #918d94;"),
                children: Children::new(vec![html! {
                    <div id="item">{"Item"}</div>
                }]),
            }
        }
    }

    start_app::<Item>();

    let item_element = utils::document().get_element_by_id("item").unwrap();

    assert_eq!(item_element.text_content().unwrap(), "Item".to_string());
}

#[wasm_bindgen_test]
fn should_create_clickable_item() {
    let on_add_item_div = Callback::from(|_| {
        let body = window().unwrap().document().unwrap().body().unwrap();

        let child_element = window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();

        child_element.set_text_content(Some("item2"));
        child_element.set_id("item2");
        body.append_child(&child_element).unwrap();
    });

    let props_item = Props {
        layouts: vec![ItemLayout::ItXs(12)],
        align_self: AlignSelf::Center,
        key: "".to_string(),
        code_ref: NodeRef::default(),
        class_name: "item-test".to_string(),
        id: "item-id-test".to_string(),
        onclick_signal: on_add_item_div,
        styles: css!("background-color: #918d94;"),
        children: Children::new(vec![html! {
            <div id="item">{"Item"}</div>
        }]),
    };

    let mouse_event = MouseEvent::new("click").unwrap();

    props_item.onclick_signal.emit(mouse_event);

    let updated_content = window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("item2")
        .unwrap()
        .text_content()
        .unwrap();

    assert_eq!(updated_content, "item2".to_string());
}
