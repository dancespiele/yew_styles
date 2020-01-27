use yew::prelude::*;
use components::{Button, ButtonType, Size, get_button_type, get_size};

pub struct ButtonPage {
    link: ComponentLink<Self>,
    button_type: String,
}

pub enum Msg {
    ChangeType(String),
}

#[derive(Clone, Properties)]
pub struct Props {
}

impl Component for ButtonPage {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ButtonPage {
            link,
            button_type: String::from("standard"),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeType(type_button) => {
                self.button_type = type_button;
            }
        }
        true
    }

    fn view(&self)-> Html {

        html! {
            <div>
                {get_buttons(self.link.clone())}
                <div>
                    {self.button_type.clone()}
                </div>
            </div>
        }
    }
}

fn get_buttons(link: ComponentLink<ButtonPage>) -> Html {
    let sizes: Vec<Size> = vec!(Size::Small, Size::Medium, Size::Big);

    sizes.into_iter().map(move |size| {
        let button_types: Vec<&str> = vec!("Standard", "Primary", "Secondary", "Info", "Success", "Warning", "Danger");
        let button_types_enum: Vec<ButtonType> = vec!(ButtonType::Standard, ButtonType::Primary, ButtonType::Secondary,
            ButtonType::Info, ButtonType::Success, ButtonType::Warning, ButtonType::Danger);
        
        let mut index = 0;

        html! {
            <div class="show-size">
                <h3>{get_size(size.clone()).to_uppercase()}</h3>
                {
                    button_types.into_iter().map(|button_type| {
                        let button = html! {
                            <Button
                                onsignal=link.callback(move |_| Msg::ChangeType(button_type.to_string().clone()))
                                class_name="button-page"
                                button_type=button_types_enum[index].clone()
                                size=size.clone()
                            >{get_button_type(button_types_enum[index].clone()).to_uppercase()}
                            </Button>
                        };
            
                        index = index + 1;
            
                        button
                    }).collect::<Html>()
                }
            </div>
        }
    }).collect::<Html>()
}
