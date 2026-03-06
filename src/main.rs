use std::marker::PhantomData;
use std::ops::Deref;
use std::str::FromStr;

use iced::alignment::Horizontal;
use iced::{Element, Font, Length, Pixels, Theme};
use iced::widget::{checkbox, column, container, row, scrollable, text, text_input};
use iced::Task;

mod simulator;

const MONO_FONT_BYTES: &[u8] = include_bytes!("../fonts/VictorMonoNerdFontMono-Regular.ttf");
const MONO_FONT: Font = Font::with_name("VictorMono Nerd Font Mono");

fn main() -> iced::Result {
    iced::application(
        HouseOrHodl::new,
        HouseOrHodl::update,
        HouseOrHodl::view,
    )
    .font(MONO_FONT_BYTES)
    .default_font(MONO_FONT)
    .theme(Theme::TokyoNight)
    .run()
}

#[derive(Debug, Clone)]
enum Message {
    MortgageChanged(String),
    InterestRateChanged(String),
    TermYearsChanged(String),
    PayPrincipalFirstChanged(bool)
}

#[derive(Clone,Debug)]
struct NumericString<NumericType>(String,PhantomData<NumericType>);

impl<NumericType: FromStr> NumericString<NumericType>{
    fn new<T: ToString> (value: T) -> Self {
        let string_val = value.to_string();
        if Self::parses_nicely(&string_val) {
            NumericString::<NumericType>(string_val,PhantomData)
        } else{
            NumericString::<NumericType>("".to_string(),PhantomData)
        }
    }
    fn update(&mut self, value: String) {
        if Self::parses_nicely(&value) {
            *self = Self::new(value);
        }
    }
    fn parses_nicely(s: &String) -> bool {
        if s.parse::<NumericType>().is_ok() || s.is_empty() {
            true
        } else {
            false
        }
    }
    fn to_numeric(&self) -> Result<NumericType,NumericType::Err> {
        self.parse()
    }
}

impl<NumericType> Deref for NumericString<NumericType>{
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

impl<T: ToString, N:FromStr> From<T> for NumericString<N>{
    fn from(value: T) -> Self {
        NumericString::<N>::new(value)
    }
}

#[derive(Clone)]
struct HouseOrHodl {
    mortgage: NumericString<f64>,
    interest: NumericString<f64>,
    term_years: NumericString<u32>,
    pay_principal_first: bool
}

impl HouseOrHodl {
    fn new() -> (Self, Task<Message>) {
        (
            HouseOrHodl {
                mortgage: "500000".into(),
                interest: "5.2".into(),
                term_years: "25".into(),
                pay_principal_first: false
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MortgageChanged(value) => {
                self.mortgage.update(value);
            }
            Message::InterestRateChanged(value) => {
                self.interest.update(value);
            }
            Message::TermYearsChanged(value) => {
                self.term_years.update(value);
            }
            Message::PayPrincipalFirstChanged(value) => {
                self.pay_principal_first = value;
            }
        }
        Task::none()
    }

    fn view(&'_ self) -> Element<'_,Message> {
        container(
            scrollable(
                column![
                    self.header(),
                    self.input_section(),
                    self.summary_section(),
                    // Self::table_section(self),
                ]
                .spacing(0)
                .width(Length::Fill)
            )
            .height(Length::Fill)
        )
        .width(Pixels(1200.0))
        .height(Pixels(1200.0))
        .into()
    }

    fn header(&self) -> Element<'_,Message> {
        container(
            column![
                text("House Or Hodl").size(48),
                text("Mortgage Calculator & Amortization Visualizer").size(24),
            ]
            .spacing(5)
        )
        .padding(20)
        .width(Length::Fill)
        .into()
    }

    fn input_section(&self) -> Element<'_,Message> {
        container(
            column![
                text("Mortgage Details").size(16),
                row![
                    column![
                        text("Mortgage Amount").size(12),
                        text_input("e.g., 300000", &self.mortgage)
                        .on_input(Message::MortgageChanged)
                        .padding(10)
                    ]
                    .spacing(6)
                    .width(Length::Fill),
                    column![
                        text("Annual Interest Rate (%)").size(12),
                        text_input("e.g., 6.5", &self.interest)
                        .on_input(Message::InterestRateChanged)
                        .padding(10)
                    ]
                    .spacing(6)
                    .width(Length::Fill),
                    column![
                        text("Term Length (Years)").size(12),
                        text_input("e.g., 25", &self.term_years)
                        .on_input(Message::TermYearsChanged)
                        .padding(10)
                    ]
                    .spacing(6)
                    .width(Length::Fill),
                    column![
                        text("Pay the principal first?").size(12),
                        checkbox(self.pay_principal_first)
                        .on_toggle(Message::PayPrincipalFirstChanged)
                    ]
                    .spacing(6)
                    .width(Length::Fill)
                ]
                .spacing(15)
                .width(Length::Fill),
            ]
            .spacing(12)
        )
        .padding(20)
        .width(Length::Fill)
        .into()
    }

