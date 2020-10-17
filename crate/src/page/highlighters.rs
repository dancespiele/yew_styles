pub fn button_code() -> String {
    "<Button
    onclick_signal=link.callback(move |_| Msg::Clicked(\"Hello world\"))
    class_name=\"hello-world\"
    button_palette=Pallete::Standard
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
    navbar_palette=Palette::Info
    branch=html!{<img src=\"/assets/spielrs_logo.png\"></img>}>
        <NavbarContainer justify_content=JustifyContent::FlexStart(Mode::NoMode)>
            <NavbarItem
                onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from(\"Home\")))>
                <span>{\"Home\"}</span>
            </NavbarItem>
            <NavbarItem
                onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from(\"Shop\")))>
                <span>{\"Shop\"}</span>
            </NavbarItem>
            <NavbarItem
                onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from(\"Shop\")))>
                <span>{\"Shop\"}</span>
            </NavbarItem>
            <NavbarItem
                onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from(\"About us\")))>   
                <span>{\"About us\"}</span>
            </NavbarItem>
            <NavbarItem
                onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from(\"Contact\")))>   
                <span>{\"Contact\"}</span>
            </NavbarItem>
        </NavbarContainer>
</Navbar>"
        .to_string()
}

pub fn input_code() -> String {
    "<FormInput
    input_type=InputType::Text
    input_palette=Palette::Standard
    input_size=Size::Medium
    id=\"form-input-test\"
    oninput_signal = form_page.link.callback(|e: InputData| Msg::Input(e.value))
    placeholder=\"test\"
    underline=false
/>"
    .to_string()
}

pub fn file_code() -> String {
    "<FormFile
    accept=vec![\"image/png\".to_string(), \"image/jpg\".to_string()]
    underline=true
    onchange_signal = form_page.link.callback(|data: ChangeData | {
        if let ChangeData::Files(files) = data {
            let file = files.get(0).unwrap();
            Msg::UploadFile(file)
        } else {
            Msg::ErrorUploadImage
        }
    })
/>"
    .to_string()
}

pub fn select_code() -> String {
    "<FormSelect
    select_size=Size::Medium
    onchange_signal = form_page.link.callback(|e: ChangeData|
        match e {
            ChangeData::Select(element) => {
                let value = element.value();
                Msg::Select(value)
            },
            _ => unreachable!(),
        }
    )
    options=html!{
        <>
            <option value=\"\" disabled=true>{\"Select library\"}</option>
            <option value=\"yew\">{\"Yew\"}</option>
            <option value=\"yew_styles\">{\"Yew Styles\"}</option>
            <option value=\"yew_prism\">{\"Yew prism\"}</option>
        </>
    }
/>"
    .to_string()
}

pub fn textarea_code() -> String {
    "<FormTextArea placeholder=\"write here\"
    textarea_size=Size::Medium
    oninput_signal=form_page.link.callback(|e: InputData| Msg::Input(e.value))
/>"
    .to_string()
}

pub fn basic_form_code() -> String {
    "<Container wrap=Wrap::Wrap direction=Direction::Row>
    <Item layouts=vec!(ItemLayout::ItXs(12))>
        <h1>{\"Basic Form\"}</h1>
    </Item>
    <Item layouts=vec!(ItemLayout::ItXs(12))>
        <Form onsubmit_signal=self.link.callback(|e| Msg::Submit)>
            <Container wrap=Wrap::Wrap direction=Direction::Row>
                <Item layouts=vec!(ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                    <FormGroup orientation=Orientation::Horizontal>
                        <FormLabel text=\"First name: \"/>
                        <FormInput
                            error_state=self.empty_fields.iter().any(|field| field == \"first_name\")
                            error_message=\"First name field is required\"
                            input_type=InputType::Text
                            oninput_signal=self.link.callback(|e: InputData| Msg::FirstName(e.value))
                        />
                    </FormGroup>
                    <FormGroup orientation=Orientation::Horizontal>
                        <FormLabel text=\"Last name: \"/>
                        <FormInput
                            error_state=self.empty_fields.iter().any(|field| field == \"last_name\")
                            error_message=\"Last name field is required\"
                            input_type=InputType::Text
                            oninput_signal=self.link.callback(|e: InputData| Msg::LastName(e.value))
                        />
                    </FormGroup>
                    <FormGroup orientation=Orientation::Horizontal>
                        <FormLabel text=\"Email: \"/>
                        <FormInput
                            error_state=self.empty_fields.iter().any(|field| field == \"email\")
                            error_message=\"Email field is required\"
                            input_type=InputType::Email
                            oninput_signal=self.link.callback(|e: InputData| Msg::Email(e.value))
                        />
                    </FormGroup>
                </Item>
                <Item layouts=vec!(ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                    <FormGroup orientation=Orientation::Vertical>
                        <FormLabel text=\"Specialty:\"/>
                        <FormSelect
                            id=\"specialty\"
                            error_state=self.empty_fields.iter().any(|field| field == \"specialty\")
                            error_message=\"Select specialty is required\"
                            onchange_signal=self.link.callback(|e: ChangeData| {
                                match e {
                                    ChangeData::Select(element) => {
                                        let value = element.value();
                                        Msg::Specialty(value)
                                    },
                                    _ => unreachable!()
                                }
                            })
                            options=html!{
                                <>
                                <option value=\"\" disabled=true>{\"Choose specialty\"}</option>
                                    <option value=\"frontend\">{\"Frontend\"}</option>
                                    <option value=\"backend\">{\"Backend\"}</option>
                                </>
                            }
                        />
                    </FormGroup>
                    <FormGroup orientation=Orientation::Vertical>
                        <FormLabel text=\"Skills:\"/>
                        <FormSelect
                            id=\"skills\"
                            multiple=true
                            onchange_signal=self.link.callback(|e: ChangeData| {
                                match e {
                                    ChangeData::Select(element) => {
                                        let mut values = vec![];
                                        let options = element.options();

                                        for i in 0..options.length() {
                                            let option = options
                                                .get_with_index(i)
                                                .unwrap()
                                                .dyn_into::<HtmlOptionElement>()
                                                .unwrap();
                                            if option.selected() {
                                                values.push(option.value());
                                            }
                                        }
                                        Msg::Skills(values)
                                    },
                                    _ => unreachable!()
                                }
                            })
                            options=html!{
                                <>
                                    <option value=\"yew\">{\"Yew.rs\"}</option>
                                    <option value=\"rustwasm\">{\"Rustwasm\"}</option>
                                    <option value=\"rust\">{\"Rust\"}</option>
                                    <option value=\"warp\">{\"Warp\"}</option>
                                    <option value=\"tokio\">{\"Tokio\"}</option>
                                </>
                            }
                        />
                    </FormGroup>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <FormGroup orientation=Orientation::Vertical>
                        <FormLabel text=\"Cover letter:\"/>
                        <FormTextArea
                            error_state=self.empty_fields.iter().any(|field| field == \"cover_letter\")
                            error_message=\"cover letter is required\"
                            oninput_signal=self.link.callback(|e: InputData| Msg::CoverLetter(e.value))/>
                    </FormGroup>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItM(3))>
                    <FormGroup>
                    <FormSubmit
                        value=\"Submit application\"
                        submit_palette=Palette::Success
                        submit_style=Style::Outline
                    />
                    </FormGroup>
                </Item>
            </Container>
        </Form>
    </Item>
    <Item layouts=vec!(ItemLayout::ItXs(12))>
        {get_result(self.result.clone())}
    </Item>
</Container>".to_string()
}

pub fn get_card() -> String {
    "<Card
    card_size=Size::Medium
    card_palette=Palette::Info
    card_style=Style::Outline
    header=Some(html!{<div>
        {\"Header\"}
    </div>})
    body=Some(html!{
        <div>{\"Content\"}</div>
    })
    footer=Some(html!{
        <div>{\"Footer\"}</div>
    })
/>"
    .to_string()
}

pub fn get_modal_code() -> String {
    "<Modal
    header=html!{
        <b>{\"Standard modal\"}</b>
    }
    header_palette=Palette::Link
    body=html!{
        <div class=\"body-content\">
            <p>{\"this is a modal example\"}</p>
            <Button
                button_palette= Palette::Info
                onclick_signal= self.link.callback(|_| Msg::CloseModal)
            >{\"Accept\"}</Button>
        </div>
    }
    body_style=Style::Outline
    body_palette=Palette::Link
    is_open=self.show_modal
    onclick_signal= self.link.callback(|_| Msg::CloseModal)
    onkeydown_signal= self.link.callback(Msg::CloseModalByKb)
/>"
    .to_string()
}

pub fn get_title_text() -> String {
    "<Text
    text_type=TextType::Title(Header::H1)
    plain_text=\"Header h1\"
    html_text=None
/>"
    .to_string()
}

pub fn get_plain_text() -> String {
    "<Text
    text_type=TextType::Plain
    text_size=Size::Medium
    plain_text=\"plain text\"
    html_text=None
/>"
    .to_string()
}

pub fn get_html_text() -> String {
    "<Text
    text_type=TextType::Plain
    text_size=Size::Medium
    html_text=Some(html!{
        <><i>{\"Italic \"}</i>{\"letter\"}</>
    })
/>
    "
    .to_string()
}

pub fn get_paragraph_text() -> String {
    "<Text
    text_type=TextType::Paragraph
    text_size=Size::Small
    plain_text=\"paragraph text\"
    html_text=None
/>"
    .to_string()
}

pub fn get_alert_text() -> String {
    "<Text
    text_type=TextType::Alert
    text_size=Size::Medium
    plain_text=\"alert text\"
    html_text=None
    text_style=Style::Outline
    text_palette=Palette::Success
/>"
    .to_string()
}

pub fn get_tag_text() -> String {
    "<Text
    id=\"tag\"
    draggable=true
    ondragstart_signal=link.callback(Msg::Dragged)
    text_type=TextType::Tag
    text_size=Size::Medium
    plain_text=\"tag text\"
    html_text=None
    text_style=Style::Regular
    text_palette=Palette::Info
/>"
    .to_string()
}

pub fn navbar_with_a_tag() -> String {
    "<Navbar
    class_name=\"navbar-router\"
    navbar_palette=Palette::Info
    navbar_style=Style::Outline
    fixed=Fixed::Top
    branch=html!{<img src=\"./spielrs_logo.png\"/>}
>
    <NavbarContainer>
        <NavbarItem
            class_name=\"navbar-route\"
            active = self.navbar_items[0]
            onclick_signal = self.link.callback(|_| Msg::ChangeNavbarItem(0))
            >
            <RouterAnchor<AppRouter>route=AppRouter::RootPath>{\"Home\"}</RouterAnchor<AppRouter>></NavbarItem>
        <NavbarItem
            class_name=\"navbar-route\"
            active = self.navbar_items[1]
            onclick_signal = self.link.callback(|_| Msg::ChangeNavbarItem(1))
            >
            <RouterAnchor<AppRouter>route=AppRouter::AboutPath>{\"About\"}</RouterAnchor<AppRouter>></NavbarItem>
    </NavbarContainer>
</Navbar>".to_string()
}
