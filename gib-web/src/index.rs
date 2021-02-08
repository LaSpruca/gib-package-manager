use yew::prelude::*;

pub struct Index {
    link: ComponentLink<Self>,
}

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        use super::compoents::header::Header;
        html! {
            <div class="text">
                <Header />
                <div class="text-bg">
                    <h1>{"Welcome to Gib PM"}</h1>
                    <p>{"Still under development"}</p>
                    <a href="https://github.com/LaSpruca/gib-package-manager">{"GitHub"}</a>
                </div>
            </div>
        }
    }
}
