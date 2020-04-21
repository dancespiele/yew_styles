pub fn button_code() -> String {
    "<Button
    onsignal=link.callback(move |_| Msg::Clicked(\"Hello world\"))
    class_name=\"hello-world\"
    button_type=Pallete::Standard
    button_style=Style::Light
    size=Size::Medium
>{\"Greeting\"}</Button>"
        .to_string()
}
