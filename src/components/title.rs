use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct TitleProps {
    pub title: String,
}

#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    html! {
        <h1 class="mt-24 text-center text-gray-300 font-bold special-title">{props.title.clone()}</h1>
    }
}
