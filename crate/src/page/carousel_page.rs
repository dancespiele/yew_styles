use yew::prelude::*;
use yew::services::ConsoleService;
use yew_styles::carousel::{Carousel, CarouselControls, CarouselDot, CarouselImage};

pub struct CarouselPage {
    link: ComponentLink<Self>,
    images: Vec<&'static str>,
    active_image: Vec<bool>,
}

pub enum Msg {
    ChangeImage(usize),
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
                <h2>{"Visual example"}</h2>
                <div>
                    <Carousel>
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
                <CarouselImage active=active_image[index] img_src=image/>
            }
        })
        .collect::<Html>()
}

fn get_dots(active_image: Vec<bool>, link: ComponentLink<CarouselPage>) -> Html {
    let mut dot = vec![];

    for (i, _) in active_image.clone().into_iter().enumerate() {
        dot.push(html! {
            <CarouselDot active=active_image[i] onclick_signal = link.callback(move |_| Msg::ChangeImage(i))/>
        })
    }

    dot.into_iter().collect::<Html>()
}

fn get_controls(link: ComponentLink<CarouselPage>) -> Html {
    html! {
        <CarouselControls
            prev_signal=link.callback(|_| Msg::Prev)
            next_signal=link.callback(|_| Msg::Next)/>
    }
}
