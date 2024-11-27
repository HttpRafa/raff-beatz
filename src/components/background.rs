use yew::{function_component, html, Html};

#[function_component(BackgroundElements)]
pub fn background_elements() -> Html {
    html! {
        <div class="background-elements">
            <div class="circle" style="top: 20%; left: 10%; width: 10px; height: 10px; animation-duration: 7s;"></div>
            <div class="circle" style="top: 50%; left: 70%; width: 20px; height: 20px; animation-duration: 5s;"></div>
            <div class="circle" style="top: 80%; left: 30%; width: 15px; height: 15px; animation-duration: 6s;"></div>
            <div class="circle" style="top: 70%; left: 80%; width: 15px; height: 15px; animation-duration: 5s;"></div>
            <div class="circle" style="top: 20%; left: 90%; width: 10px; height: 10px; animation-duration: 6s;"></div>
        </div>
    }
}
