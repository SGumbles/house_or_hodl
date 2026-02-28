use iced::{Alignment, Element, Font, Length, Pixels};
use iced::widget::{column, row, text, text_input, container, scrollable};
use iced::Task;

const SANS_FONT_BYTES: &[u8] = include_bytes!("../fonts/IBMPlexSans-Regular.ttf");
const MONO_FONT_BYTES: &[u8] = include_bytes!("../fonts/MapleMono-Regular.ttf");

const SANS_FONT: Font = Font::with_name("IBM Plex Sans");
const MONO_FONT: Font = Font::with_name("Maple Mono");

fn main() -> iced::Result {
    iced::application(
        HouseOrHodl::new,
        HouseOrHodl::update,
        HouseOrHodl::view,
    )
    .font(SANS_FONT_BYTES)
    .font(MONO_FONT_BYTES)
    .default_font(SANS_FONT)
    .run()
}

#[derive(Debug, Clone)]
enum Message {
    MortgageChanged(String),
    InterestRateChanged(String),
}

#[derive(Clone)]
struct HouseOrHodl {
    mortgage_input: String,
    interest_input: String,
}

impl HouseOrHodl {
    fn new() -> (Self, Task<Message>) {
        (
            HouseOrHodl {
                mortgage_input: "300000".to_string(),
                interest_input: "6.5".to_string(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MortgageChanged(value) => {
                self.mortgage_input = value;
            }
            Message::InterestRateChanged(value) => {
                self.interest_input = value;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        container(
            scrollable(
                column![
                    Self::header(),
                    Self::input_section(&self.mortgage_input, &self.interest_input),
                    Self::summary_section(self),
                    Self::table_section(self),
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

    fn header() -> Element<'static, Message> {
        container(
            column![
                text("HOUSE OR HODL").size(32),
                text("Mortgage Calculator & Amortization Visualizer").size(14),
            ]
            .spacing(5)
        )
        .padding(20)
        .width(Length::Fill)
        .into()
    }

    fn input_section(mortgage_input: &str, interest_input: &str) -> Element<'static, Message> {
        container(
            column![
                text("Enter Mortgage Details").size(16),
                row![
                    column![
                        text("Mortgage Amount").size(12),
                        text_input("e.g., 300000", mortgage_input)
                            .on_input(Message::MortgageChanged)
                            .padding(10)
                    ]
                    .spacing(6)
                    .width(Length::Fill),
                    column![
                        text("Annual Interest Rate (%)").size(12),
                        text_input("e.g., 6.5", interest_input)
                            .on_input(Message::InterestRateChanged)
                            .padding(10)
                    ]
                    .spacing(6)
                    .width(Length::Fill),
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

    fn summary_section(&self) -> Element<'_, Message> {
        let mortgage: f64 = self.mortgage_input.parse().unwrap_or(0.0);
        let annual_rate: f64 = self.interest_input.parse().unwrap_or(0.0);

        let (monthly_payment, total_interest, _amortization) =
            if mortgage > 0.0 && annual_rate > 0.0 {
                calculate_mortgage(mortgage, annual_rate, 360)
            } else {
                (0.0, 0.0, vec![])
            };

        if mortgage > 0.0 && annual_rate > 0.0 {
            let total_paid = mortgage + total_interest;
            
            container(
                column![
                    text("Payment Summary").size(16),
                    row![
                        summary_card("Monthly Payment", format!("${:.2}", monthly_payment)),
                        summary_card("Total Principal", format!("${:.2}", mortgage)),
                        summary_card("Total Interest", format!("${:.2}", total_interest)),
                        summary_card("Total Amount Paid", format!("${:.2}", total_paid)),
                    ]
                    .spacing(15)
                ]
                .spacing(12)
            )
            .padding(20)
            .width(Length::Fill)
            .into()
        } else {
            container(
                text("Enter values to see calculations").size(14)
            )
            .padding(20)
            .width(Length::Fill)
            .into()
        }
    }

    fn table_section(&self) -> Element<'_, Message> {
        let mortgage: f64 = self.mortgage_input.parse().unwrap_or(0.0);
        let annual_rate: f64 = self.interest_input.parse().unwrap_or(0.0);

        let (_monthly_payment, _total_interest, amortization) =
            if mortgage > 0.0 && annual_rate > 0.0 {
                calculate_mortgage(mortgage, annual_rate, 360)
            } else {
                (0.0, 0.0, vec![])
            };

        if !amortization.is_empty() {
            // Pre-format the entire table as a single string to avoid thousands of widgets.
            // Header + 360 rows × ~60 chars ≈ 22 KB — one text widget instead of ~3,600.
            let mut table = String::with_capacity(amortization.len() * 64 + 80);
            table.push_str(&format!(
                "{:>5}  {:>11}  {:>11}  {:>10}  {:>12}\n",
                "Month", "Payment", "Principal", "Interest", "Balance"
            ));
            for p in &amortization {
                table.push_str(&format!(
                    "{:>5}  ${:>10.2}  ${:>10.2}  ${:>9.2}  ${:>11.2}\n",
                    p.month, p.payment, p.principal, p.interest, p.balance
                ));
            }

            container(
                column![
                    text("Amortization Schedule").size(16),
                    text(format!(
                        "All {} months of 30-year mortgage",
                        amortization.len()
                    ))
                    .size(12),
                    text(table).size(10).font(MONO_FONT)
                ]
            )
            .padding(20)
            .width(Length::Fill)
            .into()
        } else {
            container(column![]).padding(20).width(Length::Fill).into()
        }
    }
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