use dioxus::prelude::*;

use crate::Service;

#[component]
pub fn ServiceCard(cx: Scope, service: Service) -> Element {
    render! {

        div { class: "flex-1 sm:w-[350px] sm:min-w-[350px] w-full rounded-[20px] shadow-3xl px-10 py-16",
            div { class: "w-11 h-11 justify-center items-center bg-coral-red rounded-full flex",
                img {
                    height: "24",
                    width: "24",
                    src: "{service.img_url}",
                    alt: "{service.label.clone()}"
                }
            }
            h3 { class: "mt-5 font-palanquin text-3xl leading-normal font-bold", "{service.label}" }
            p { class: "mt-3 break-words font-montserrat text-lg leading-normal text-slate-gray",
                "{service.subtext}"
            }
        }
    }
}
