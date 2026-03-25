#![allow(dead_code, unused_variables, unused_imports)]


use dioxus::{prelude::*};

mod components;
use components::{FormatOptions, NumericInput};

mod simulator;
use simulator::{CompoundingInterestStyle,run_house_simulator,HouseSimulatorParameters};

use crate::simulator::{StockSimulatorParameters, run_stock_simulator};


fn main() {
    dioxus::launch(App);
}

#[derive(Clone, PartialEq, Debug)]
struct HouseResults {
    monthly_payments : f64,
    params: HouseSimulatorParameters,
    total_paid_to_principal : f64,
    total_paid_to_interest: f64,
    term_length : f64,
    total_expenses: f64,
    total_renovation: f64,
}

#[derive(Clone, PartialEq, Debug)]
struct HodlResults {
    total_paid_to_rent : f64,
    total_portfolio : f64,
    net_portfolio : f64
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
                class: "bg-slate-800 text-amber-50 border-t-2 grow",
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

    let house_node = match house_results() {
        Some(r) => {
            rsx!{
                div { 
                    class : "grid gap-3 grid-cols-[repeat(auto-fit,minmax(10rem,1fr))]",
                    LabelData { label:"Average Monthly Payments", data:"{r.monthly_payments:.2}"  }
                    LabelData { label:"Total paid to Interest", data:"{r.total_paid_to_interest:0.2}"  }
                    LabelData { label:"Total Expenses", data:"{(r.total_expenses + r.total_renovation):0.2}"  }
                    LabelData { label:"Total spend", data:"{(r.total_paid_to_interest + r.total_paid_to_principal + r.total_expenses + r.total_renovation):0.2}"  }
                    LabelData { label:"Resulting Equity", data:"One House"  }
                }
            }
        }
        None => {
            rsx!{
                h3 {
                    "Computing"
                }
            }
        }
    };

    let hodl_node = match hodl_results() {
        Some(r) => {
            rsx!{
                div { 
                    class : "grid gap-3 grid-cols-[repeat(auto-fit,minmax(10rem,1fr))]",
                    LabelData { label:"Total paid to Rent", data:"{r.total_paid_to_rent:0.2}"  }
                    LabelData { label:"Total portfolio", data:"{r.total_portfolio:0.2}"  }
                    LabelData { label:"Net portfolio", data:"{r.net_portfolio:0.2}"  }
                }
            }
        }
        None => {
            rsx!{
                h3 {
                    "Computing"
                }
            }
        }
    };

    let percentage_per_month_on_mortgage = use_memo(move || 100.0 * house_results().map_or(0.0,|e|e.monthly_payments) / (75000.0 / 12.0) );

    let average_payment_per_month_future = use_memo(move||{
        match (house_results(), hodl_results()){
            (Some(h),Some(s)) => {
                if let Ok(params) = HouseSimulatorParameters::new( 
                    (h.total_paid_to_interest + h.total_paid_to_principal + h.total_expenses + h.total_renovation + s.net_portfolio) as f64,
                    h.params.annual_interest_rate,
                    h.params.term_length_years,
                    h.params.compounding_style,
                    h.params.monthly_expenses,
                    h.params.total_renovation){
                        let res = run_house_simulator(params.clone());
                        res.monthly_payment
                    }
                    else{
                        0.0
                    }
            }
            _ =>{
                0.0
            }
        }
    });

    let average_future_salary = use_memo(move || {
        match house_results(){
            Some(h) => {
                75000.0 * (1.03_f64).powf(h.term_length)
            }
            _ =>{
                0.0
            }
        }      
    });

    let percentage_per_month_in_future = use_memo( move ||{
        match house_results(){
            Some(h) => {
                average_payment_per_month_future() / (average_future_salary() / 12.0) * 100.00
            }
            _ =>{
                0.0
            }
        }
    });

