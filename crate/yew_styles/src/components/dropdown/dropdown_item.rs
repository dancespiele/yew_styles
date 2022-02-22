use gloo::utils;
use stylist::{css, StyleSource};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::start_app_with_props;

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
/// use yew_styles::styles::helpers::{Palette, Size, Style};
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
pub struct DropdownItem;

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
    /// Set css styles directly in the component
    #[prop_or(css!(""))]
    pub styles: StyleSource<'static>,
    pub children: Children,
}

pub enum Msg {
    Clicked(MouseEvent),
}

impl Component for DropdownItem {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(mouse_event) => {
                ctx.props().onclick_signal.emit(mouse_event);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            onclick_signal,
            key,
            class_name,
            id,
            styles,
            children,
        } = &ctx.props();

        html! {
            <li
                class={classes!("dropdown-item", class_name.clone(), styles.clone())}
                id={id.clone()}
                key={key.clone()}
                onclick={ctx.link().callback(Msg::Clicked)}
            >{children.clone()}</li>
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
        styles: css!("background-color: #918d94;"),
        children: Children::new(vec![html! {
            <div id="item">{"Item"}</div>
        }]),
    };

    start_app_with_props::<DropdownItem>(dropdown_item_props);

    let content_element = utils::document().get_element_by_id("item").unwrap();
    assert_eq!(content_element.text_content().unwrap(), "Item".to_string());
}
