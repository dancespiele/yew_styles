use yew::prelude::*;
use yew_styles::dropdown::{Dropdown, DropdownItem};
use yew_styles::styles::Size;

pub struct DropDownPage {
    link: ComponentLink<Self>,
    menu: String,
}

pub enum Msg {
    ChangeMenu(String),
}

impl Component for DropDownPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            menu: String::from(""),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeMenu(menu) => {
                self.menu = menu;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{"Dropdown Component"}</h1>

                <h2>{"Visual Example"}</h2>
                <Dropdown
                    main_content=html!{<span>{"Menu"}</span>}
                    dropdown_size=Size::Medium
                    >
                    <DropdownItem
                        onclick_signal=self.link.callback(|_| Msg::ChangeMenu(String::from("Menu 1")))>{"Menu 1"}</DropdownItem>
                    <DropdownItem
                        onclick_signal=self.link.callback(|_| Msg::ChangeMenu(String::from("Menu 2")))>{"Menu 2"}</DropdownItem>
                    <DropdownItem
                        onclick_signal=self.link.callback(|_| Msg::ChangeMenu(String::from("Menu 3")))>{"Menu 3"}</DropdownItem>
                </Dropdown>

                <div>{self.menu.clone()}</div>
            </>
        }
    }
}
