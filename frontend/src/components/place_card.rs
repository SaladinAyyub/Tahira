use crate::models::place::PlaceCardProps;
use yew::prelude::*;

#[function_component]
fn Places(props: &PlaceCardProps) -> Html {
    html! {
    <article class="card">
                <header>
                    <h3>{ &props.name }</h3>
                    <p>{ &props.place_type.to_string() }</p>
                </header>
                <img src={props.image_url.clone()} alt={format!("Image of {}", &props.name)} />
                <section>
                    <p><strong>{"Halal Rating: "}</strong>{ &props.halal_label.to_string() }</p>
                    <p><strong>{"Address: "}</strong>{ &props.address }</p>
                    <p><strong>{"Contact: "}</strong>{ &props.mobile_number }</p>
                    <p><strong>{"Description: "}</strong>{ &props.place_description }</p>
                    <p><strong>{"Label Details: "}</strong>{ &props.label_description }</p>
                    {
                        if props.recommended {
                            html! { <p class="contrast">{"Recommended ✅"}</p> }
                        } else {
                            html! {}
                        }
                    }
                    <p>
                        <a href={props.map_url.clone()} target="_blank">{"View on Map 🗺️"}</a>
                    </p>
                </section>
            </article>
        }
}
