use crate::components::{background::BackgroundElements, navigation::NavigationMenu, title::Title};
use yew::{function_component, html, Html};

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <main>
            <NavigationMenu current=4/>
            <Title title="Contact"/>
            <BackgroundElements/>
        </main>
    }
}
