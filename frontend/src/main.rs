use components::place_list::PlaceList;
use yew::prelude::*;
mod components;
mod models;

#[function_component]
fn App() -> Html {
    html! {
        <>
        <header class="container" style="text-align: center;">
                <h1>{"Tahira"}</h1>
                <img src="https://cdn.saladin.pro/tahira-logo-no-text.png" alt="Tahira Logo" style="max-width: 250px; margin-bottom: 1rem;" />
                <p class="lead">{"Discover Halal. Dine with Dignity. 🌙✨"}</p>
                <p>{"Discover local eateries that respect your values — no haram music 🎧🚫, no disrespect to dress codes 👳🧕, just wholesome meals 🍽️."}</p>
        </header>
        <main class="container">
                <PlaceList />
        </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
