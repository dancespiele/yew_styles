use yew::prelude::*;
use stylist::css;

pub fn get_error_message(error_state: bool, error_message: String) -> Html {
    if error_state {
        html! {<span class=classes!(css!(r#"
            color: #ed1c24;
            font-size: 12px;
        "#),"form-error")>{error_message}</span>}
    } else {
        html! {}
    }
}
