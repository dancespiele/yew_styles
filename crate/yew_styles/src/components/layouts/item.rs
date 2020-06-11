use crate::utils::{create_style, get_random_string};
use wasm_bindgen_test::*;
use web_sys::window;
use yew::prelude::*;
use yew::{utils, App};

/// Percent of the layout that will take the item.
#[derive(Clone)]
pub enum ItemLayout {
    ItXs(i8),
    ItS(i8),
    ItM(i8),
    ItL(i8),
    ItXl(i8),
}

/// Align the item itself
#[derive(Clone)]
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
    link: ComponentLink<Self>,
    props: Props,
    pub key: String,
}

#[derive(Clone)]
struct ItemProps {
    layouts_classes: String,
    class_name: String,
}

#[derive(Clone, Copy)]
struct ItemModel;

#[derive(Clone, Properties)]
pub struct Props {
    /// Percent of the layout that will take the item. Required
    pub layouts: Vec<ItemLayout>,
    #[prop_or(AlignSelf::Auto)]
    /// Align the item itself
    pub align_self: AlignSelf,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    /// Click event for the item
    #[prop_or(Callback::noop())]
    pub onclick_signal: Callback<MouseEvent>,
    pub children: Children,
}

impl Component for Item {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let key = get_random_string(10);

        Item { link, props, key }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked(mouse_event) => {
                self.props.onclick_signal.emit(mouse_event);
            }
        };

        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            ItemModel.init(self.props.align_self.clone(), self.key.clone());
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let item_props = ItemProps::from(self.props.clone());

        html! {
            <div
                class=format!("item item-{} {} {}", self.key, item_props.layouts_classes, item_props.class_name)

                onclick=self.link.callback(Msg::Clicked)
            >
                {self.props.children.render()}
            </div>
        }
    }
}

impl From<Props> for ItemProps {
    fn from(props: Props) -> Self {
        ItemProps {
            layouts_classes: ItemModel.get_layout_classes(props.layouts),
            class_name: props.class_name,
        }
    }
}

impl ItemModel {
    fn init(self, align_self: AlignSelf, key: String) {
        self.get_item_align(align_self, key);
    }

    fn get_layout_classes(self, layouts_prop: Vec<ItemLayout>) -> String {
        let mut layouts = layouts_prop
            .into_iter()
            .map(|layout| self.get_layout(layout))
            .collect::<String>();

        layouts.truncate(layouts.len() - 1);
        layouts
    }

    fn get_layout(self, item_layout: ItemLayout) -> String {
        match item_layout {
            ItemLayout::ItXs(size) => format!("it-xs-{} ", size),
            ItemLayout::ItS(size) => format!("it-s-{} ", size),
            ItemLayout::ItM(size) => format!("it-m-{} ", size),
            ItemLayout::ItL(size) => format!("it-l-{} ", size),
            ItemLayout::ItXl(size) => format!("it-xl-{} ", size),
        }
    }

    fn get_item_align(self, align: AlignSelf, key: String) {
        let value = match align {
            AlignSelf::Auto => "auto".to_string(),
            AlignSelf::Baseline => "baseline".to_string(),
            AlignSelf::Center => "center".to_string(),
            AlignSelf::FlexStart => "flex-start".to_string(),
            AlignSelf::FlexEnd => "flex-end".to_string(),
            AlignSelf::Stretch => "stretch".to_string(),
        };

        create_style(String::from("align-self"), value, format!("item-{}", key));
    }
}

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn should_create_item() {
    let props_item = Props {
        layouts: vec![ItemLayout::ItXs(12)],
        align_self: AlignSelf::Center,
        class_name: "item-test".to_string(),
        id: "item-id-test".to_string(),
        onclick_signal: Callback::noop(),
        children: Children::new(vec![html! {
            <div id="item">{"Item"}</div>
        }]),
    };

    let item: App<Item> = App::new();

    item.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props_item,
    );

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
        class_name: "item-test".to_string(),
        id: "item-id-test".to_string(),
        onclick_signal: on_add_item_div,
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
