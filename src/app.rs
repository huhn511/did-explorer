use log::*;
use serde_derive::{Deserialize, Serialize};
use yew::prelude::*;


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
                self.hash = value;
            }
        }
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <div>
                <input
                    oninput=self.link.callback(|e: InputData| Msg::UpdateHash(e.value))
                    class="hash"
                    placeholder="paste your hash"
                    value=&self.hash />
            </div>
        }
    }
}