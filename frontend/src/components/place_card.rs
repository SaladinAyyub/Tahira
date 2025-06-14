use crate::models::place::PlaceCardProps;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub place: PlaceCardProps,
}
#[function_component(PlaceCard)]
pub fn place_card(props: &Props) -> Html {
    let place = &props.place;

    html! {
        <article class="card" style="padding: 1rem; display: flex; flex-direction: column; height: 100%;">
            <div style="text-align: center;">
                <img
                    src={place.image_url.clone()}
                    alt={format!("Image of {}", place.name)}
                    style="width: 100%; max-height: 200px; object-fit: cover; border-radius: 0.5rem;"
                />
                <h3 style="margin-top: 0.5rem; min-height: 4rem; overflow: hidden; text-overflow: ellipsis;">
                    { &place.name }
                </h3>
                <p style="margin: 0; font-style: italic; color: gray; min-height: 1.5rem;">{ place.place_type.to_string() }</p>
            </div>

            <section style="margin-top: 1rem; flex-grow: 1;">
                <p style="min-height: 2rem;"><strong>{"Halal Rating: "}</strong>{ place.halal_label.to_string() }</p>
                <p style="min-height: 6rem; overflow: hidden; text-overflow: ellipsis;">
                    <strong>{"Address: "}</strong>{ &place.address }
                </p>
                <p style="min-height: 2rem;"><strong>{"Contact: "}</strong>{ &place.mobile_number }</p>
                <p style="min-height: 6rem; overflow: hidden; text-overflow: ellipsis;">
                    <strong>{"Description: "}</strong>{ &place.place_description }
                </p>
                <p style="min-height: 2.5rem;"><strong>{"Label Details: "}</strong>{ &place.label_description }</p>

                {
                    if place.recommended {
                        html! { <p class="contrast" style="margin-top: 0.5rem;">{"Recommended ✅"}</p> }
                    } else {
                        html! {}
                    }
                }
            </section>

            <div style="margin-top: auto;">
                <a href={place.map_url.clone()} target="_blank">{"View on Map 🗺️"}</a>
            </div>
        </article>
    }
}
