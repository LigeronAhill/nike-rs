use dioxus::prelude::*;

use crate::{components::ServiceCard, Service};

#[component]
pub fn Services(cx: Scope) -> Element {
    let services = Service::iter();
    render! {
        section { class: "max-container flex justify-center flex-wrap gap-9",
            for service in services {
                render! {
                    ServiceCard {service: service}
                }
            }
        }
    }
}
