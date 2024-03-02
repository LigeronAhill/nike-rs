use dioxus::prelude::*;

use crate::Review;

#[component]
pub fn ReviewCard(cx: Scope, review: Review) -> Element {
    let rating = format!("{:2}", review.rating);
    render! {
        div { class: "flex justify-center items-center flex-col",
            img {
                src: "{review.img_url}",
                alt: "customer",
                class: "rounded-full object-cover w-[120px] h-[120px]"
            }
            p { class: "mt-6 max-w-sm text-center info-text", "{review.feedback}" }
            div { class: "mt-3 flex justify-center items-center gap-2.5",
                img {
                    alt: "rating star",
                    height: "24",
                    src: "icons/star.svg",
                    width: "24",
                    class: "object-contain m-0"
                }
                p { class: "text-xl font-montserrat text-slate-gray", "({rating})" }
            }
            h3 { class: "mt-1 font-palanquin text-3xl text-center font-bold", "{review.customer_name}" }
        }
    }
}
