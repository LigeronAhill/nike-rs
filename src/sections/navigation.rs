use dioxus::prelude::*;

use crate::NavLinks;
#[component]
pub fn NavBar(cx: Scope) -> Element {
    let nlinks = NavLinks::iter();
    render! {
        header { class: "padding-x py-8 absolute z-10 w-full",
            nav { class: "flex justify-between items-center max-container",
                a { href: "", img { src: "images/header-logo.svg", width: "130", alt: "logo", height: "29" } }
                ul { class: "flex-1 flex justify-center items-center gap-16 max-lg:hidden",
                    for navlink in nlinks {
                        render! {
                            li {
                                a {
                                    href: "{navlink.href}",
                                    class: "font-montserrat leading-normal text-lg text-slate-gray",
                                    navlink.label
                                }
                            }
                        }
                    }
                }
                div { class: "hidden max-lg:block", img { alt: "Menu", width: "25", src: "icons/hamburger.svg", height: "25" } }
            }
        }
    }
}
