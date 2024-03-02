use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CustomButtonProps<T: ToString + Clone> {
    pub label: T,
    pub icon_url: Option<T>,
    pub full_width: Option<bool>,
    pub bg_color: Option<T>,
}

#[component]
pub fn CustomButton<T: ToString + Clone>(cx: Scope<CustomButtonProps<T>>) -> Element {
    let bg = match cx.props.bg_color.clone() {
        None => "bg-coral-red text-white border-coral-red".to_string(),
        Some(color) => format!("bg-{} text-slate-gray border-slate-gray", color.to_string()),
    };
    let class = format!("flex justify-center items-center gap-2 px-7 py-4 border font-montserrat text-lg leading-none rounded-full {bg}");
    render! {
        button { class: "{class}",
            cx.props.label.to_string(),
            if cx.props.icon_url.is_some() {
                render! {
                    img {
                        src: "{cx.props.icon_url.clone().unwrap().to_string()}",
                        alt: "{cx.props.label.to_string()}",
                        class: "ml-2 rounded-full w-5 h-5"
                    }
                }
            }
        }
    }
}
