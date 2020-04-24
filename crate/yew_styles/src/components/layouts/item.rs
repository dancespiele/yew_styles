use crate::utils::{create_style, get_random_string};
use yew::prelude::*;

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
    Clicked,
}

/// # Item component
///
/// The layouts in yew styles is base in flexbox
/// you can fine more information about the properties options
/// [here](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Flexible_Box_Layout/Basic_Concepts_of_Flexbox)
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
    hash: String,
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
    /// Percent of the layout that will take the item.
    pub layouts: Vec<ItemLayout>,
    #[prop_or(AlignSelf::Auto)]
    /// Align the item itself
    pub align_self: AlignSelf,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// Click event for the item
    #[prop_or(Callback::noop())]
    pub onsignal: Callback<()>,
    pub children: Children,
}

impl Component for Item {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let hash = get_random_string(10);

        Item { link, props, hash }
    }

    fn mounted(&mut self) -> ShouldRender {
        ItemModel.init(self.props.align_self.clone(), self.hash.clone());

        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.props.onsignal.emit(());
            }
        };

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let item_props = ItemProps::from(self.props.clone());

        html! {
            <div
                class=format!("item item-{} {} {}", self.hash, item_props.layouts_classes, item_props.class_name)

                onclick=self.link.callback(|_| Msg::Clicked)
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
    fn init(self, align_self: AlignSelf, hash: String) {
        self.get_item_align(align_self, hash);
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

    fn get_item_align(self, align: AlignSelf, hash: String) {
        let value = match align {
            AlignSelf::Auto => "auto".to_string(),
            AlignSelf::Baseline => "baseline".to_string(),
            AlignSelf::Center => "center".to_string(),
            AlignSelf::FlexStart => "flex-start".to_string(),
            AlignSelf::FlexEnd => "flex-end".to_string(),
            AlignSelf::Stretch => "stretch".to_string(),
        };

        create_style(String::from("align-self"), value, format!("item-{}", hash));
    }
}
