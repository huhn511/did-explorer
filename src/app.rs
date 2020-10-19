use log::*;
use yew::prelude::*;

use anna_design_system::{Theme, Page, Container, atoms::H1};

pub struct App {
    link: ComponentLink<Self>,
    hash: String
}


pub enum Msg {
    UpdateHash(String),

}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
     
        App {
            link,
            hash: "".into()
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateHash(value) => {
                info!("value: {:?}", value);
                self.hash = value;
            }
        }
        false
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <Theme>
                <Page>
                    <Container>
                        <H1>{ "DID Explorer" }</H1>
                        <input
                            oninput=self.link.callback(|e: InputData| Msg::UpdateHash(e.value))
                            class="hash"
                            placeholder="paste your hash"
                            value=&self.hash />
                    </Container>
                </Page>
            </Theme>
        }
    }
}