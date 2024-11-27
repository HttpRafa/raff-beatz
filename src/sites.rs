use about::About;
use beats::Beats;
use contact::Contact;
use home::Home;
use yew::{html, Html};
use yew_router::Routable;

pub mod about;
pub mod beats;
pub mod contact;
pub mod home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/beats")]
    Beats,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home/>
        },
        Route::Beats => html! {
            <Beats/>
        },
        Route::About => html! {
            <About/>
        },
        Route::Contact => html! {
            <Contact/>
        },
    }
}
