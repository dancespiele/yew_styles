use crate::styles::helpers::{get_palette, get_size, Palette, Size};
use stylist::{css, StyleSource};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::start_app;
use gloo::utils;

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
/// use yew_styles::styles::helpers::{Palette, Size};
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
    /// Type spinner palette. Default `Palette::Standard`
    #[prop_or(Palette::Standard)]
    pub spinner_palette: Palette,
    /// Three diffent spinner standard sizes. Default `Size::Medium`
    #[prop_or(Size::Medium)]
    pub spinner_size: Size,
    /// General property to get the ref of the component
    #[prop_or_default]
    pub code_ref: NodeRef,
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
}

impl Component for Spinner {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: *ctx.props(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props = *ctx.props();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        get_spinner_type(ctx.props().clone())
    }
}

fn get_spinner_type(props: Props) -> Html {
    match props.spinner_type {
        SpinnerType::Plane => render_spinner_type("sk-plane", 0, props),
        SpinnerType::Chase => render_spinner_type("sk-chase", 6, props),
        SpinnerType::Bounce => render_spinner_type("sk-bounce", 2, props),
        SpinnerType::Wave => render_spinner_type("sk-wave", 5, props),
        SpinnerType::Pulse => render_spinner_type("sk-pulse", 0, props),
        SpinnerType::Flow => render_spinner_type("sk-flow", 3, props),
        SpinnerType::Swing => render_spinner_type("sk-swing", 2, props),
        SpinnerType::Circle => render_spinner_type("sk-circle", 12, props),
        SpinnerType::CircleFade => render_spinner_type("sk-circle-fade", 12, props),
        SpinnerType::Grid => render_spinner_type("sk-grid", 9, props),
        SpinnerType::Fold => render_spinner_type("sk-fold", 4, props),
        SpinnerType::Wander => render_spinner_type("sk-wander", 6, props),
    }
}

fn render_spinner_type(spinner_type: &str, dots: u8, props: Props) -> Html {
    let mut vdots: Vec<Html> = vec![];
    let mut i = 0;

    while i < dots {
        vdots.push(html! {
            <div class={if spinner_type == "sk-wave" {
                format!("{}-rect", spinner_type)
            } else if spinner_type == "sk-grid" || spinner_type == "sk-fold" || spinner_type == "sk-wander" {
                format!("{}-cube", spinner_type)
            } else {
                format!("{}-dot", spinner_type)
            }}></div>
        });

        i += 1;
    }
    html! {
        <div
            class={classes!(spinner_type.to_owned(), get_palette(props.spinner_palette), get_size(props.spinner_size), props.class_name, props.styles)}
            ref={props.code_ref}
            id={props.id}
            key={props.key}
        >
            {vdots.into_iter().collect::<Html>()}
        </div>
    }
}

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn should_create_spinner() {
    impl Default for Props {
        fn default() -> Self {
            Self {
                spinner_palette: Palette::Clean,
                spinner_type: SpinnerType::Circle,
                spinner_size: Size::Medium,
                code_ref: NodeRef::default(),
                key: String::from("dropdown-1"),
                class_name: String::from("class-test"),
                id: String::from("id-test"),
                styles: css!("font-size: 50px;"),
            }
        }
    }

    start_app::<Spinner>();

    let content_element = utils::document().get_element_by_id("id-test").unwrap();
    assert_eq!(content_element.id(), "id-test".to_string());
}
