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

pub fn navbar_code() -> String {
    "<Navbar
    fixed=Fixed::None
    navbar_style=Style::Light
    navbar_type=Palette::Info
    branch=html!{<img src=\"/assets/spielrs_logo.png\"></img>}>
        <NavbarContainer justify_content=JustifyContent::FlexStart(Mode::NoMode)>
            <NavbarItem
                onsignal=link.callback(move |_| Msg::ChangeMenu(String::from(\"Home\")))>
                <span>{\"Home\"}</span>
            </NavbarItem>
            <NavbarItem
                onsignal=link.callback(move |_| Msg::ChangeMenu(String::from(\"Shop\")))>
                <span>{\"Shop\"}</span>
            </NavbarItem>
            <NavbarItem
                onsignal=link.callback(move |_| Msg::ChangeMenu(String::from(\"Shop\")))>
                <span>{\"Shop\"}</span>
            </NavbarItem>
            <NavbarItem
                onsignal=link.callback(move |_| Msg::ChangeMenu(String::from(\"About us\")))>   
                <span>{\"About us\"}</span>
            </NavbarItem>
            <NavbarItem
                onsignal=link.callback(move |_| Msg::ChangeMenu(String::from(\"Contact\")))>   
                <span>{\"Contact\"}</span>
            </NavbarItem>
        </NavbarContainer>
</Navbar>"
        .to_string()
}
