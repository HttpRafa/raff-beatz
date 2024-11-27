use crate::components::{background::BackgroundElements, navigation::NavigationMenu};
use yew::{function_component, html, Html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <main>
            <NavigationMenu current=1/>
            <LogoContainer/>
            <BackgroundElements/>
        </main>
    }
}

#[function_component(LogoContainer)]
fn logo_container() -> Html {
    html! {
        <div class="logo-container">
            <img src="images/logo.png" alt="RaffBeatz Production Logo"/>
        </div>
    }
}
