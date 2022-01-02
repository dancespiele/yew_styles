use super::highlighters::get_carousel;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::utils::document;
use yew_assets::communication_assets::{CommunicationAssets, CommunicationIcon};
use yew_prism::Prism;
use yew_styles::carousel::{Carousel, CarouselControls, CarouselDot, CarouselImage};
use yew_styles::styles::helpers::Size;

pub struct CarouselPage {
    link: ComponentLink<Self>,
    images: Vec<&'static str>,
    active_image: Vec<bool>,
}

pub enum Msg {
    ChangeImage(usize),
    Scroll(WheelEvent),
    ShowScroll,
    HideScroll,
    Prev,
    Next,
}

impl Component for CarouselPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            images: vec!["/slide_1.jpg", "/slide_2.jpg", "/slide_3.jpg"],
            active_image: vec![true, false, false],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeImage(image_index) => {
                for (i, _) in self.active_image.clone().into_iter().enumerate() {
                    self.active_image[i] = false;
                }

                self.active_image[image_index] = true;
            }
            Msg::Prev => {
                let len = self.active_image.len();
                let index_opt = self.active_image.to_vec().into_iter().position(|ai| ai);
                for (i, _) in self.active_image.clone().into_iter().enumerate() {
                    self.active_image[i] = false;
                }

                if let Some(index) = index_opt {
                    if index == 0 {
                        self.active_image[len - 1] = true
                    } else {
                        self.active_image[index - 1] = true
                    }
                } else {
                    ConsoleService::error("no image active")
                }
            }

            Msg::Next => {
                let len = self.active_image.len();
                let index_opt = self.active_image.to_vec().into_iter().position(|ai| ai);

                for (i, _) in self.active_image.clone().into_iter().enumerate() {
                    self.active_image[i] = false;
                }

                if let Some(index) = index_opt {
                    if index == len - 1 {
                        self.active_image[0] = true
                    } else {
                        self.active_image[index + 1] = true
                    }
                } else {
                    ConsoleService::error("no image active")
                }
            }
            Msg::Scroll(wheel_event) => {
                let len = self.active_image.len();
                let index_opt = self.active_image.to_vec().into_iter().position(|ai| ai);
                for (i, _) in self.active_image.clone().into_iter().enumerate() {
                    self.active_image[i] = false;
                }

                if wheel_event.delta_y() > 0.00 {
                    if let Some(index) = index_opt {
                        if index == 0 {
                            self.active_image[len - 1] = true
                        } else {
                            self.active_image[index - 1] = true
                        }
                    } else {
                        ConsoleService::error("no image active")
                    }
                } else if let Some(index) = index_opt {
                    if index == len - 1 {
                        self.active_image[0] = true
                    } else {
                        self.active_image[index + 1] = true
                    }
                } else {
                    ConsoleService::error("no image active")
                }
            }
            Msg::ShowScroll => {
                let body_style = document().body().unwrap().style();
                body_style.set_property("overflow", "hidden").unwrap();
            }
            Msg::HideScroll => {
                let body_style = document().body().unwrap().style();
                body_style.set_property("overflow", "scroll").unwrap();
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"Carousel Component"}</h1>
                <h2>{"Features required"}</h2>
                <span><code>{"carousel"}</code></span>

                <h2>{"Code Example"}</h2>
                <Prism
                    code=get_carousel()
                    language="rust"
                />

                <h2>{"Carousel properties"}</h2>
                <h3>{"Carousel Container"}</h3>
                <ul>
                    <li><b>{"onwheel_signal: "}</b>{"wheel event for carousel."}</li>
                    <li><b>{"onmouseover_signal: "}</b>{"mouse over event for carousel."}</li>
                    <li><b>{"onmouseleave_signal: "}</b>{"mouse leave event for carousel."}</li>
                    <li><b>{"key: "}</b>{"general property to add keys."}</li>
                    <li><b>{"code_ref: "}</b>{"general property to get the ref of the component."}</li>
                    <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                    <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>

                </ul>

                <h3>{"Carousel Controls"}</h3>
                <ul>
                    <li><b>{"prev_signal: "}</b>{"click event for the left control button to go to previous image. Required."}</li>
                    <li><b>{"next_signal: "}</b>{"click event for the left control button to go to the next image. Required."}</li>
                    <li><b>{"controls_palette: "}</b>{"type controls style. Options included in "}<code>{"Pallete"}</code>{". Default "}<code>{"Standard"}</code>{"."}</li>
                    <li><b>{"controls_size: "}</b>{"three diffent controls standard sizes. Options included in "}<code>{"Size"}</code>{". Default "}<code>{"Medium"}</code>{"."}</li>
                    <li><b>{"controls_style: "}</b>{"controls styles. Options included in "}<code>{"Style"}</code>{". Default "}<code>{"Regular"}</code>{"."}</li>
                    <li><b>{"key: "}</b>{"general property to add keys."}</li>
                    <li><b>{"code_ref: "}</b>{"general property to get the ref of the component."}</li>
                    <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                    <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>
                    <li><b>{"styles: "}</b>{"use stylist-rs to write styles in the component. Example: "}<code>{"css!(\"background-color: #918d94;\")"}</code></li>
                </ul>

                <h3>{"Carousel Dot"}</h3>
                <ul>
                    <li><b>{"onclick_signal: "}</b>{"Click event for carousel dot. Required."}</li>
                    <li><b>{"carousel_dot_palette: "}</b>{"type carousel dot style. Options included in "}<code>{"Pallete"}</code>{". Default "}<code>{"Standard"}</code>{"."}</li>
                    <li><b>{"key: "}</b>{"general property to add keys."}</li>
                    <li><b>{"code_ref: "}</b>{"general property to get the ref of the component."}</li>
                    <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                    <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>
                </ul>

                <p><b>{"This component has a optional children prop, in case that it is not included a default dot icon will be added"}</b></p>

                <h3>{"Carousel Image"}</h3>
                <ul>
                    <li><b>{"img_src: "}</b>{"Url image path. Required."}</li>
                    <li><b>{"active: "}</b>{"Show the image if it is active. "}<code>{"false"}</code>{" by default"}</li>
                    <li><b>{"carousel_dot_palette: "}</b>{"type carousel dot style. Options included in "}<code>{"Pallete"}</code>{". Default "}<code>{"Standard"}</code>{"."}</li>
                    <li><b>{"key: "}</b>{"general property to add keys."}</li>
                    <li><b>{"code_ref: "}</b>{"general property to get the ref of the component."}</li>
                    <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                    <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>
                </ul>

                <h2>{"Visual example"}</h2>
                <div>
                    <Carousel
                        class_name="fill-background"
                        onwheel_signal= self.link.callback(Msg::Scroll)
                        onmouseover_signal= self.link.callback(|_| Msg::ShowScroll)
                        onmouseleave_signal= self.link.callback(|_| Msg::HideScroll)>
                        {get_images(self.images.to_vec(), self.active_image.to_vec())}
                        {get_dots(self.active_image.to_vec(), self.link.clone())}
                        {get_controls(self.link.clone())}
                    </Carousel>
                </div>
            </div>
        }
    }
}

fn get_images(images: Vec<&str>, active_image: Vec<bool>) -> Html {
    images
        .into_iter()
        .enumerate()
        .map(|(index, image)| {
            html! {
                <CarouselImage active=active_image[index] img_src=image.to_owned()/>
            }
        })
        .collect::<Html>()
}

fn get_dots(active_image: Vec<bool>, link: ComponentLink<CarouselPage>) -> Html {
    let mut dot = vec![];

    for (i, _) in active_image.clone().into_iter().enumerate() {
        dot.push(html! {
            <CarouselDot active=active_image[i] onclick_signal = link.callback(move |_| Msg::ChangeImage(i))>
                <CommunicationAssets size=("12".to_string(), "12".to_string()) icon=CommunicationIcon::Smile/>
            </CarouselDot>
        })
    }

    dot.into_iter().collect::<Html>()
}

fn get_controls(link: ComponentLink<CarouselPage>) -> Html {
    html! {
        <CarouselControls
            controls_size=Size::Small
            prev_signal=link.callback(|_| Msg::Prev)
            next_signal=link.callback(|_| Msg::Next)/>
    }
}
