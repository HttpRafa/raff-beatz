use yew::prelude::*;
use yew_router::prelude::*;

use crate::sites::{self, switch};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<sites::Route> render={switch} />
        </BrowserRouter>
    }
}
