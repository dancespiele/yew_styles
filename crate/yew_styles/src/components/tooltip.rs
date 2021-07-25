use crate::styles::{get_palette, get_size, Palette, Position, Size, Style};
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    /// Tooltip palette. Default `Palette::Standard`
    #[prop_or(Palette::Standard)]
    pub tooltip_palette: Palette,
    /// Tooltip style. Default `Style::Regular`
    #[prop_or(Style::Regular)]
    pub tooltip_style: Style,
    /// Tooltip size. Default `Size::Medium`
    #[prop_or(Size::Medium)]
    pub tooltip_size: Size,
    #[prop_or_default]
    pub key: String,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    /// Tooltip position over targeted element. Left, Right, Above, Below
    pub tooltip_position: Position,
    /// Show the content of tooltip
    pub content: Html,
    pub children: Children,
}

pub struct Tooltip {
    props: Props,
    link: ComponentLink<Self>,
    show_tooltip: bool,
}

pub enum Msg {
    TargetOver,
    TargetLeave,
}

impl Component for Tooltip {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            show_tooltip: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TargetOver => self.show_tooltip = true,
            Msg::TargetLeave => self.show_tooltip = false,
        };

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
        let tooltip = html! {
            <div class=classes!(
                "tooltip",
                get_position(self.props.tooltip_position),
                props.tooltip_palette,
                props.tooltip_style,
                props.tooltip_size,
                props.class_name
        )>
             {self.props.content}
            </div>
        };

        html! {
            <div class="tooltip_container",
                onmouseover = self.link.callback(|_| Msg::TargetOver)
                onmouseleave = self.link.callback(|_| Msg::TargetLeave)
            >
                {if self.show_tooltip {
                    tooltip
                }else {
                    html!{}
                }}
                {props.children}
            </div>
        }
    }
}

fn get_position(position: Position) -> String {
    match position {
        Position::Left => String::from("left"),
        Position::Right => String::from("rigth"),
        Position::Above => String::from("above"),
        Position::Below => String::from("below"),
    }
}
