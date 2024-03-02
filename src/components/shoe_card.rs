use dioxus::prelude::*;

use crate::Shoes;

#[component]
pub fn ShoeCard(cx: Scope, shoe: Shoes, current_bp: String) -> Element {
    let border = if *current_bp == shoe.big_shoe {
        "coral-red"
    } else {
        "transparent"
    };
    let class = format!("border-2 rounded-xl border-{border} cursor-pointer max-sm:flex-1");
    render! {
        div { class: "{class}",
            div { class: "flex justify-center items-center bg-card bg-center bg-cover sm:w-40 sm:h-40 rounded-xl max-sm:p-4",
                img {
                    src: "{shoe.thumbnail}",
                    alt: "shoe clillection",
                    width: 127,
                    height: 103,
                    class: "object-contain"
                }
            }
        }
    }
}
