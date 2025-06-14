use crate::components::place_card::PlaceCard;
use crate::models::place::PlaceCardProps;
use gloo_net::http::Request;
use web_sys::console;
use yew::prelude::*;

#[function_component(PlaceList)]
pub fn place_list() -> Html {
    let places = use_state(Vec::new);

    {
        let places = places.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched: Result<Vec<PlaceCardProps>, _> = async {
                    let res = Request::get("http://localhost:3000/tahira/api/places")
                        .send()
                        .await?;
                    res.json::<Vec<PlaceCardProps>>().await
                }
                .await;

                if let Ok(data) = fetched {
                    places.set(data);
                } else {
                    console::log_1(&"Failed to fetch places".into());
                }
            });
            || ()
        });
    }

    html! {
        <section class="container">
            <h2>{"Some spots from our Database📍"}</h2>
            <div class="grid">
                { for places.iter().map(|place| html! {
                    <PlaceCard place={place.clone()} />
                })}
            </div>
        </section>
    }
}
