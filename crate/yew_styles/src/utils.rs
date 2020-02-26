use stdweb::js;

pub fn create_style(style: String, value: String, wrap: String) {
    js! {
        const element = document.getElementsByClassName(@{wrap})[0];
        element.style[@{style}] = @{value};
    }
}
