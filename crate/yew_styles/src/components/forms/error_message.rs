use yew::prelude::*;

pub fn get_error_message(error_state: bool, error_message: String) -> Html {
    if error_state {
        html! {<span class="form-error">{error_message}</span>}
    } else {
        html! {}
    }
}
