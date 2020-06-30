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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
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
                    <p>{"Each component is split in two parts, the logical yew component and its sass module, however,
                    it is not necessary to worry about the sass module only it needs to be include in the project"}</p>

                    <h3>{"How install it"}</h3>
                    <ol>
                        <li>{"Install the sass module: "}<code>{"npm install yew-styles"}</code></li>
                        <li>{"Add the yew_style crate with the features needed for your project in Cargo.toml file: "}<br/>
                            <code>{"yew_styles = {version=\"0.6.2\", features=[\"layouts\",\"button\"]}"}</code>
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
                <Item layouts=vec!(ItemLayout::ItXl(8), ItemLayout::ItM(10), ItemLayout::ItXs(12))>
                    <h2>{"How contribute"}</h2>
                    <p>{"First, open an issue describing about the fix, improvement or implementation and as suggestion, don't start to work in it until that is discussed.
                    If the contribution is a fix or small improvement in a component, only a pull request to master explaining what resolve or improve that, is required.
                    If it is an implementation, please follow the next requirements:"}</p>
                    <ol>
                        <li>{"Firstable open and issue describing about the component"}</li>
                        <li>{"Unit tests, which checks that the component is created and
                        its logic works, in the same file where it is implemented (test events is not needed for now)"}</li>
                        <li>{"One component per file, if multiple components have connections between them, it is possible create subfolder"}</li>
                        <li>{"Documentation in the component showing an example of using it and small description of each prop"}</li>
                        <li>{"Create a component page in "}<code>{"/crate/src/page"}</code>{" with the same structure than the rest of the components"}</li>
                    </ol>
                </Item>
            </Container>
        }
    }
}
