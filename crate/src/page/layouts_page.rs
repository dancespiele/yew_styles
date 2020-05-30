use super::highlighters::container_code;
use yew::prelude::*;
use yew_prism::Prism;
use yew_styles::layouts::{
    container::{AlignContent, AlignItems, Container, Direction, JustifyContent, Mode, Wrap},
    item::{AlignSelf, Item, ItemLayout},
};

pub struct LayoutsPage;

pub struct LayoutsPageModel;

impl Component for LayoutsPage {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        LayoutsPage {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h1>{"Layouts Components"}</h1>
                </Item>

                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h2>{"Code example"}</h2>
                    <Prism
                        code=container_code()
                        language="rust"
                    />
                </Item>

                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h2>{"Container properties"}</h2>
                    <ul>
                        <li><b>{"direction: "}</b>{"which direction are placing the items. Options included in "}<code>{"Direction"}</code>{". Required."}</li>
                        <li><b>{"wrap: "}</b>{"set a wrap for the items. Options included in "}<code>{"Wrap"}</code>{". Default "}<code>{"Wrap"}</code>{"."}</li>
                        <li><b>{"justify_content: "}</b>{"set how will be justified the content. Options included in "}<code>{"JustifyContent"}</code>{". Default "}<code>{"FlexStart(No Mode)"}</code>{"."}</li>
                        <li><b>{"align_content: "}</b>{"set how will be aligned the content. Options included in "}<code>{"AlignContent"}</code>{". Default "}<code>{"Stretch(NoMode)"}</code>{"."}</li>
                        <li><b>{"align_items: "}</b>{"set how will be aligned the items. Options included in "}<code>{"AlignItems"}</code>{". Default "}<code>{"Stretch(NoMode)"}</code>{"."}</li>
                        <li><b>{"mode: "}</b>{"safe postion handler which is additional option for justify_content, align_content and align_items. Options included in "}<code>{"Mode"}</code>{". Default "}<code>{"NoMode"}</code>{"."}</li>
                        <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                        <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>
                    </ul>
                </Item>

                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h2>{"Item properties"}</h2>
                    <ul>
                        <li><b>{"layouts: "}</b>{"percent of the layout that will take the item. The value is a vector "}<code>{"Vec<ItemLayout>"}</code>{". Required"}</li>
                        <li><b>{"align_self: "}</b>{"align the item itself. Options include in "}<code>{"AlignSelf"}</code>{". Default "}<code>{"Auto"}</code></li>
                        <li><b>{"onclick_signal: "}</b>{"click event for the item. Default "}<code>{"noop()"}</code></li>
                        <li><b>{"id: "}</b>{"general property to add custom id"}</li>
                        <li><b>{"class_name: "}</b>{"general property to add custom class styles"}</li>
                    </ul>
                </Item>


