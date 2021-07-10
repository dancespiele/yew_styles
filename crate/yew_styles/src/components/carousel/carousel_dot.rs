use crate::styles::{get_palette, Palette};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};
use yew_assets::object_assets::{ObjectAssets, ObjectIcon};

/// # Carousel Dots
///
/// ## Features required
///
/// carousel
///
/// ## Example
///
/// ```rust
/// use super::highlighters::get_carousel;
/// use yew::prelude::*;
/// use yew::services::ConsoleService;
/// use yew::utils::document;
/// use yew_prism::Prism;
/// use yew_styles::carousel::{Carousel, CarouselControls, CarouselDot, CarouselImage};
/// use yew_styles::styles::Size;
///
/// pub struct CarouselPage {
///     link: ComponentLink<Self>,
///     images: Vec<&'static str>,
///     active_image: Vec<bool>,
/// }
///
/// pub enum Msg {
///     ChangeImage(usize),
///     Scroll(WheelEvent),
///     ShowScroll,
///     HideScroll,
///     Prev,
///     Next,
/// }
///
/// impl Component for CarouselPage {
///     type Message = Msg;
///     type Properties = ();
///
///     fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
///         Self {
///             link,
///             images: vec!["/slide_1.jpg", "/slide_2.jpg", "/slide_3.jpg"],
///             active_image: vec![true, false, false],
///         }
///     }
///
///     fn update(&mut self, msg: Self::Message) -> ShouldRender {
///         match msg {
///             Msg::ChangeImage(image_index) => {
///                 for (i, _) in self.active_image.clone().into_iter().enumerate() {
///                     self.active_image[i] = false;
///                 }
///
///                 self.active_image[image_index] = true;
///             }
///             Msg::Prev => {
///                 let len = self.active_image.len();
///                 let index_opt = self.active_image.to_vec().into_iter().position(|ai| ai);
///                 for (i, _) in self.active_image.clone().into_iter().enumerate() {
///                     self.active_image[i] = false;
///                 }
///
///                 if let Some(index) = index_opt {
///                     if index == 0 {
///                         self.active_image[len - 1] = true
///                     } else {
///                         self.active_image[index - 1] = true
///                     }
///                 } else {
///                     ConsoleService::error("no image active")
///                 }
///             }
///
///             Msg::Next => {
///                 let len = self.active_image.len();
///                 let index_opt = self.active_image.to_vec().into_iter().position(|ai| ai);
///
///                 for (i, _) in self.active_image.clone().into_iter().enumerate() {
///                     self.active_image[i] = false;
///                 }
///
///                 if let Some(index) = index_opt {
///                     if index == len - 1 {
///                         self.active_image[0] = true
///                     } else {
///                         self.active_image[index + 1] = true
///                     }
///                 } else {
///                     ConsoleService::error("no image active")
///                 }
///             }
///             Msg::Scroll(wheel_event) => {
///                 let len = self.active_image.len();
///                 let index_opt = self.active_image.to_vec().into_iter().position(|ai| ai);
///                 for (i, _) in self.active_image.clone().into_iter().enumerate() {
///                     self.active_image[i] = false;
///                 }
///
///                 if wheel_event.delta_y() > 0.00 {
///                     if let Some(index) = index_opt {
///                         if index == 0 {
///                             self.active_image[len - 1] = true
///                         } else {
///                             self.active_image[index - 1] = true
///                         }
///                     } else {
///                         ConsoleService::error("no image active")
///                     }
///                 } else if let Some(index) = index_opt {
///                     if index == len - 1 {
///                         self.active_image[0] = true
///                     } else {
///                         self.active_image[index + 1] = true
///                     }
///                 } else {
///                     ConsoleService::error("no image active")
///                 }
///             }
///             Msg::ShowScroll => {
///                 let body_style = document().body().unwrap().style();
///                 body_style.set_property("overflow", "hidden").unwrap();
///             }
///             Msg::HideScroll => {
///                 let body_style = document().body().unwrap().style();
///                 body_style.set_property("overflow", "scroll").unwrap();
///             }
///         }
///
///         true
///     }
///
///     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
///         false
///     }
///
///     fn view(&self) -> Html {
///         html! {
///             <div>
///                 <Carousel
///                     class_name="fill-background"
///                     onwheel_signal= self.link.callback(Msg::Scroll)
///                     onmouseover_signal= self.link.callback(|_| Msg::ShowScroll)
///                     onmouseleave_signal= self.link.callback(|_| Msg::HideScroll)>
///                     {get_images(self.images.to_vec(), self.active_image.to_vec())}
///                     {get_dots(self.active_image.to_vec(), self.link.clone())}
///                     {get_controls(self.link.clone())}
///                 </Carousel>
///             </div>
///         }
///     }
/// }
///
/// fn get_images(images: Vec<&str>, active_image: Vec<bool>) -> Html {
///     images
///         .into_iter()
///         .enumerate()
///         .map(|(index, image)| {
///             html! {
///                 <CarouselImage active=active_image[index] img_src=image/>
///             }
///         })
///         .collect::<Html>()
/// }
///
/// fn get_dots(active_image: Vec<bool>, link: ComponentLink<CarouselPage>) -> Html {
///     let mut dot = vec![];
///
///     for (i, _) in active_image.clone().into_iter().enumerate() {
///         dot.push(html! {
///             <CarouselDot active=active_image[i] onclick_signal = link.callback(move |_| Msg::ChangeImage(i))>
///                 <CommunicationAssets size=("12".to_string(), "12".to_string()) icon=CommunicationIcon::Smile/>
///             </CarouselDot>
///         })
///     }
///
///     dot.into_iter().collect::<Html>()
/// }
///
/// fn get_controls(link: ComponentLink<App>) -> Html {
///     html! {
///         <CarouselControls
///             controls_size=Size::Small
///             prev_signal=link.callback(|_| Msg::Prev)
///             next_signal=link.callback(|_| Msg::Next)/>
///     }
/// }
/// ```
pub struct CarouselDot {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    /// Click event for carousel dot. Required
    pub onclick_signal: Callback<MouseEvent>,
    /// Type botton style. Default `Palette::Standard`
    #[prop_or(Palette::Standard)]
    pub carousel_dot_palette: Palette,
    /// If the dot is active to add active style
    #[prop_or(false)]
    pub active: bool,
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
    /// In case that children is not included will add a dot icon by default
    #[prop_or_default]
    pub children: Option<Children>,
}

