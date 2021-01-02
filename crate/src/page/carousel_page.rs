use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_styles::carousel::{Carousel, CarouselDot, CarouselImage};

pub struct CarouselPage {
    link: ComponentLink<Self>,
    images: Vec<&'static str>,
}

pub enum Msg {
    ChangeImage(usize),
}

impl Component for CarouselPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            images: vec!["/slide_1.jpg", "/slide_2.jpg", "/slide_3.jpg"],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeImage(image_index) => {}
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
                        {get_images(self.images.to_vec())}
                        {get_dots(self.images.len(), self.link.clone())}
                    </Carousel>
                </div>
            </div>
        }
    }
}

fn get_images(images: Vec<&str>) -> Html {
    images
        .into_iter()
        .map(|image| {
            html! {
                <CarouselImage img_src=image/>
            }
        })
        .collect::<Html>()
}

fn get_dots(index: usize, link: ComponentLink<CarouselPage>) -> Html {
    let mut dot = vec![];

    for i in 0..index {
        dot.push(html! {
            <CarouselDot onclick_signal = link.callback(move |_| Msg::ChangeImage(i))/>
        })
    }

    dot.into_iter().collect::<Html>()
}
