use super::highlighters::get_spinner;
use inflector::Inflector;
use yew::prelude::*;
use yew_prism::Prism;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::spinner::{Spinner, SpinnerType};
use yew_styles::styles::{get_palette, get_size, Palette, Size};

pub struct SpinnerPage;

impl Component for SpinnerPage {
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
                <h1>{"Spinner Component"}</h1>

                <h2>{"Features required"}</h2>
                <span><code>{"spinner"}</code></span>

                <h2>{"Code example"}</h2>
                <Prism
                    code=get_spinner()
                    language="rust"
                />

                <h2>{"Properties"}</h2>
                <ul>
                    <li><b>{"spinner_type: "}</b>{"spinner type. Options included in "}<code>{"SpinnerType"}</code>{". Default "}<code>{"Circle"}</code>{"."}</li>
                    <li><b>{"spinner_palette: "}</b>{"type spinner palette. Options included in "}<code>{"Pallete"}</code>{". Default "}<code>{"Standard"}</code>{"."}</li>
                    <li><b>{"spinner_size: "}</b>{"three diffent spinner standard sizes. Options included in "}<code>{"Size"}</code>{". Default "}<code>{"Medium"}</code>{"."}</li>
                    <li><b>{"key: "}</b>{"general property to add keys."}</li>
                    <li><b>{"code_ref: "}</b>{"general property to get the ref of the component."}</li>
                    <li><b>{"id: "}</b>{"general property to add custom id."}</li>
                    <li><b>{"class_name: "}</b>{"general property to add custom class styles."}</li>
                </ul>

                <h2>{"Sources"}</h2>
                <span>{"The spinners are created by "}<a href="https://tobiasahlin.com/spinkit/" target="_blank">{"Tobias Ahlin"}</a>{" and all of them have the most permissive license (MIT)"}</span>

                <h2>{"Visual exmples"}</h2>

                <h3>{"Sizes"}</h3>
                <Container wrap=Wrap::Wrap direction=Direction::Row>
                    {get_sizes()}
                </Container>

                <h3>{"Palettes"}</h3>
                <Container wrap=Wrap::Wrap direction=Direction::Row>
                    {get_palettes()}
                </Container>
                <h3>{"Spinner types"}</h3>
                <Container wrap=Wrap::Wrap direction=Direction::Row>
                    {get_spinner_types()}
                </Container>
            </>
        }
    }
}

fn get_sizes() -> Html {
    let sizes: Vec<Size> = vec![Size::Small, Size::Medium, Size::Big];

    sizes
        .into_iter()
        .map(|size| {
            html! {
                <Item layouts=vec![ItemLayout::ItXs(12), ItemLayout::ItS(6), ItemLayout::ItL(4)]>
                    <h4>{get_size(size.clone()).to_pascal_case()}</h4>
                    <Spinner spinner_size=size spinner_palette=Palette::Info/>
                </Item>
            }
        })
        .collect::<Html>()
}

fn get_palettes() -> Html {
    let palettes: Vec<Palette> = vec![
        Palette::Standard,
        Palette::Primary,
        Palette::Secondary,
        Palette::Info,
        Palette::Link,
        Palette::Success,
        Palette::Warning,
        Palette::Danger,
        Palette::Clean,
    ];

    palettes
        .into_iter()
        .map(|palette| {
            if palette != Palette::Clean {
                html!{
                    <Item layouts=vec![ItemLayout::ItXs(12), ItemLayout::ItS(6), ItemLayout::ItL(4)]>
                    <h4>{get_palette(palette.clone()).to_pascal_case()}</h4>
                    <Spinner spinner_palette=palette/>
                </Item>
                }

            } else {
                html!{
                    <Item layouts=vec![ItemLayout::ItXs(12), ItemLayout::ItS(6), ItemLayout::ItL(4)]>
                        <h4>{get_palette(palette.clone()).to_pascal_case()}</h4>
                        <div class="spinner-clean">
                            <Spinner spinner_palette=palette/>
                        </div>
                    </Item>
                }
            }
        })
        .collect::<Html>()
}

fn get_spinner_types() -> Html {
    let spinner_types: Vec<SpinnerType> = vec![
        SpinnerType::Plane,
        SpinnerType::Chase,
        SpinnerType::Bounce,
        SpinnerType::Wave,
        SpinnerType::Pulse,
        SpinnerType::Flow,
        SpinnerType::Swing,
        SpinnerType::Circle,
        SpinnerType::CircleFade,
        SpinnerType::Grid,
        SpinnerType::Fold,
        SpinnerType::Wander,
    ];

    spinner_types
        .into_iter()
        .map(|spinner_type| {
            html! {
                <Item layouts=vec![ItemLayout::ItXs(12), ItemLayout::ItS(6), ItemLayout::ItL(4)]>
                    <h4>{get_spinner_type(spinner_type.clone()).to_pascal_case()}</h4>
                    <Spinner spinner_type=spinner_type/>
                </Item>
            }
        })
        .collect::<Html>()
}

fn get_spinner_type(spinner_type: SpinnerType) -> String {
    match spinner_type {
        SpinnerType::Plane => String::from("Plane"),
        SpinnerType::Chase => String::from("Chase"),
        SpinnerType::Bounce => String::from("Bounce"),
        SpinnerType::Wave => String::from("Wave"),
        SpinnerType::Pulse => String::from("Pulse"),
        SpinnerType::Flow => String::from("Flow"),
        SpinnerType::Swing => String::from("Swing"),
        SpinnerType::Circle => String::from("Circle"),
        SpinnerType::CircleFade => String::from("CircleFade"),
        SpinnerType::Grid => String::from("Grid"),
        SpinnerType::Fold => String::from("Fold"),
        SpinnerType::Wander => String::from("Wander"),
    }
}
