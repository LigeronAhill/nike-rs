use dioxus::prelude::*;

use crate::{FooterLink, SocialMedia};

#[component]
pub fn Footer(cx: Scope) -> Element {
    let footer_links = FooterLink::iter();
    let social_media = SocialMedia::iter();
    render! {

        footer { class: "max-container",
            div { class: "flex justify-between items-start gap-20 flex-wrap max-lg:flex-col",
                div { class: "flex flex-col items-start",
                    a { href: "/", img {
                        class: "m-0",
                        width: "150",
                        src: "images/footer-logo.svg",
                        height: "46",
                        alt: "logo"
                    } }
                    p { class: "mt-6 text-base leading-7 font-montserrat text-white-400 sm:max-w-sm",
                        "Get shoes ready for the new term at your nearest Nike store. Find Your perfect Size In Store. Get Rewards"
                    }
                    div { class: "flex items-center gap-5 mt-8",
                        social_media.into_iter().map(|m| {
                        render! {
                            div { class: "flex justify-center items-center w-12 h-12 bg-white rounded-full",
                                img { width: "24", alt: "{m.alt}", src: "{m.src}", height: "24" }
                            }
                        }
                    })
                    }
                }
                div { class: "flex flex-1 justify-between lg:gap-10 gap-20 flex-wrap",
                    footer_links.into_iter().map(|f| {
                    render! {
                        div {
                            h4 { class: "font-montserrat text-2xl leading-normal font-medium mb-6 text-white",
                            "{f.title}"
                            }
                            ul {
                                f.links.into_iter().map(|l|{
                                    render! {
                                        li { class: "mt-3 font-montserrat text-base leading-normal text-white-400 hover:text-slate-gray",
                                            a { href: "{l.link}", "{l.name}" }
                                        }

                                    }
                                })
                            }
                        }
                    }
                })
                }
            }
            div { class: "flex justify-between text-white-400 mt-24 max-sm:flex-col max-sm:items-center",
                div { class: "flex flex-1 justify-start items-center gap-2 font-montserrat cursor-pointer",
                    img {
                        width: "20",
                        src: "icons/copyright-sign.svg",
                        alt: "copyright sign",
                        height: "20",
                        class: "rounded-full m-0"
                    }
                    p { "Copyright. All rights reserved." }
                }
                p { class: "font-montserrat cursor-pointer", "Terms & Conditions" }
            }
        }
    }
}
