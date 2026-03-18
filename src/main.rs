use dioxus::{prelude::*};

mod components;
use components::{FormatOptions, NumericInput};



fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx!{
        Stylesheet { href: asset!("/assets/tailwind.css") }
        div{
            class: "h-dvh bg-pink-600 flex flex-col",
            div {
                class: "flex bg-slate-800 text-amber-50 border-b-2 divide-x-2 flex-none",
                Header{}
            }
            div {  
                class: "flex text-amber-50 min-h-80 divide-x-2 flex-none",
                div {
                    class: "bg-mauve-800 basis-1/2 p-2",
                    House{}
                }
                div {
                    class: "bg-mist-800 basis-1/2 p-2",
                    Hodl{}
                }
            }
            div{
                class: "bg-slate-800 text-amber-50 border-t-2 grow p-2",
                Results{}
            }
        }
    }
}

#[component]
fn Results() -> Element {
    rsx!{
        div {
            h1 { class: "text-xl text-center","Results" }
        }
    }
}

#[component]
fn Hodl() -> Element {
    let per_month = use_signal(|| Some::<f64>(1000.0));
    let annual_interest = use_signal(|| Some::<f64>(7.1));
    let term_years = use_signal(|| Some::<f64>(25.0));

    rsx!{
        div{
            h1 {
                class: "text-xl text-center",
                "Hodl"
            }
            div {
                class: "mt-3 flex flex-col gap-4",
                NumericInput {
                    label: "Per Month Investment".to_string(),
                    value: per_month,
                    format: FormatOptions::float(0.0, 10000000.0),
                }
                NumericInput {
                    label: "Annual Interest (%)".to_string(),
                    value: annual_interest,
                    format: FormatOptions::float(0.0, 20.0),
                }
                NumericInput {
                    label: "Term (Years)".to_string(),
                    value: term_years,
                    format: FormatOptions::integer(1.0, 50.0),
                }
            }
        }
    }
}

#[component]
fn House() -> Element {
    let mortgage = use_signal(|| Some::<f64>(500000.0));
    let annual_interest = use_signal(|| Some::<f64>(5.25));
    let term_years = use_signal(|| Some::<f64>(25.0));

    use_effect({
        move || tracing::debug!("Re-calculating housing with {:?} {:?} {:?}",mortgage.read(),annual_interest,term_years)
    }
    );

    rsx!{
        div{
            h1 {
                class: "text-xl text-center",
                "House"
            }
            div {
                class: "mt-3 flex flex-col gap-4",
                NumericInput {
                    label: "Mortgage".to_string(),
                    value: mortgage,
                    format: FormatOptions::float(0.0, 10000000.0),
                }
                NumericInput {
                    label: "Annual Interest (%)".to_string(),
                    value: annual_interest,
                    format: FormatOptions::float(0.0, 20.0),
                }
                NumericInput {
                    label: "Term (Years)".to_string(),
                    value: term_years,
                    format: FormatOptions::integer(1.0, 50.0),
                }
            }
        }
    }
}

#[component]
fn Header() -> Element {
    rsx!{
        div { 
            class: "size-36 basis-1/4 flex flex-col justify-center items-center",
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
            class:"size-36 basis-3/4 bg-slate-800 p-4 flex flex-col justify-around",
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
}