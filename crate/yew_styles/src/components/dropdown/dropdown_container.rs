use crate::styles::colors::{darker, get_styles};
use crate::styles::helpers::{get_palette, get_size, get_style, Palette, Size, Style};
use stylist::{css, StyleSource, YieldStyle};
use wasm_bindgen_test::*;
use yew::prelude::*;

/// # Dropdown Container component
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
pub struct Dropdown {
    active: bool,
    props: Props,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    /// clickeable content to show the dropdown. Required
    pub main_content: Html,
    /// Palette style color for dropdown
    #[prop_or(Palette::Standard)]
    pub dropdown_palette: Palette,
    /// Style for dropdown
    #[prop_or(Style::Regular)]
    pub dropdown_style: Style,
    /// Size for dropdown
    #[prop_or(Size::Medium)]
    pub dropdown_size: Size,
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
    ShowDropdown,
}

impl YieldStyle for Dropdown {
    fn style_from(&self) -> StyleSource<'static> {
        let style = get_style(self.props.dropdown_style);
        let styles = get_styles();
        let color = styles
            .get(style.as_str())
            .unwrap()
            .iter()
            .find(|pallete| pallete.name == get_palette(self.props.dropdown_palette))
            .unwrap();

        css!(
            r#"
                cursor: pointer;
                padding-left: 0;
                background: ${background};
                color: ${color};
                border: ${border_color};

                .dropdown-item:focus {
                    background-color: ${focus_background};
                }

                .dropdown_item:hover {
                    background-color: ${hover_background};
                }

                .dropdown_item:active, dropdown_item.active {
                    background-color: ${active_background};
                }

                ul {
                    padding: 0;
                    marging-top: 3px;
                }

                &.small .main-content, li {
                    padding: 2px 5px 2px;
                }

                &.big .main-content, li {
                    padding: 12px 15px 12px;
                }

                .main-content, li {
                    padding: 8px 10px 8px;
                }
            "#,
            background = color.background,
            color = color.color,
            border_color = color.border_color,
            focus_background = darker(&color.background, -5.0),
            hover_background = darker(&color.background, -10.0),
            active_background = darker(&color.background, -15.0),
        )
    }
}

impl Component for Dropdown {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            active: false,
            props: *ctx.props(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowDropdown => {
                self.active = !self.active;
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props = *ctx.props();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            main_content,
            dropdown_palette,
            dropdown_size,
            dropdown_style,
            key,
            id,
            styles,
            class_name,
            children,
        } = &ctx.props();

        html! {
            <div
                class={classes!("dropdown", class_name.clone(), get_style(dropdown_style.clone()), get_palette(dropdown_palette.clone()), get_size(dropdown_size.clone()), styles.clone())}
                id={id.clone()}
                key={key.clone()}
                onclick={ctx.link().callback(|_| Msg::ShowDropdown)}
                >
                <div class="main-content">{main_content.clone()}</div>
                {get_items(self.active, children.clone())}
            </div>
        }
    }
}

fn get_items(active: bool, children: Children) -> Html {
    if active {
        html! {
            <ul>
                {children.clone()}
            </ul>
        }
    } else {
        html! {}
    }
}

wasm_bindgen_test_configure!(run_in_browser);

// #[wasm_bindgen_test]
// fn should_create_dropdown_container() {
//     let dropdown_container_props = Props {
//         main_content: html! {<div id="test">{"test"}</div>},
//         dropdown_palette: Palette::Clean,
//         dropdown_size: Size::Medium,
//         dropdown_style: Style::Outline,
//         key: String::from("dropdown-1"),
//         class_name: String::from("class-test"),
//         id: String::from("id-test"),
//         styles: css!("background-color: #918d94;"),
//         children: Children::new(vec![html! {
//             <div id="item">{"Item"}</div>
//         }]),
//     };

//     start_app::<Dropdown>();

//     dropdown_container.mount_with_props(
//         utils::document().get_element_by_id("output").unwrap(),
//         dropdown_container_props,
//     );

//     let content_element = utils::document().get_element_by_id("test").unwrap();
//     assert_eq!(content_element.text_content().unwrap(), "test".to_string());
// }
