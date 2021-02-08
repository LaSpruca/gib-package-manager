use crate::console_log;
use crate::util::log;
use crate::JsValue;
use web_sys::{MouseEvent, Window};
use yew::prelude::*;

#[derive(Clone)]
pub struct Header {
    auth_token: String,
    logged_in: bool,
    window: Window,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Logout,
}

impl Header {
    fn set_session(&self, key: &str, value: &str) -> Result<(), JsValue> {
        self.window
            .session_storage()
            .unwrap()
            .unwrap()
            .set_item(key, value)
    }

    fn get_session(&self, key: &str) -> Result<Option<String>, JsValue> {
        self.window
            .session_storage()
            .unwrap()
            .unwrap()
            .get_item(key)
    }
}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let window = web_sys::window().unwrap();
        let session = window.session_storage().unwrap().unwrap();
        let auth_token = match session.get_item("authToken").unwrap() {
            Some(a) => a,
            None => "".to_string(),
        };

        let logged_in = match session.get_item("loggedIn").unwrap() {
            Some(a) => a,
            None => "false".to_string(),
        };

        console_log!("Logged In: {}\nAuth Key: {}", logged_in, auth_token);

        Self {
            link,
            auth_token,
            logged_in: logged_in.parse().unwrap(),
            window,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Logout => {
                self.set_session("loggedIn", "false").unwrap();
                self.set_session("authToken", "").unwrap();

                self.auth_token = "".to_string();
                self.logged_in = false;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="header-wrap">
                <header>
                    <div class="links">
                        <a href="/">{"Home"}</a>
                    </div>

                    <div class="account">
                        {
                            if !self.logged_in {
                                html! {
                                    <>
                                        <a href="/account/login.html">{"Login"}</a>
                                    </>
                                }
                            } else {
                                html! {
                                    <>
                                        <a href="/account" > {"Account"} </a>
                                        <a href="javascript: void(0)" onclick=self.link.callback(|_: MouseEvent| Msg::Logout)>{"Logout"}</a>
                                    </>
                                }
                            }
                        }
                    </div>
                </header>
            </div>
        }
    }
}
