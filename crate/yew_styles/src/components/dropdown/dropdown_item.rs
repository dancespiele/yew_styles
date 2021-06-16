use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

/// # Dropdown Item component
///
/// ## Features required
///
/// dropdown
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_styles::dropdown::{Dropdown, DropdownItem};
/// use yew_styles::styles::{Palette, Size, Style};
///
/// pub struct DropDownPage {
///     link: ComponentLink<Self>,
///     menu: String,
/// }
///
/// pub enum Msg {
///     ChangeMenu(String),
/// }
///
/// impl Component for DropDownPage {
///     type Message = Msg;
///     type Properties = ();
///
///     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
///         Self {
///             link,
///             menu: String::from(""),
///         }
///     }
///
///     fn update(&mut self, msg: Self::Message) -> ShouldRender {
///         match msg {
///             Msg::ChangeMenu(menu) => {
///                 self.menu = menu;
///             }
///         }
///         true
///     }
///
///     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
///         false
///     }
///
///     fn view(&self) -> Html {
///         <>
///             <Dropdown
///                 main_content=html!{<span>{"Menu"}</span>}
///                 dropdown_style=Style::Outline
///                 dropdown_palette=Palette::Primary
///                 >
///                 <DropdownItem
///                     onclick_signal=link.callback(|_| Msg::ChangeMenu(String::from("Menu 1")))>{"Menu 1"}</DropdownItem>
///                 <DropdownItem
///                     onclick_signal=link.callback(|_| Msg::ChangeMenu(String::from("Menu 2")))>{"Menu 2"}</DropdownItem>
///                 <DropdownItem
///                     onclick_signal=link.callback(|_| Msg::ChangeMenu(String::from("Menu 3")))>{"Menu 3"}</DropdownItem>
///             </Dropdown>
///             <span>{self.menu.clone()}</span>
///         </>
///     }
/// }
/// ```
pub struct DropdownItem {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or(Callback::noop())]
    /// Click event for dropdown item
    pub onclick_signal: Callback<MouseEvent>,
    /// General property to add keys
    #[prop_or_default]
    pub key: String,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    pub children: Children,
}

pub enum Msg {
    Clicked(MouseEvent),
}

impl Component for DropdownItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked(mouse_event) => {
                self.props.onclick_signal.emit(mouse_event);
            }
        }
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
        html! {
            <li
                class=classes!("dropdown-item", self.props.class_name.clone())
                id=self.props.id.clone()
                key=self.props.key.clone()
                onclick=self.link.callback(Msg::Clicked)
            >{self.props.children.clone()}</li>
        }
    }
}

#[wasm_bindgen_test]
fn should_create_dropdown_item() {
    let dropdown_item_props = Props {
        onclick_signal: Callback::noop(),
        key: String::from("dropdown-item-1"),
        class_name: String::from("class-test"),
        id: String::from("id-test"),
        children: Children::new(vec![html! {
            <div id="item">{"Item"}</div>
        }]),
    };

    let dropdown_item: App<DropdownItem> = App::new();

    dropdown_item.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        dropdown_item_props,
    );

    let content_element = utils::document().get_element_by_id("item").unwrap();
    assert_eq!(content_element.text_content().unwrap(), "Item".to_string());
}
