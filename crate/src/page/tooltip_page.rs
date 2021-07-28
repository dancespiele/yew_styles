use yew::prelude::*;
use yew_styles::styles::Position;
use yew_styles::tooltip::Tooltip;

pub struct TooltipPage;

impl Component for TooltipPage {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
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
                <h1>{"Tooltip Component"}</h1>
                <Tooltip
                    tooltip_position=Position::Left
                    content=html!{<span>{"Left"}</span>}
                >
                    <p>{"Tooltip test left"}</p>
                </Tooltip>
                <Tooltip
                    tooltip_position=Position::Right
                    content=html!{<span>{"Right"}</span>}
                >
                    <p>{"Tooltip test right"}</p>
                </Tooltip>
                <Tooltip
                    tooltip_position=Position::Above
                    content=html!{<span>{"Above"}</span>}
                >
                    <p>{"Tooltip test above"}</p>
                </Tooltip>
                <Tooltip
                    tooltip_position=Position::Below
                    content=html!{<span>{"Below"}</span>}
                >
                    <p>{"Tooltip test below"}</p>
                </Tooltip>
            </>
        }
    }
}