    let summary_node = match (house_results(), hodl_results()) {
        (Some(h), Some(s)) =>{
            rsx!{
                div{
                    class : "space-y-4 mt-4",
                    p {
                        "If the average person makes $75,000/yr, they would spend {percentage_per_month_on_mortgage:0.2}% of their monthly salary (before tax) in order to pay the ${h.monthly_payments:0.2}. This doesn't include the monthly expense from the house."
                    }
                    p {
                        "In order to do better than the investment path, you would need to sell the house for ${h.total_paid_to_interest + h.total_paid_to_principal + h.total_expenses + h.total_renovation + s.net_portfolio:0.2}."
                    }
                    p {  
                        "If salaries continue to grow at an average of 3% per year, when you go to sell your house, you have to sell it to someone making ${average_future_salary:0.2} willing to spend ${average_payment_per_month_future:0.2} (the same terms as your mortgage) or {percentage_per_month_in_future:0.2}% of their salary per month to finance it. JUST to break even with the investment path."
                    }
                }
            }
        }
        _ => {
            rsx!{
                h3 {
                    "Computing"
                }
            }
        }
    };
    
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
            {house_node}
        }
        div {
            class : "p-3",
            h1 { 
                class : "text-xl",
                "Stock Market" 
            }
            {hodl_node}
        }
        div {
            class : "bg-slate-900 p-4",
            h1 { class: "text-2xl text-center","Summary" }
            {summary_node}
        }
    }
}

#[component]
fn Hodl(term_years: Signal<f64>, hodl_results : Signal<Option<HodlResults>>) -> Element {
    
    let per_month = use_signal(||1000.0);
    let annual_interest = use_signal(|| 7.1);
    let monthly_rent = use_signal(|| 1700.0);

    use_effect(move || {
        if let Ok(params) = StockSimulatorParameters::new(per_month(), monthly_rent(), term_years().trunc() as u32, annual_interest()){
            let res = run_stock_simulator(params);
            
            let total_paid_to_rent = res.timeline[res.timeline.len()-1].total_rent;
            let total_portfolio = res.timeline[res.timeline.len()-1].total_portfolio;
            let net_portfolio = total_portfolio - total_paid_to_rent;

            *hodl_results.write() = Some( HodlResults { 
                total_paid_to_rent,
                total_portfolio,
                net_portfolio
            })
        }
    });

    rsx!{
        div{
            h1 {
                class: "text-2xl text-center",
                "Hodl"
            }
            div {
                class: "flex flex-row justify-around",
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
                div {
                    class : "mt-3 flex flex-col gap-4",
                    NumericInput {
                        label: "Monthly Rent".to_string(),
                        value: monthly_rent,
                        format: FormatOptions::float(0.0, 10000000.0),
                    }
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
    let monthly_expenses = use_signal(|| 300.0);
    let total_renovation = use_signal(|| 75000.0);

    use_effect(move || {
        if let Ok(params) = HouseSimulatorParameters::new(mortgage(), annual_interest(), term_years().trunc() as u32, interest_method(), monthly_expenses(), total_renovation()){
            let res = run_house_simulator(params.clone());
            
            let total_paid_to_principal = res
            .timeline
            .iter()
            .fold(0.0,|acc, e| acc + e.amount_paid_to_principal);

            let total_paid_to_interest = res
            .timeline
            .iter()
            .fold(0.0,|acc, e| acc + e.amount_paid_to_interest);

            *house_results.write() = Some(HouseResults{
                monthly_payments: res.monthly_payment,
                params: params.clone(),
                total_paid_to_principal,
                total_paid_to_interest,
                term_length: term_years(),
                total_expenses: res.total_expenses,
                total_renovation: res.total_renovation,
            })
        }
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
                    NumericInput {
                        label: "Monthly Expenses (Insurance etc)".to_string(),
                        value: monthly_expenses,
                        format: FormatOptions::float(1.0, 3000.0),
                    }
                    NumericInput {
                        label: "Total Renovation Budget".to_string(),
                        value: total_renovation,
                        format: FormatOptions::float(1.0, 1000000.0),
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