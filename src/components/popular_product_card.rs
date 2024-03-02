use dioxus::prelude::*;

use crate::Product;

#[component]
pub fn PopularProductCard(cx: Scope, product: Product) -> Element {
    render! {
        div { class: "flex flex-1 flex-col w-full max-sm:w-full",
            img {
                alt: "{product.name.clone()}",
                src: "{product.img_url}",
                class: "w-[290px] h-[280px]"
            }
            div { class: "mt-8 flex justify-start gap-2.5",
                img { height: "24", alt: "rating", width: "25", src: "icons/star.svg" }
                p { class: "font-montserrat text-xl leading-normal text-slate-gray", "(4.5)" }
            }
            h3 { class: "mt-2 text-2xl leading-normal font-semibold font-palanquin", product.name.clone() }
            p { class: "mt-2 font-semibold font-montserrat text-coral-red text-2xl leading-normal",
                product.price.clone()
            }
        }
    }
}
