pub fn button_code() -> String {
    "<Button
    onsignal=link.callback(move |_| Msg::Clicked(\"Hello world\"))
    class_name=\"hello-world\"
    button_type=Pallete::Standard
    button_style=Style::Light
    size=Size::Medium
>{{to_first_upercase(&get_pallete(button_types_enum[index].clone()))}}
</Button>"
        .to_string()
}
