use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <main>
            <h1>{"Tahira"}</h1>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
