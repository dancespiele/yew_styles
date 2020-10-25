use crate::styles::{get_palette, get_size, Palette, Size};
use yew::prelude::*;

/// # Spinner component
///
/// ## Features required
///
/// spinner
///
/// ## Example
///
/// ```rust
///
///  use yew::prelude::*;
/// use yew_styles::spinner::{Spinner, SpinnerType};
/// use yew_styles::styles::{Palette, Size};
///
/// pub struct SpinnerExample;
///
/// impl Component for SpinnerExample {
///     type Message = ();
///     type Properties = ();
///
///     fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
///         Self {}
///     }
///
///     fn update(&mut self, _msg: Self::Message) -> ShouldRender {
///         false
///     }
///
///     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
///         false
///     }
///
///     fn view(&self) -> Html {
///         html! {
///             <div>
///                 <Spinner
///                     spinner_type=SpinnerType::Circle
///                     spinner_size=Size::Medium
///                     spinner_palette=Palette::Info/>
///             </div>
///         }
///     }
/// }
/// ```
pub struct Spinner {
    props: Props,
}

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

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    /// Spinner type. Default `SpinnerType::Circle`
    #[prop_or(SpinnerType::Circle)]
    pub spinner_type: SpinnerType,
    /// Type spinner style. Default `Palette::Standard`
    #[prop_or(Palette::Standard)]
    pub spinner_palette: Palette,
    /// Three diffent spinner standard sizes. Default `Size::Medium`
    #[prop_or(Size::Medium)]
    pub spinner_size: Size,
    /// General property to add keys
    #[prop_or_default]
    pub key: String,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
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
            self.props.spinner_size.clone(),
            self.props.class_name.clone(),
            self.props.id.clone(),
            self.props.key.clone(),
        )
    }
}

fn get_spinner_type(
    spinner_type: SpinnerType,
    spinner_palette: Palette,
    spinner_size: Size,
    class_name: String,
    id: String,
    key: String,
) -> Html {
    match spinner_type {
        SpinnerType::Plane => render_spinner_type(
            "sk-plane",
            0,
            spinner_palette,
            spinner_size,
            class_name,
            id,
            key,
        ),
        SpinnerType::Chase => render_spinner_type(
            "sk-chase",
            6,
            spinner_palette,
            spinner_size,
            class_name,
            id,
            key,
        ),
        SpinnerType::Bounce => render_spinner_type(
            "sk-bounce",
            2,
            spinner_palette,
            spinner_size,
            class_name,
            id,
            key,
        ),
        SpinnerType::Wave => render_spinner_type(
            "sk-wave",
            5,
            spinner_palette,
            spinner_size,
            class_name,
            id,
            key,
        ),
        SpinnerType::Pulse => render_spinner_type(
            "sk-pulse",
            0,
            spinner_palette,
            spinner_size,
            class_name,
            id,
            key,
        ),
        SpinnerType::Flow => render_spinner_type(
            "sk-flow",
            3,
            spinner_palette,
            spinner_size,
            class_name,
            id,
            key,
        ),
        SpinnerType::Swing => render_spinner_type(
            "sk-swing",
            2,
            spinner_palette,
            spinner_size,
            class_name,
            id,
            key,
        ),
        SpinnerType::Circle => render_spinner_type(
            "sk-circle",
            12,
            spinner_palette,
            spinner_size,
            class_name,
            id,
            key,
        ),
        SpinnerType::CircleFade => render_spinner_type(
            "sk-circle-fade",
            12,
            spinner_palette,
            spinner_size,
            class_name,
            id,
            key,
        ),
        SpinnerType::Grid => render_spinner_type(
            "sk-grid",
            9,
            spinner_palette,
            spinner_size,
            class_name,
            id,
            key,
        ),
        SpinnerType::Fold => render_spinner_type(
            "sk-fold",
            4,
            spinner_palette,
            spinner_size,
            class_name,
            id,
            key,
        ),
        SpinnerType::Wander => render_spinner_type(
            "sk-wander",
            6,
            spinner_palette,
            spinner_size,
            class_name,
            id,
            key,
        ),
    }
}

fn render_spinner_type(
    spinner_type: &str,
    dots: u8,
    spinner_palette: Palette,
    spinner_size: Size,
    class_name: String,
    id: String,
    key: String,
) -> Html {
    let mut vdots: Vec<Html> = vec![];
    let mut i = 0;

    while i < dots {
        vdots.push(html! {
            <div class=if spinner_type == "sk-wave" {
                format!("{}-rect", spinner_type)
            } else if spinner_type == "sk-grid" || spinner_type == "sk-fold" || spinner_type == "sk-wander" {
                format!("{}-cube", spinner_type)
            } else {
                format!("{}-dot", spinner_type)
            }></div>
        });

        i += 1;
    }
    html! {
        <div
            class=(spinner_type, get_palette(spinner_palette), get_size(spinner_size), class_name)
            id=id
            key=key
        >
            {vdots.into_iter().map(|vdot| vdot).collect::<Html>()}
        </div>
    }
}
