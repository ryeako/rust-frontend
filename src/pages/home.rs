use yew::{Component, Context, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Home
    }
    
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section>
                <h1>{"Welcome Home"}</h1>
            </section>
        }
    }
}