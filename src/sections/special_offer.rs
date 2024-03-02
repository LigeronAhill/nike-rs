use dioxus::prelude::*;

use crate::components::CustomButton;

#[component]
pub fn SpecialOffer(cx: Scope) -> Element {
    render! {
        section {

            section { class: "flex justify-wrap items-center max-xl:flex-col-reverse gap-10 max-container",
                div { class: "flex-1",
                    img {
                        height: "687",
                        alt: "special offer",
                        width: "773",
                        src: "images/offer.svg",
                        class: "object-contain w-full"
                    }
                }
                div { class: "flex flex-1 flex-col",
                    h2 { class: "font-palanquin text-4xl capitalize font-bold lg:max-w-lg",
                        span { class: "text-coral-red", "Special" }
                        "Offer"
                    }
                    p { class: "mt-4 info-text",
                        "Embark on a shopping journey that redefines your experience with unbeatable deals. From premier selections to incredible savings, we offer unparalleled value that sets us apart."
                    }
                    p { class: "mt-6 info-text",
                        "Navigate a realm of possibilities designed to fulfill your unique desires, surpassing the loftiest expectations. Your journey with us is nothing short of exceptional."
                    }
                    div { class: "mt-11 flex flex-wrap gap-4",
                        CustomButton { label: "Shop now", icon_url: "icons/arrow-right.svg" }
                        CustomButton { label: "Learn More", bg_color: "white" }
                    }
                }
            }
        }
    }
}
