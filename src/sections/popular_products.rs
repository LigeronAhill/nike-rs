use dioxus::prelude::*;

use crate::{components::PopularProductCard, Product};

#[component]
pub fn PopularProducts(cx: Scope) -> Element {
    let products = Product::iter();
    render! {
        section { id: "products", class: "max-container max-sm:mt-12",
            div { class: "flex flex-col justify-start gap-5",
                h2 { class: "text-4xl font-palanquin font-bold",
                    "Our"
                    span { class: "text-coral-red", " Popular " }
                    "Products"
                }
                p { class: "lg:max-w-lg mt-2 font-montserrat text-slate-gray",
                    "Experience top-notch quality and style with our sought-after selections. Discover a world of comfort, design and value"
                }
            }
            div { class: "mt-16 grid lg:grid-cols-4 md:grid-cols-3 sm:grid-cols-2 grid-cols-1 sm:gap-4 gap-14",
                for product in products {
                    render! {
                        PopularProductCard {product: product}
                    }
                }
            }
        }
    }
}
