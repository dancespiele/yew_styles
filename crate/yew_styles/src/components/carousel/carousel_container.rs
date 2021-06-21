use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

/// # Carousel Component
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
///                 
///                 <Carousel class_name="fill-background">
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
/// fn get_dots(active_image: Vec<bool>, link: ComponentLink<App>) -> Html {
///     let mut dot = vec![];
///
///     for (i, _) in active_image.clone().into_iter().enumerate() {
///         dot.push(html! {
///             <CarouselDot active=active_image[i] onclick_signal = link.callback(move |_| Msg::ChangeImage(i))/>
///         })
///     }
///
///     dot.into_iter().collect::<Html>()
/// }
///
/// fn get_controls(link: ComponentLink<App>) -> Html {
///     html! {
///         <Carousel
///             class_name="fill-background"
///             onwheel_signal= self.link.callback(Msg::Scroll)
///             onmouseover_signal= self.link.callback(|_| Msg::ShowScroll)
///             onmouseleave_signal= self.link.callback(|_| Msg::HideScroll)>
///             {get_images(self.images.to_vec(), self.active_image.to_vec())}
///             {get_dots(self.active_image.to_vec(), self.link.clone())}
///             {get_controls(self.link.clone())}
///         </Carousel>
///     }
/// }
/// ```
pub struct Carousel {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    /// General property to get the ref of the component
    #[prop_or_default]
    pub code_ref: NodeRef,
    /// wheel event for carousel
    #[prop_or(Callback::noop())]
    pub onwheel_signal: Callback<WheelEvent>,
    /// mouse over event for carousel
    #[prop_or(Callback::noop())]
    pub onmouseover_signal: Callback<MouseEvent>,
    /// mouse leave event for carousel
    #[prop_or(Callback::noop())]
    pub onmouseleave_signal: Callback<MouseEvent>,
    /// General property to add keys
    #[prop_or_default]
    pub key: String,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    pub children: Children,
}

pub enum Msg {
    Wheel(WheelEvent),
    MouseOver(MouseEvent),
    MouseLeave(MouseEvent),
}

impl Component for Carousel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Wheel(wheel_event) => {
                self.props.onwheel_signal.emit(wheel_event);
            }
            Msg::MouseOver(mouse_event) => {
                self.props.onmouseover_signal.emit(mouse_event);
            }
            Msg::MouseLeave(mouse_event) => {
                self.props.onmouseleave_signal.emit(mouse_event);
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
                class=classes!("carousel-container", self.props.class_name.clone())
                id=self.props.id.clone()
                key=self.props.key.clone()
                onwheel=self.link.callback(Msg::Wheel)
                onmouseover=self.link.callback(Msg::MouseOver)
                onmouseleave=self.link.callback(Msg::MouseLeave)
                ref=self.props.code_ref.clone()
            >
                {self.props.children.clone()}
            </div>
        }
    }
}

#[wasm_bindgen_test]
fn should_create_carousel_container_component() {
    let props = Props {
        class_name: String::from("test-carousel"),
        id: String::from("carousel-id-test"),
        key: "".to_string(),
        code_ref: NodeRef::default(),
        onwheel_signal: Callback::noop(),
        onmouseover_signal: Callback::noop(),
        onmouseleave_signal: Callback::noop(),
        children: Children::new(vec![html! {<div id="result">{"result"}</div>}]),
    };

    let carousel: App<Carousel> = App::new();
    carousel.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let carousel_element = utils::document()
        .get_elements_by_class_name("carousel-container")
        .get_with_index(0)
        .unwrap();

    let child = carousel_element.first_element_child().unwrap();

    assert_eq!(child.id(), "result");
}
