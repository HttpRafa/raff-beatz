use crate::components::{background::BackgroundElements, navigation::NavigationMenu, title::Title};
use yew::{function_component, html, Html};

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <main>
            <NavigationMenu current=3/>
            <Title title="About"/>
            <BackgroundElements/>
        </main>
    }
}