pub enum Msg {
    DotClicked(MouseEvent),
}

impl Component for CarouselDot {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DotClicked(mouse_event) => {
                self.props.onclick_signal.emit(mouse_event);
            }
        }

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
        html! {
            <div
                class=classes!(
                    "carousel-dot",
                    self.props.class_name.clone(),
                    get_palette(self.props.carousel_dot_palette.clone()),
                    if self.props.active {
                        "active"
                    } else {
                        ""
                    },
                    self.props.class_name.clone(),
                )
                id={self.props.id.clone()}
                key=self.props.key.clone()
                ref=self.props.code_ref.clone()
                onclick=self.link.callback(Msg::DotClicked)
            >
            {
                if let Some(children) = self.props.children.clone() {
                    html! {
                        <>{children }</>
                    }
                } else {
                    html!{
                        <ObjectAssets size=("12".to_string(), "12".to_string()) icon=ObjectIcon::Circle class_name="carousel-dot-assets"/>
                    }
                }
            }
            </div>
        }
    }
}

#[wasm_bindgen_test]
fn should_create_carousel_dot_component() {
    let props = Props {
        code_ref: NodeRef::default(),
        class_name: String::from("test-carousel"),
        id: String::from("carousel-id-test"),
        carousel_dot_palette: Palette::Standard,
        active: false,
        onclick_signal: Callback::noop(),
        key: "".to_string(),
        children: None,
    };

    let carousel: App<CarouselDot> = App::new();
    carousel.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let carousel_element = utils::document()
        .get_element_by_id("carousel-id-test")
        .unwrap();

    assert_eq!(carousel_element.id(), "carousel-id-test");
}
