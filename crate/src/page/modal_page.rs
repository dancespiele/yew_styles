use super::highlighters::get_modal_code;
use inflector::Inflector;
use lipsum::lipsum;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew::utils::document;
use yew_prism::Prism;
use yew_styles::button::Button;
use yew_styles::modal::Modal;
use yew_styles::styles::{get_size, Palette, Size, Style};

pub struct ModalPage {
    link: ComponentLink<Self>,
    show_modal: Vec<bool>,
}

pub enum Msg {
    CloseModal(usize),
    OpenModal(usize),
    CloseModalByKb(KeyboardEvent, usize),
}

impl Component for ModalPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            show_modal: vec![false; 5],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let body_style = document()
            .body()
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .style();

        match msg {
            Msg::CloseModal(index) => {
                body_style.set_property("overflow", "auto").unwrap();
                self.show_modal[index] = false;
            }
            Msg::CloseModalByKb(keyboard_event, index) => {
                if keyboard_event.key_code() == 27 {
                    body_style.set_property("overflow", "auto").unwrap();
                    self.show_modal[index] = false;
                }
            }
            Msg::OpenModal(index) => {
                body_style.set_property("overflow", "hidden").unwrap();

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

                <h2>{"Code example"}</h2>
                <Prism
                    code=get_modal_code()
                    language="rust"
                />

                <h2>{"Propeties"}</h2>
                <ul>
                    <li><b>{"modal_size: "}</b>{"three diffent modal standard sizes. Options included in "}<code>{"Size"}</code>{". Default "}<code>{"Medium"}</code>{"."}</li>
                    <li><b>{"modal_type: "}</b>{"type modal background style. Options included in "}<code>{"Pallete"}</code>{". Default "}<code>{"Standard"}</code>{"."}</li>
                    <li><b>{"header: "}</b>{"header of the modal. Required"}</li>
                    <li><b>{"header_type: "}</b>{"type modal header style. Options included in "}<code>{"Pallete"}</code>{". Default "}<code>{"Standard"}</code>{"."}</li>
                    <li><b>{"header_style: "}</b>{"modal header styles. Options included in "}<code>{"Style"}</code>{". Default "}<code>{"Regular"}</code>{"."}</li>
                    <li><b>{"header_interaction: "}</b>{"if hove, focus, active effects are enable in the header. Default "}<code>{"false"}</code>{"."}</li>
                    <li><b>{"body: "}</b>{"body of the modal. Required"}</li>
                    <li><b>{"body_type: "}</b>{"type modal body style. Options included in "}<code>{"Pallete"}</code>{". Default "}<code>{"Standard"}</code>{"."}</li>
                    <li><b>{"body_style: "}</b>{"modal body styles. Options included in "}<code>{"Style"}</code>{". Default "}<code>{"Regular"}</code>{"."}</li>
                    <li><b>{"body_interaction: "}</b>{"if hove, focus, active effects are enable in the body. Default "}<code>{"false"}</code>{"."}</li>
                    <li><b>{"is_open: "}</b>{"if it is true, shows the modal otherwise is hidden. Required"}</li>
                    <li><b>{"onclick_signal: "}</b>{"click event for modal (usually to close the modal)."}</li>
                    <li><b>{"onkeydown_signal: "}</b>{"keyboard event for modal (usually to close the modal)."}</li>
                    <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                    <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>
                </ul>

                <h2>{"Visual examples"}</h2>
                <h3>{"Standard modal"}</h3>
                <Modal
                    header=html!{
                        <b>{"Standard modal"}</b>
                    }
                    header_type=Palette::Secondary
                    modal_type=Palette::Secondary
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
                    body_type=Palette::Secondary
                    is_open=self.show_modal[0]
                    onclick_signal= self.link.callback(|_| Msg::CloseModal(0))
                    onkeydown_signal= self.link.callback(|e| Msg::CloseModalByKb(e, 0))
                />
                <Button
                    button_type= Palette::Secondary
                    onclick_signal= self.link.callback(|_| Msg::OpenModal(0))
                >{"Standard modal"}</Button>

                <h3>{"Interactive modal"}</h3>
                <Modal
                    header=html!{
                        <b>{"Interactive modal"}</b>
                    }
                    header_interaction= true
                    modal_type=Palette::Info
                    header_type=Palette::Link
                    body=html!{
                        <div class="body-content">
                            <p>{lipsum(7)}</p>
                            <Button
                                button_type= Palette::Info
                                onclick_signal= self.link.callback(|_| Msg::CloseModal(1))
                            >{"Accept"}</Button>
                        </div>
                    }
                    body_interaction= true
                    body_style=Style::Outline
                    body_type=Palette::Link
                    is_open=self.show_modal[1]
                    onclick_signal= self.link.callback(|_| Msg::CloseModal(1))
                    onkeydown_signal= self.link.callback(|e| Msg::CloseModalByKb(e, 1))
                />
                <Button
                    button_type= Palette::Link
                    onclick_signal= self.link.callback(|_| Msg::OpenModal(1))
                >{"Interactive modal"}</Button>
                {get_modal_sizes(self.show_modal.clone(), self.link.clone())}
            </>
        }
    }
}

fn get_modal_sizes(show_modal: Vec<bool>, link: ComponentLink<ModalPage>) -> Html {
    let sizes: Vec<Size> = vec![Size::Small, Size::Medium, Size::Big];

    sizes
        .into_iter()
        .enumerate()
        .map(|(index, size)| {
            html! {
                <>
                    <h3>{format!{"{} modal", get_size(size.clone()).to_pascal_case()}}</h3>
                    <Modal
                        header=html!{
                            <b>{format!{"{} modal", get_size(size.clone()).to_pascal_case()}}</b>
                        }
                        header_type=Palette::Link
                        body=html!{
                            <div class="body-content">
                                <p>{lipsum(7)}</p>
                                <Button
                                    button_type= Palette::Info
                                    onclick_signal= link.callback(move |_| Msg::CloseModal(index + 2))
                                >{"Accept"}</Button>
                            </div>
                        }
                        modal_size=size.clone()
                        body_style=Style::Outline
                        body_type=Palette::Link
                        is_open=show_modal[index + 2]
                        onclick_signal= link.callback(move |_| Msg::CloseModal(index + 2))
                        onkeydown_signal= link.callback(move |e| Msg::CloseModalByKb(e, index + 2))
                    />
                    <Button
                        button_type= Palette::Standard
                        onclick_signal= link.callback(move |_| Msg::OpenModal(index + 2))
                    >{{format!{"{} modal", get_size(size).to_pascal_case()}}}</Button>
                </>
            }
        })
        .collect::<Html>()
}
