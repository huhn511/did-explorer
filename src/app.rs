use log::*;
use yew::prelude::*;

use anna_design_system::{Theme, Page, Container, atoms::{H1}, Button };

use identity_core::{did::{DID, DIDDocument, DIDDocumentBuilder}, diff::Diff};
use identity_crypto::{KeyGen, KeyGenerator};

pub struct App {
    link: ComponentLink<Self>,
    hash: String,
    did_doc: String
}


pub enum Msg {
    UpdateHash(String),
    Clicked,

}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {

        App {
            link,
            hash: "".into(),
            did_doc: "".into(),
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
                false
            },
            Msg::Clicked => {
                               
                let mut did_doc = DIDDocumentBuilder::default()
                    .context(vec![DID::BASE_CONTEXT.into()])
                    .id("did:iota:123456789abcdefghi".parse().unwrap())
                    .build()
                    .unwrap();

                info!("did_doc: {:?}", did_doc);
                self.did_doc = did_doc.to_string();
                info!("did_doc: {:?}", did_doc.to_string());
                false
            },
        }
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
                        <Button 
                            onclick_signal=self.link.callback(move |_| Msg::Clicked)
                            >{ "Search DID" }</Button>
                        <pre>{ &self.did_doc }</pre>
                    </Container>
                </Page>
            </Theme>
        }
    }
}