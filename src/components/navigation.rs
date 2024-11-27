use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct NavigationProps {
    pub current: i32,
}

#[function_component(NavigationMenu)]
pub fn navigation_menu(props: &NavigationProps) -> Html {
    html! {
        <nav class="bg-gray-800">
            <div class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                <div class="relative flex h-16 items-center justify-between">
                    <div class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
                        <div class="flex shrink-0 items-center">
                            <img class="h-8 w-auto" src="images/logo-no-bg.png" alt="RaffBeatz Production"/>
                        </div>
                        <div class="sm:ml-6 sm:block">
                            <div class="flex space-x-4">
                                {
                                    match props.current {
                                        1 => html! {
                                            <>
                                                <a href="/" class="rounded-md bg-gray-900 px-3 py-2 text-sm font-medium text-white" aria-current="page">{ "Home" }</a>
                                                <a href="/beats" class="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white">{ "Beats" }</a>
                                                <a href="/about" class="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white">{ "About" }</a>
                                                <a href="/contact" class="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white">{ "Contact" }</a>
                                            </>
                                        },
                                        2 => html! {
                                            <>
                                                <a href="/" class="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white">{ "Home" }</a>
                                                <a href="/beats" class="rounded-md bg-gray-900 px-3 py-2 text-sm font-medium text-white" aria-current="page">{ "Beats" }</a>
                                                <a href="/about" class="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white">{ "About" }</a>
                                                <a href="/contact" class="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white">{ "Contact" }</a>
                                            </>
                                        },
                                        3 => html! {
                                            <>
                                                <a href="/" class="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white">{ "Home" }</a>
                                                <a href="/beats" class="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white">{ "Beats" }</a>
                                                <a href="/about" class="rounded-md bg-gray-900 px-3 py-2 text-sm font-medium text-white" aria-current="page">{ "About" }</a>
                                                <a href="/contact" class="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white">{ "Contact" }</a>
                                            </>
                                        },
                                        4 => html! {
                                            <>
                                                <a href="/" class="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white">{ "Home" }</a>
                                                <a href="/beats" class="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white">{ "Beats" }</a>
                                                <a href="/about" class="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white">{ "About" }</a>
                                                <a href="/contact" class="rounded-md bg-gray-900 px-3 py-2 text-sm font-medium text-white" aria-current="page">{ "Contact" }</a>
                                            </>
                                        },
                                        _ => html! {}
                                    }
                                }
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}
