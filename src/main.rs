use yew::prelude::*;
use yew::{classes};
use yew_router::prelude::*;

mod pages;
mod shared;
mod martial_arts;
mod services;
mod dice_roller;

use shared::nav::{Navigation, Route, switch};

#[function_component(App)]
fn app() -> Html {
    let container_classes = String::from("fixed top-0 left-16 h-screen pl-5 pt-5 z-0 w-full bg-gray-100 dark:bg-gray-700 dark:text-white");
    html! {
        <BrowserRouter>
            <Navigation />
            <article class={classes!(container_classes)}> 
                <Switch<Route> render={Switch::render(switch)} />
            </article>
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}