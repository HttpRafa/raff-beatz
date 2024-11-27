use crate::components::{background::BackgroundElements, navigation::NavigationMenu, title::Title};
use yew::{function_component, html, Html};

#[function_component(Beats)]
pub fn beats() -> Html {
    html! {
        <main>
            <NavigationMenu current=2/>
            <Title title="Beats"/>
            <BackgroundElements/>
        </main>
    }
}
