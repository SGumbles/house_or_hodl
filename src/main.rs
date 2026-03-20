#![allow(dead_code, unused_variables)]

use dioxus::{prelude::*};

mod components;
use components::{FormatOptions, NumericInput};

mod simulator;
use simulator::{CompoundingInterestStyle};


fn main() {
    dioxus::launch(App);
}

#[derive(Clone, PartialEq, Debug)]
struct HouseResults {
    monthly_payments : f64,
    total_paid_to_principal : f64,
    total_paid_to_interest: f64
}

#[derive(Clone, PartialEq, Debug)]
struct HodlResults {
    total_paid_to_rent : f64,
    annual_interest : f64,
}

#[component]
fn App() -> Element {
    let term_years = use_signal(||25.0);

    let house_results = use_signal(||Option::<HouseResults>::None);
    let hodl_results = use_signal(||Option::<HodlResults>::None);

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
                    House{ term_years, house_results }
                }
                div {
                    class: "bg-mist-800 basis-1/2 p-2",
                    Hodl{ term_years, hodl_results }
                }
            }
            div{
                class: "bg-slate-800 text-amber-50 border-t-2 grow p-2",
                Results{house_results, hodl_results}
            }
        }
    }
}


#[component]
fn LabelData(label : String, data : String) -> Element {
    rsx!{
        div {  
            class : "w-fit",
            h3 {
                class: "gay",
                "{label}"
            }
            p { 
                "{data}"
            }
        }
    }
}


#[component]
fn Results(house_results : Signal<Option<HouseResults>>, hodl_results: Signal<Option<HodlResults>>) -> Element {
    rsx!{
        div {
            h1 { class: "text-2xl text-center","Results" }
        }
        div {
            class : "p-3",
            h1 { 
                class : "text-xl",
                "House Ownership" 
            }
            div { 
                class : "grid gap-3 grid-cols-[repeat(auto-fit,minmax(10rem,1fr))]",
                LabelData { label:"Average Monthly Payments", data:"2750.00"  }
                LabelData { label:"Total paid to Interest", data:"150.00"  }
                LabelData { label:"Total paid to Principal", data:"150.00"  }
            }
        }
        div {
            class : "p-3",
            h1 { 
                class : "text-xl",
                "Stock Market" 
            }
            div { 
                class : "grid gap-3 grid-cols-[repeat(auto-fit,minmax(10rem,1fr))]",
                LabelData { label:"Average Monthly Payments", data:"1000.0"  }
                LabelData { label:"Total paid to Rent", data:"150.00"  }
                LabelData { label:"Total paid to Principal", data:"150.00"  }
            }
        }
    }
}

#[component]
fn Hodl(term_years: Signal<f64>, hodl_results : Signal<Option<HodlResults>>) -> Element {
    
    let per_month = use_signal(||1000.0);
    let annual_interest = use_signal(|| 7.1);

    use_effect(move || {
        tracing::debug!("Recalculating Hodl based on {per_month} {annual_interest} {term_years}");
        *hodl_results.write() = Some( HodlResults { 
            total_paid_to_rent: 50025.2, 
            annual_interest: 5.2 
        })
    });

    rsx!{
        div{
            h1 {
                class: "text-2xl text-center",
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
                    format: FormatOptions::float(0.0, 40.0),
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
fn House(term_years: Signal<f64>, house_results : Signal<Option<HouseResults>>) -> Element {

    let mortgage = use_signal(|| 500000.0);
    let annual_interest = use_signal(|| 5.25);
    let mut interest_method = use_signal(|| CompoundingInterestStyle::Canadian);

    use_effect(move || {
        tracing::debug!("Recalculating House based on {mortgage} {annual_interest} {interest_method:?} {term_years}");
        *house_results.write() = Some(HouseResults{
            monthly_payments: 2600.52,
            total_paid_to_principal: 305122.50,
            total_paid_to_interest: 12533.5
        })
    });

    rsx!{
        div{
            h1 {
                class: "text-2xl text-center",
                "House"
            }
            div {
                class: "flex flex-row justify-around",
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
                div {
                    class: "mt-3 flex flex-col gap-4",
                    form {
                        onchange: move |evt| {
                            if let Some(value) = evt.get_first("interest") {
                                match value {
                                    FormValue::Text(selected) => {
                                        match selected.as_str() {
                                            "canadian" => *interest_method.write() = CompoundingInterestStyle::Canadian,
                                            "american" => *interest_method.write() = CompoundingInterestStyle::American,
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        },
                        fieldset {  
                            legend {  
                                "Interest Method"
                            }
                            div {
                                class: "space-x-4 flex flex-row",
                                div {  
                                    class: "space-x-2",
                                    label {  
                                        class:"text-sm",
                                        "Canadian"    
                                    }
                                    input{
                                        type:"radio",
                                        name:"interest",
                                        value:"canadian",
                                        checked:true
                                    }
                                }
                                div {
                                    class: "space-x-2",
                                    label {  
                                        class:"text-sm",
                                        "American"    
                                    }
                                    input{
                                        type:"radio",
                                        name:"interest",
                                        value:"american"
                                    }
                                }
                            }
                        }
                    }
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