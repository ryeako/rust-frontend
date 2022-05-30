use yew::{Component, Context, html, Html, Properties, classes};
use yew_router::prelude::*;

use crate::pages::home::Home;
use crate::pages::about::About;
use crate::martial_arts::move_generator::MoveGenerator;
use crate::dice_roller::rolling_bones::RollingBones;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/martialArts")]
    MartialArts,
    #[at("/rollingBones")]
    RollingBones,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::MartialArts => html! { <MoveGenerator /> },
        Route::RollingBones => html! { <RollingBones /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct Navigation;

impl Component for Navigation {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Navigation
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let nav_classes = String::from("fixed top-0 left-0 h-screen w-16 m-0 flex flex-col justify-between bg-gray-300 dark:bg-gray-900 dark:text-white shadow-lg z-10");
        let nav_icon = String::from("relative flex items-center justify-center 
        h-12 w-12 mt-2 mb-2 mx-auto shadow-lg
        bg-gray-200 dark:bg-gray-800 text-green-500 hover:bg-gray-400
        dark:hover:bg-green-600 dark:hover:text-white
        rounded-3xl hover:rounded-xl
        transition-all duration-300 ease-linear
        cursor-pointer");
        html! {
            <nav class={classes!(nav_classes)}>
                <div class={classes!("justify-start")}>
                    <Link<Route> to={Route::Home}>
                        <div class={classes!(nav_icon.clone())}>
                            {"Home"} //figure out icon
                        </div>
                    </Link<Route>>
                    <Link<Route> to={Route::RollingBones}>
                        <div class={classes!(nav_icon.clone())}>
                            {"Dem Bones"} //figure out icon
                        </div>
                    </Link<Route>>
                    <Link<Route> to={Route::MartialArts}>
                        <div class={classes!(nav_icon.clone())}>
                            {"Sick Moves"} //figure out icon
                        </div>
                    </Link<Route>>
                    <Link<Route> to={Route::About}>
                        <div class={classes!(nav_icon.clone())}>
                            { "About" }
                        </div>
                    </Link<Route>>
                </div>
            </nav>
        }
    }
}