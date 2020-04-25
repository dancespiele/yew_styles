use yew::prelude::*;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};

pub struct HomePage;

impl Component for HomePage {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        HomePage {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Container wrap=Wrap::Wrap
                direction=Direction::Row>
                <Item layouts=vec!(ItemLayout::ItXl(8), ItemLayout::ItM(10), ItemLayout::ItXs(12))>
                    <h1>{"Yew Styles"}</h1>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXl(8), ItemLayout::ItM(10), ItemLayout::ItXs(12))>
                    <p>{"Yew Styles is a style framework for yew"}</p>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXl(8), ItemLayout::ItM(10), ItemLayout::ItXs(12))>
                    <h2>{"Motivation"}</h2>
                    <p>{"The purpose of developing this project is first, provide a style framework for yew
                    because there isn't not many options currently, also to create a layout system which is not
                    far of the flexbox concept, and, to take the rust benefits and implement a properties
                    selected by enumeration in the most of the cases which makes fast for developing applications
                    and avoids the practice try and error"}</p>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXl(8), ItemLayout::ItM(10), ItemLayout::ItXs(12))>
                    <h2>{"How it works"}</h2>
                    <p>{"Each component is splited in two parts, the logical yew component and its sass module, however,
                    it is not necessary to worry about the sass module only it needs to be include in the project"}</p>

                    <h3>{"How install it"}</h3>
                    <ol>
                        <li>{"Install the sass module: "}<code>{"npm install yew-styles"}</code></li>
                        <li>{"Add the yew_style crate in Cargo.toml file: "}
                            <code>{"yew_styles = \"0.3.0\""}</code>
                        </li>
                        <li>{"Import the main.css file in you main javascript/typescript file project: "}
                            <code>{"import 'node_modules/yew-styles/main.css';"}</code>
                        </li>
                        <li>{"Ready to import and use in your project \u{1F680}"}</li>
                    </ol>

                    <p>{"In the left side there is a list of links where each one access to a correspondent component documentation,
                    there, shows how to use it."}</p>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXl(8), ItemLayout::ItM(10), ItemLayout::ItXs(12))>
                    <h2>{"Development phase"}</h2>
                    <p>{"Yew style is in early phase, currently doesn't have enough components to cover all the requirements
                    that could need a website/web application. All contributions are appreciated."}</p>
                </Item>
            </Container>
        }
    }
}
