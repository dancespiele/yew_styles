use super::highlighters::get_stylist;
use lipsum::lipsum;
use stylist::css;
use yew::prelude::*;
use yew_prism::Prism;
use yew_styles::text::{Text, TextType};

pub struct StylistPage;

impl Component for StylistPage {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{"Style components with "}<code>{"stylist-rs"}</code></h1>

                <h2>{"Code example"}</h2>
                <Prism
                    code=get_stylist()
                    language="rust"
                />

                <h2>{"Sources"}</h2>
                <span>{"More info about "}<code>{"stylist-rs"}</code>{" please click "}<a href="https://github.com/futursolo/stylist-rs" target="_blank">{"here"}</a>{"."}</span>

                <h2>{"Visual exmples"}</h2>
                <Text
                    text_type=TextType::Plain
                    plain_text=lipsum(8)
                    html_text=None
                    styles=css!("
                        color: #FFF;
                        background-color: #918d94;
                    ")
                />
            </>
        }
    }
}
