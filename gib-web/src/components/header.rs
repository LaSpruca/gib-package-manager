use yew::prelude::*;
use web_sys::Storage;
use crate::{console_log, log};

#[derive(Clone)]
pub struct Header {
    auth_token: String,
    link: ComponentLink<Self>
}

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        console_log!("Getting window");
        let window = web_sys::window().unwrap();
        console_log!("Got window");
        console_log!("Getting session");
        let mut session: Storage = window.session_storage().unwrap().unwrap();
        console_log!("Got session");
        let auth_key = match session.get_item("auth-key").unwrap() {
            Some(a) => {a}
            None => {"".to_string()}
        };

        Self {
            link,
            auth_token: auth_key
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <header>
                <div class="links">
                    <a href="/">{"Home"}</a>
                </div>

                <div class="acount">

                </div>
            </header>
        }
    }
}