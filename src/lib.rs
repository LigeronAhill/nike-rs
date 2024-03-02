mod components;
mod constants;
mod sections;
pub use constants::*;

use dioxus::prelude::*;

use crate::sections::*;
#[component]
pub fn App(cx: Scope) -> Element {
    render! {
        main { class: "relative", NavBar {} }
        section { class: "xl:padding-l wide:padding-r padding-b", Hero {} }
        section { class: "padding", PopularProducts {} }
        section { class: "padding", SuperQuality {} }
        section { class: "padding-x py-10", Services {} }
        section { class: "padding", SpecialOffer {} }
        section { class: "padding bg-pale-blue", CustomerReviews {} }
        section { class: "padding-x sm:py-32 py-16 w-full", Subscribe {} }
        section { class: "padding-x padding-t bg-black pb-8", Footer {} }
    }
}
