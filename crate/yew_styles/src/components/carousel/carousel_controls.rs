use crate::styles::helpers::{get_palette, get_size, get_style, Palette, Size, Style};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::utils;
use yew_assets::controller_assets::{ControllerAssets, ControllerIcon};

/// # Carousel Controls
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
/// use yew_styles::styles::helpers::Size;
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
pub struct CarouselControls;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub prev_signal: Callback<MouseEvent>,
    pub next_signal: Callback<MouseEvent>,
    /// controls styles. Default `Style::Regular`
    #[prop_or(Style::Regular)]
    pub controls_style: Style,
    /// Type controls style. Default `Palette::Standard`
    #[prop_or(Palette::Standard)]
    pub controls_palette: Palette,
    /// Three diffent button standard sizes. Default `Size::Medium`
    #[prop_or(Size::Medium)]
    pub controls_size: Size,
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
}

pub enum Msg {
    PrevClicked(MouseEvent),
    NextClicked(MouseEvent),
}

impl Component for CarouselControls {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PrevClicked(mouse_event) => {
                ctx.props().prev_signal.emit(mouse_event);
            }
            Msg::NextClicked(mouse_event) => {
                ctx.props().next_signal.emit(mouse_event);
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            prev_signal,
            next_signal,
            controls_style,
            controls_palette,
            controls_size,
            code_ref,
            key,
            class_name,
            id,
        } = &ctx.props();

        html! {
            <div class="carousel-control"
                key={key.clone()}
                id={id.clone()}
                ref={code_ref.clone()}
            >
                <div
                    class="carousel-control-left-container"

                    onclick={ctx.link().callback(Msg::PrevClicked)}>
                    <ControllerAssets
                        size={("50".to_string(),"50".to_string())}
                        class_name={format!("carousel-control-left {} {} {} {}",
                            get_size(controls_size.clone()),
                            get_style(controls_style.clone()),
                            get_palette(controls_palette.clone()),
                            class_name.clone(),
                        )}
                        icon={ControllerIcon::ChevronLeft}
                    />
                </div>
                <div
                    class="carousel-control-right-container"
                    onclick={ctx.link().callback(Msg::NextClicked)}
                >
                    <ControllerAssets
                        size={("50".to_string(),"50".to_string())}
                        class_name={format!("carousel-control-right {} {} {} {}",
                            get_size(controls_size.clone()),
                            get_style(controls_style.clone()),
                            get_palette(controls_palette.clone()),
                            class_name.clone(),
                        )}
                        icon={ControllerIcon::ChevronRight}/>
                </div>
            </div>
        }
    }
}

// #[wasm_bindgen_test]
// fn should_create_carousel_controls_component() {
//     let props = Props {
//         code_ref: NodeRef::default(),
//         class_name: String::from("test-carousel"),
//         id: String::from("carousel-id-test"),
//         controls_palette: Palette::Standard,
//         controls_style: Style::Regular,
//         controls_size: Size::Medium,
//         next_signal: Callback::noop(),
//         prev_signal: Callback::noop(),
//         key: "".to_string(),
//     };

//     start_app::<CarouselControls>();
//     carousel.mount_with_props(
//         utils::document().get_element_by_id("output").unwrap(),
//         props,
//     );

//     let carousel_element = utils::document()
//         .get_element_by_id("carousel-id-test")
//         .unwrap();

//     assert_eq!(carousel_element.id(), "carousel-id-test");
// }
