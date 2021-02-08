use yew::prelude::*;
use web_sys::Storage;

#[derive(Clone)]
pub struct Header {
    auth_token: String,
    link: ComponentLink<Self>
}

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let window = web_sys::window().unwrap();
        let mut session: Storage = window.session_storage().unwrap().unwrap();
        let auth_key = match session.get_item("auth-key") {
            Ok(a) => {a.unwrap()},
            Err(e) => {panic!(e.as_string())}
        };
        Self {
            link,
            auth_token: "nigga".to_string()
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