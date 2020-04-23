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

pub fn container_code() -> String {
    "<Container direction=Direction::Row wrap=Wrap::Wrap class_name=\"align-item\">
    <Item name=\"align\" index=0 layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexStart>
        <h3>{\"start\"}</h3>
    </Item>
    <Item name=\"align\" index=1 layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::Center>
        <h3>{\"center\"}</h3>
    </Item>
    <Item name=\"align\" index=2 layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexEnd>
        <h3>{\"end\"}</h3>
    </Item>
</Container>"
        .to_string()
}
