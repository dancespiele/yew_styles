use crate::styles::{get_pallete, Palette};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum SpinnerType {
    Plane,
    Chase,
    Bounce,
    Wave,
    Pulse,
    Flow,
    Swing,
    Circle,
    CircleFade,
    Grid,
    Fold,
    Wander,
}

pub struct Spinner {
    props: Props,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    spinner_type: SpinnerType,
    spinner_palette: Palette,
}

impl Component for Spinner {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            return true;
        }
        false
    }

    fn view(&self) -> Html {
        get_spinner_type(
            self.props.spinner_type.clone(),
            self.props.spinner_palette.clone(),
        )
    }
}

fn get_spinner_type(spinner_type: SpinnerType, spinner_palette: Palette) -> Html {
    match spinner_type {
        SpinnerType::Plane => render_spinner_type("sk-plane", 0, spinner_palette),
        SpinnerType::Chase => render_spinner_type("sk-chase", 6, spinner_palette),
        SpinnerType::Bounce => render_spinner_type("sk-bounce", 2, spinner_palette),
        SpinnerType::Wave => render_spinner_type("sk-wave", 5, spinner_palette),
        SpinnerType::Pulse => render_spinner_type("sk-pulse", 0, spinner_palette),
        SpinnerType::Flow => render_spinner_type("sk-flow", 3, spinner_palette),
        SpinnerType::Swing => render_spinner_type("sk-swing", 2, spinner_palette),
        SpinnerType::Circle => render_spinner_type("sk-circle", 12, spinner_palette),
        SpinnerType::CircleFade => render_spinner_type("sk-circle-fade", 12, spinner_palette),
        SpinnerType::Grid => render_spinner_type("sk-grid", 9, spinner_palette),
        SpinnerType::Fold => render_spinner_type("sk-fold", 4, spinner_palette),
        SpinnerType::Wander => render_spinner_type("sk-wander", 6, spinner_palette),
    }
}

fn render_spinner_type(spinner_type: &str, dots: u8, spinner_palette: Palette) -> Html {
    let mut vdots: Vec<Html> = vec![];
    let mut i = 0;

    while i < dots {
        vdots.push(html! {
            <div class=format!("{}-dot", spinner_type)></div>
        });

        i += 1;
    }
    html! {
        <div class=(spinner_type, get_pallete(spinner_palette))>
            {vdots.into_iter().map(|vdot| vdot).collect::<Html>()}
        </div>
    }
}
