use dioxus::prelude::*;

use crate::{
    components::{CustomButton, ShoeCard},
    Shoes, Statistics,
};

#[component]
pub fn Hero(cx: Scope) -> Element {
    let statistics = Statistics::iter();
    let shoes = Shoes::iter();
    let current_bp = use_state(cx, || "images/big-shoe1.png".to_string());
    render! {
        section {
            id: "home",
            class: "w-full flex xl:flex-row flex-col justify-center min-h-screen gap-10 max-container",
            div { class: "relative xl:w-2/5 flex flex-col justify-center items-start w-full max-xl:padding-x pt-28",
                p { class: "font-montserrat text-xl text-coral-red", "Our Summer Collection" }
                h1 { class: "mt-10 font-palanquin text-8xl max-sm:text-[72px] max-sm:leading-[82] font-bold",
                    span { class: "xl:bg-white xl:whitespace-nowrap relative z-10 pr-10",
                        "The New Arrival"
                    }
                    br {}
                    span { class: "text-coral-red inline-block mt-3", "Nike" }
                    "Shoes"
                }
                p { class: "font-montserrat text-slate-gray text-lg leading-8 mt-6 mb-14 sm:max-w-sm",
                    "Discover stylish Nike arrivals, quality comfort, and innovation for your active life."
                }
                CustomButton { label: "Shop Now", icon_url: "icons/arrow-right.svg" }
                div { class: "flex justify-start items-start flex-wrap w-full mt-20 gap-16",
                    for stat in statistics {
                        render! {
                            div {
                                p { class: "text-4xl font-palanquin font-bold", stat.value }
                                p { class: "leading-7 font-montserrat text-slate-gray", stat.label }
                            }
                        }
                    }
                }
            }
            div { class: "relative flex-1 flex justify-center items-center xl:min-h-screen max-xl:py-40 bg-primary bg-hero bg-cover bg-center",
                img {
                    src: "{current_bp}",
                    alt: "shoe collection",
                    class: "object-contain relative z-10",
                    width: 610,
                    height: 500.
                }
                div { class: "flex sm:gap-6 gap-4 absolute -bottom-[5%] sm:left-[10%] max-sm:px-6",
                    for shoe in shoes {
                        render! {
                            div {
                                onclick: move |_| current_bp.set(shoe.big_shoe.clone()),
                                ShoeCard { shoe: shoe.clone(), current_bp: current_bp.get().to_string() }
                            }
                        }
                    }
                }
            }
        }
    }
}
