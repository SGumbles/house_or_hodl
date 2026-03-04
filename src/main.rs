use iced::{Alignment, Element, Font, Length, Pixels, Theme};
use iced::widget::{column, row, text, text_input, container, scrollable};
use iced::Task;

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
}

#[derive(Clone)]
struct HouseOrHodl {
    mortgage: String,
    interest: String,
    term_years: String
}

impl HouseOrHodl {
    fn new() -> (Self, Task<Message>) {
        (
            HouseOrHodl {
                mortgage: String::from("500000"),
                interest: String::from("5.2"),
                term_years: String::from("25"),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MortgageChanged(value) => {
                self.mortgage = value;
            }
            Message::InterestRateChanged(value) => {
                self.interest = value;
            }
            Message::TermYearsChanged(value) => {
                self.term_years = value;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        container(
            scrollable(
                column![
                    self.header(),
                    self.input_section()
                    // Self::input_section(&self.mortgage_input, &self.interest_input),
                    // Self::summary_section(self),
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

    fn header(&self) -> Element<'static, Message> {
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

    fn input_section(&self) -> Element<'static, Message> {
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

    // fn summary_section(&self) -> Element<'_, Message> {
    //     let mortgage: f64 = self.mortgage;
    //     let annual_rate: f64 = self.interest;

    //     let (monthly_payment, total_interest, _amortization) =
    //         if mortgage > 0.0 && annual_rate > 0.0 {
    //             calculate_mortgage(mortgage, annual_rate, 360)
    //         } else {
    //             (0.0, 0.0, vec![])
    //         };

    //     if mortgage > 0.0 && annual_rate > 0.0 {
    //         let total_paid = mortgage + total_interest;
            
    //         container(
    //             column![
    //                 text("Payment Summary").size(16),
    //                 row![
    //                     summary_card("Monthly Payment", format!("${:.2}", monthly_payment)),
    //                     summary_card("Total Principal", format!("${:.2}", mortgage)),
    //                     summary_card("Total Interest", format!("${:.2}", total_interest)),
    //                     summary_card("Total Amount Paid", format!("${:.2}", total_paid)),
    //                 ]
    //                 .spacing(15)
    //             ]
    //             .spacing(12)
    //         )
    //         .padding(20)
    //         .width(Length::Fill)
    //         .into()
    //     } else {
    //         container(
    //             text("Enter values to see calculations").size(14)
    //         )
    //         .padding(20)
    //         .width(Length::Fill)
    //         .into()
    //     }
    // }

    // fn table_section(&self) -> Element<'_, Message> {
   

}

#[derive(Clone, Debug)]
struct PaymentRecord {
    month: u32,
    payment: f64,
    principal: f64,
    interest: f64,
    balance: f64,
}

fn summary_card(label: &str, value: String) -> Element<'_, Message> {
    container(
        column![
            text(label).size(11),
            text(value).size(18),
        ]
        .spacing(8)
        .align_x(Alignment::Center)
    )
    .width(Length::Fill)
    .padding(15)
    .into()
}

fn calculate_mortgage(
    principal: f64,
    annual_rate: f64,
    months: u32,
) -> (f64, f64, Vec<PaymentRecord>) {
    let monthly_rate = annual_rate / 100.0 / 12.0;

    // Calculate monthly payment using standard formula
    // M = P * [r(1+r)^n] / [(1+r)^n - 1]
    let numerator = monthly_rate * (1.0 + monthly_rate).powi(months as i32);
    let denominator = (1.0 + monthly_rate).powi(months as i32) - 1.0;
    let monthly_payment = principal * (numerator / denominator);

    let mut balance = principal;
    let mut total_interest = 0.0;
    let mut schedule = Vec::new();

    for month in 1..=months {
        let interest_payment = balance * monthly_rate;
        let principal_payment = monthly_payment - interest_payment;
        balance -= principal_payment;

        total_interest += interest_payment;

        // Avoid negative balance due to floating point errors
        if balance < 0.0 {
            balance = 0.0;
        }

        schedule.push(PaymentRecord {
            month,
            payment: monthly_payment,
            principal: principal_payment,
            interest: interest_payment,
            balance,
        });
    }

    (monthly_payment, total_interest, schedule)
}


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