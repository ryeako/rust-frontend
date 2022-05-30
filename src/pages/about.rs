use yew::{Component, Context, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct About;

impl Component for About {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        About
    }
    
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section>
                {"Need to learn all the things"}
            </section>
        }
    }
}