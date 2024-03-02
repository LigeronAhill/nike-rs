use dioxus::prelude::*;

use crate::{components::ReviewCard, Review};

#[component]
pub fn CustomerReviews(cx: Scope) -> Element {
    let reviews = Review::iter();
    render! {
        div {

            section { class: "max-container",
                h3 { class: "font-palanquin text-center text-4xl font-bold",
                    "What Our "
                    span { class: "text-coral-red", "Customers" }
                    " Say?"
                }
                p { class: "m-auto mt-4 max-w-lg text-center info-text",
                    "Hear genuine stories from our satisfied customers about their exceptional experiences with us."
                }
                div { class: "mt-24 flex flex-1 justify-evenly items-center max-lg:flex-col gap-14",
                    reviews.into_iter().map(|r| render! {
                        ReviewCard {review: r}
                    })
                }
            }
        }
    }
}
