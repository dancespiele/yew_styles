use lipsum::lipsum;
use yew::prelude::*;
use yew_prism::Prism;
use yew_styles::button::Button;
use yew_styles::modal::Modal;
use yew_styles::styles::{Palette, Size, Style};

pub struct ModalPage {
    link: ComponentLink<Self>,
    show_modal: Vec<bool>,
}

pub enum Msg {
    CloseModal(usize),
    OpenModal(usize),
}

impl Component for ModalPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            show_modal: vec![false; 4],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CloseModal(index) => {
                self.show_modal[index] = false;
            }
            Msg::OpenModal(index) => {
                self.show_modal[index] = true;
            }
        };
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{"Modal Component"}</h1>

                <h2>{"Features required"}</h2>
                <span><code>{"modal"}</code></span>

                <h2>{"Visual examples"}</h2>
                <h3>{"Standard modal"}</h3>
                <Modal
                    header=html!{
                        <h3>{"Standard modal"}</h3>
                    }
                    header_type=Palette::Link
                    body=html!{
                        <div class="body-content">
                            <p>{lipsum(7)}</p>
                            <Button
                                button_type= Palette::Info
                                onclick_signal= self.link.callback(|_| Msg::CloseModal(0))
                            >{"Accept"}</Button>
                        </div>
                    }
                    body_style=Style::Outline
                    body_type=Palette::Link
                    is_open=self.show_modal[0]
                    modal_size=Size::Small
                    onclick_signal= self.link.callback(|_| Msg::CloseModal(0))
                />
                <Button
                    onclick_signal= self.link.callback(|_| Msg::OpenModal(0))
                >{"Standard Modal"}</Button>
            </>
        }
    }
}