                <h2>{"Visual examples"}</h2>
                <h3>{"Wrap"}</h3>
                <Container direction=Direction::Row wrap=Wrap::Wrap>
                    {(1..13).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                </Container>
                <h3>{"No wrap"}</h3>
                <Container direction=Direction::Row wrap=Wrap::Nowrap>
                    {(1..13).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                </Container>
                <h3>{"Wrap reverse"}</h3>
                <Container direction=Direction::Row wrap=Wrap::WrapReverse>
                    {(1..13).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                </Container>
                <h3>{"Row direction:"}</h3>
                <Container direction=Direction::Row wrap=Wrap::Wrap>
                    {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                </Container>
                <h3>{"Row reverse direction:"}</h3>
                <Container direction=Direction::RowReverse wrap=Wrap::Wrap>
                    {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                </Container>
                <h3>{"Column direction:"}</h3>
                <Container direction=Direction::Column wrap=Wrap::Wrap>
                    {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                </Container>
                <h3>{"Column reverse direction:"}</h3>
                <Container direction=Direction::ColumnReverse wrap=Wrap::Wrap>
                    {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                </Container>
                <h3>{"Combination of column and row direction"}</h3>
                <Container direction=Direction::Row wrap=Wrap::Wrap>
                    <Item
                        layouts=vec!(ItemLayout::ItXs(6))
                    >
                        <Container direction=Direction::Column wrap=Wrap::Wrap>
                            {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                        </Container>
                    </Item>
                    <Item
                        layouts=vec!(ItemLayout::ItXs(6))
                    >
                        <Container direction=Direction::Row wrap=Wrap::Wrap>
                            {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                        </Container>
                    </Item>
                </Container>
                <h3>{"Justify Content"}</h3>
                <Container
                    direction=Direction::Row wrap=Wrap::Wrap
                    justify_content=JustifyContent::Rigth(Mode::NoMode)
                >
                    <Item
                        layouts=vec!(ItemLayout::ItXs(4))
                    >
                        <Container direction=Direction::Column wrap=Wrap::Wrap>
                            {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                        </Container>
                    </Item>
                    <Item
                        layouts=vec!(ItemLayout::ItXs(4))
                    >
                        <Container direction=Direction::Column wrap=Wrap::Wrap>
                            {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                        </Container>
                    </Item>
                </Container>
                <p>{"To know about more options please visit "}
                    <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content" target="_bank">{"Justify Content"}</a>
                </p>
                <h3>{"Align Content"}</h3>
                <Container
                    direction=Direction::Row
                    wrap=Wrap::Wrap
                 align_content=AlignContent::FlexEnd(Mode::NoMode)
                    class_name="align"
                >
                    <Item
                        layouts=vec!(ItemLayout::ItXs(6))
                    >
                        <Container direction=Direction::Column wrap=Wrap::Wrap>
                            {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                        </Container>
                    </Item>
                    <Item
                        layouts=vec!(ItemLayout::ItXs(6))
                    >
                        <Container direction=Direction::Column wrap=Wrap::Wrap>
                            {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                        </Container>
                    </Item>
                </Container>
                <p>{"To know about more options please visit "}
                    <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/align-content" target="_bank">{"Align Content"}</a>
                </p>
                <h3>{"Align Items"}</h3>
                <Container direction=Direction::Row wrap=Wrap::Wrap>
                    <Item
                        layouts=vec!(ItemLayout::ItXs(6))
                    >
                        <Container
                            direction=Direction::Column
                            wrap=Wrap::Wrap
                         align_items=AlignItems::Center(Mode::NoMode)
                        >
                            {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                        </Container>
                    </Item>
                    <Item
                        layouts=vec!(ItemLayout::ItXs(6))
                    >
                        <Container
                            direction=Direction::Column
                            wrap=Wrap::Wrap
                            align_items=AlignItems::FlexEnd(Mode::NoMode)
                        >
                            {(1..5).map(|x| LayoutsPageModel.get_items(x)).collect::<Html>()}
                        </Container>
                    </Item>
                </Container>
                <p>{"To know about more options please visit "}
                    <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/align-items" target="_bank">{"Align Items"}</a>
                </p>
                <h3>{"Align self"}</h3>
                <Container direction=Direction::Row wrap=Wrap::Wrap class_name="align-item">
                    <Item layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexStart>
                        <h3>{"start"}</h3>
                    </Item>
                    <Item layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::Center>
                        <h3>{"center"}</h3>
                    </Item>
                    <Item layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexEnd>
                        <h3>{"end"}</h3>
                    </Item>
                </Container>
                <p>{"To know about more options please visit "}
                    <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/align-self" target="_bank">{"Align Self"}</a>
                </p>
            </div>
        }
    }
}

impl LayoutsPageModel {
    fn get_items(self, number: i8) -> Html {
        html! {
            <Item
                layouts=vec!(ItemLayout::ItXl(3), ItemLayout::ItL(3), ItemLayout::ItM(6), ItemLayout::ItXs(12))
            >
                <h3>{number}</h3>
            </Item>
        }
    }
}