    fn summary_section(&self) -> Element<'_,Message> {
        
        let blank_section = container(
            text("Enter values to see calculations").size(28)
        )
        .padding(20)
        .width(Length::Fill)
        .align_x(Horizontal::Center)
        .into();

        // Just to be clear, I would never write this, I just want to practice destructuring in a ridiculous way
        let (   Ok(mortgage),
                Ok(interest),
                Ok(term_years)) = 
                (self.mortgage.to_numeric(),
                self.interest.to_numeric(),
                self.term_years.to_numeric()) else {
            return blank_section;
        };

        let sim_results = simulator::run_simulator( simulator::SimulatorParameters{
            mortgage:mortgage,
            annual_interest_rate:interest,
            term_length_years:term_years,
            pay_down_principal_first: false
        });
        container(
            text(format!("Gonna be paying about {} :(",sim_results.monthly_payments)).size(28)
        )
        .padding(20)
        .width(Length::Fill)
        .align_x(Horizontal::Center)
        .into()
        
        // let annual_rate: f64 = self.interest;

        // let (monthly_payment, total_interest, _amortization) =
        //     if mortgage > 0.0 && annual_rate > 0.0 {
        //         calculate_mortgage(mortgage, annual_rate, 360)
        //     } else {
        //         (0.0, 0.0, vec![])
        //     };

        // if mortgage > 0.0 && annual_rate > 0.0 {
        //     let total_paid = mortgage + total_interest;
            
        //     container(
        //         column![
        //             text("Payment Summary").size(16),
        //             row![
        //                 summary_card("Monthly Payment", format!("${:.2}", monthly_payment)),
        //                 summary_card("Total Principal", format!("${:.2}", mortgage)),
        //                 summary_card("Total Interest", format!("${:.2}", total_interest)),
        //                 summary_card("Total Amount Paid", format!("${:.2}", total_paid)),
        //             ]
        //             .spacing(15)
        //         ]
        //         .spacing(12)
        //     )
        //     .padding(20)
        //     .width(Length::Fill)
        //     .into()
        // } else {
            
        // }
    }

    // fn table_section(&self) -> Element<'_, Message> {
   

}

// #[derive(Clone, Debug)]
// struct PaymentRecord {
//     month: u32,
//     payment: f64,
//     principal: f64,
//     interest: f64,
//     balance: f64,
// }

// fn summary_card(label: &str, value: String) -> Element<'_, Message> {
//     container(
//         column![
//             text(label).size(11),
//             text(value).size(18),
//         ]
//         .spacing(8)
//         .align_x(Alignment::Center)
//     )
//     .width(Length::Fill)
//     .padding(15)
//     .into()
// }

// fn calculate_mortgage(
//     principal: f64,
//     annual_rate: f64,
//     months: u32,
// ) -> (f64, f64, Vec<PaymentRecord>) {
//     let monthly_rate = annual_rate / 100.0 / 12.0;

//     // Calculate monthly payment using standard formula
//     // M = P * [r(1+r)^n] / [(1+r)^n - 1]
//     let numerator = monthly_rate * (1.0 + monthly_rate).powi(months as i32);
//     let denominator = (1.0 + monthly_rate).powi(months as i32) - 1.0;
//     let monthly_payment = principal * (numerator / denominator);

//     let mut balance = principal;
//     let mut total_interest = 0.0;
//     let mut schedule = Vec::new();

//     for month in 1..=months {
//         let interest_payment = balance * monthly_rate;
//         let principal_payment = monthly_payment - interest_payment;
//         balance -= principal_payment;

//         total_interest += interest_payment;

//         // Avoid negative balance due to floating point errors
//         if balance < 0.0 {
//             balance = 0.0;
//         }

//         schedule.push(PaymentRecord {
//             month,
//             payment: monthly_payment,
//             principal: principal_payment,
//             interest: interest_payment,
//             balance,
//         });
//     }

//     (monthly_payment, total_interest, schedule)
// }


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use iced_test::{Error, simulator};

//     #[test]
//     fn it_counts() -> Result<(), Error> {
//         let mut HouseOrHodl = HouseOrHodl { value: 0 };
//         let mut ui = simulator(HouseOrHodl.view());

//         let _ = ui.click("Increment")?;
//         let _ = ui.click("Increment")?;
//         let _ = ui.click("Decrement")?;

//         for message in ui.into_messages() {
//             HouseOrHodl.update(message);
//         }

//         assert_eq!(HouseOrHodl.value, 1);

//         let mut ui = simulator(HouseOrHodl.view());
//         assert!(ui.find("1").is_ok(), "HouseOrHodl should display 1!");

//         Ok(())
//     }
// }