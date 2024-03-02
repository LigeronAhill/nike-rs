use dioxus::prelude::*;

use crate::components::CustomButton;
#[component]
pub fn SuperQuality(cx: Scope) -> Element {
    render! {
        div {
            section {
                class: "flex justify-between items-center max-lg:flex-col gap-10 w-full max-container",
                id: "about-us",
                div { class: "flex flex-1 flex-col",
                    h2 { class: "font-palanquin text-4xl capitalize font-bold lg:max-w-lg",
                        "We Provide You "
                        span { class: "text-coral-red", "Super Quality" }
                        " Shoes"
                    }
                    p { class: "mt-4 lg:max-w-lg info-text",
                        "Ensuring premium comfort and style, our meticulously crafted footwear is designed to elevate your experience, providing you with unmatched quality, innovation, and a touch of elegance."
                    }
                    p { class: "mt-6 lg:max-w-lg info-text",
                        "Our dedication to detail and excellence ensures your satisfaction"
                    }
                    div { class: "mt-11", CustomButton { label: "View Details" } }
                }
                div { class: "flex-1 flex justify-center items-center",
                    img {
                        src: "images/shoe8.svg",
                        height: "522",
                        width: "570",
                        alt: "shoe8",
                        class: "object-contain"
                    }
                }
            }
        }
    }
}
