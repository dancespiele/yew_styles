use stdweb::js;
use yew::Callback;

pub fn create_style(style: String, value: String, wrap: String) {
    js! {
        const element = document.getElementsByClassName(@{wrap})[0];
        element.style[@{style}] = @{value};
    }
}

#[derive(Clone)]
pub struct DefaultCallback<T> {
    callback: T,
}

impl Default for DefaultCallback<Callback<()>> {
    fn default() -> Self {
        DefaultCallback {
            callback: Callback::noop(),
        }
    }
}
