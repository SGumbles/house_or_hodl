use dioxus::{prelude::*};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx!{
        Stylesheet { href: asset!("/assets/tailwind.css") }
        div {
            class: "flex bg-slate-500 text-amber-50",
            div { 
                class: "size-32 flex flex-col justify-center items-center",
                p{
                    class: "font-bold text-4xl",
                    "House"
                }
                p{
                    class: "font-light",
                    "Or"
                }
                p{
                    class: "font-bold text-4xl",
                    "Hodl"
                }  
            }
            div { 
                class:"size-32 grow bg-slate-600 p-2 flex flex-col justify-around",
                p {  
                    "Is it better to own a house with a mortgage, or pour that money into an index fund and hope the market doesn't implode?"
                } 
                p { 
                    "What does each timeline look like under different circumstances?"
                }
                p { 
                    "Use this calculator to model out both scenarios"
                }
            }
        }

        div {  
            class: "flex text-amber-50",
            div {
                class: "bg-mauve-800 grow",
                "Yo"
            }
            div {
                class: "bg-mist-800 grow",
                "Hello"
            }
        }
    }
}
