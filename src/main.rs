use iced::{Element, Length, Alignment};
use iced::widget::{column, row, text, text_input, container, space};
use iced::Task;

fn main() -> iced::Result {
    iced::application(
        HouseOrHodl::new,
        HouseOrHodl::update,
        HouseOrHodl::view,
    )
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
        let mortgage: f64 = self.mortgage_input.parse().unwrap_or(0.0);
        let annual_rate: f64 = self.interest_input.parse().unwrap_or(0.0);

        let (monthly_payment, total_interest, amortization) =
            if mortgage > 0.0 && annual_rate > 0.0 {
                calculate_mortgage(mortgage, annual_rate, 360)
            } else {
                (0.0, 0.0, vec![])
            };

        let input_section = row![
            column![
                text("Total Mortgage Amount ($):").size(14),
                text_input("e.g., 300000", &self.mortgage_input)
                    .on_input(Message::MortgageChanged)
                    .padding(8)
            ]
            .spacing(5)
            .width(Length::Fixed(250.0)),
            space(),
            column![
                text("Annual Interest Rate (%):").size(14),
                text_input("e.g., 6.5", &self.interest_input)
                    .on_input(Message::InterestRateChanged)
                    .padding(8)
            ]
            .spacing(5)
            .width(Length::Fixed(250.0)),
        ]
        .spacing(20)
        .padding(20)
        .align_y(Alignment::Start);

        let summary_section = if mortgage > 0.0 && annual_rate > 0.0 {
            row![
                column![
                    text("Monthly Payment").size(12),
                    text(format!("${:.2}", monthly_payment)).size(20),
                ]
                .spacing(5)
                .width(Length::FillPortion(1)),
                column![
                    text("Total Principal").size(12),
                    text(format!("${:.2}", mortgage)).size(20),
                ]
                .spacing(5)
                .width(Length::FillPortion(1)),
                column![
                    text("Total Interest (30 years)").size(12),
                    text(format!("${:.2}", total_interest)).size(20),
                ]
                .spacing(5)
                .width(Length::FillPortion(1)),
                column![
                    text("Total Paid").size(12),
                    text(format!("${:.2}", mortgage + total_interest)).size(20),
                ]
                .spacing(5)
                .width(Length::FillPortion(1)),
            ]
            .spacing(15)
            .padding(20)
            .align_y(Alignment::Center)
        } else {
            row![text("Enter values to see calculations")].padding(20)
        };

        let charts_section = if !amortization.is_empty() {
            column![
                text("Payment Breakdown Analysis").size(14),
                row![
                    build_principal_vs_interest_chart(&amortization),
                    build_balance_chart(&amortization),
                ]
                .spacing(10),
                row![
                    build_payment_composition_chart(monthly_payment, total_interest),
                ]
                .spacing(10),
            ]
            .spacing(10)
            .padding(10)
        } else {
            column![].padding(10)
        };

        let amortization_table = if !amortization.is_empty() {
            let table_header = text("Amortization Schedule (First 60 months of 360)").size(12);
            
            let table_content = column(
                amortization
                    .iter()
                    .take(60)
                    .map(|payment| {
                        text(format!(
                            "Mo {:>3} | Pmt ${:>10.2} | Prin ${:>10.2} | Int ${:>9.2} | Bal ${:>11.2}",
                            payment.month, payment.payment, payment.principal, payment.interest, payment.balance
                        ))
                        .size(10)
                        .into()
                    })
                    .collect::<Vec<_>>()
            )
            .spacing(1);

            column![
                table_header,
                table_content
            ]
            .spacing(5)
            .padding(10)
        } else {
            column![].padding(10)
        };

        container(
            column![
                text("HOUSE OR HODL - Mortgage Calculator").size(24),
                input_section,
                summary_section,
                charts_section,
                amortization_table,
            ]
            .padding(20)
            .spacing(20)
            .width(Length::Fill)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
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

/// Creates a visual chart showing principal vs interest breakdown over time (sample of 120 months)
fn build_principal_vs_interest_chart(amortization: &[PaymentRecord]) -> Element<'static, Message> {
    let sample_interval = (amortization.len() / 120).max(1);
    let samples: Vec<_> = amortization
        .iter()
        .step_by(sample_interval)
        .take(120)
        .collect();

    if samples.is_empty() {
        return container(text("No data")).into();
    }

    let max_principal = samples
        .iter()
        .map(|p| p.principal)
        .fold(0.0, f64::max);

    let mut chart_bars_principal = Vec::new();
    let mut chart_bars_interest = Vec::new();

    for payment in samples.iter() {
        let principal_height = (payment.principal / max_principal * 100.0).max(1.0);
        let interest_height = (payment.interest / max_principal * 100.0).max(1.0);

        chart_bars_principal.push(
            container(text(""))
                .width(Length::Fixed(3.0))
                .height(Length::Fixed(principal_height as f32))
                .padding(0)
        );
        
        chart_bars_interest.push(
            container(text(""))
                .width(Length::Fixed(3.0))
                .height(Length::Fixed(interest_height as f32))
                .padding(0)
        );
    }

    let principal_row = row(
        chart_bars_principal
            .into_iter()
            .map(|b| b.into())
            .collect::<Vec<Element<Message>>>()
    ).spacing(0);

    let interest_row = row(
        chart_bars_interest
            .into_iter()
            .map(|b| b.into())
            .collect::<Vec<Element<Message>>>()
    ).spacing(0);

    column![
        text("Principal (top) vs Interest (bottom) Breakdown - 120 months").size(11),
        text("Principal splits in early years, interest dominates at start").size(9),
        container(principal_row)
            .width(Length::Fixed(500.0))
            .height(Length::Fixed(60.0)),
        container(interest_row)
            .width(Length::Fixed(500.0))
            .height(Length::Fixed(60.0)),
    ]
    .spacing(5)
    .padding(10)
    .into()
}

/// Creates a visual chart showing remaining balance over time
fn build_balance_chart(amortization: &[PaymentRecord]) -> Element<'static, Message> {
    let sample_interval = (amortization.len() / 120).max(1);
    let samples: Vec<_> = amortization
        .iter()
        .step_by(sample_interval)
        .take(120)
        .collect();

    if samples.is_empty() {
        return container(text("No data")).into();
    }

    let max_balance = samples
        .iter()
        .map(|p| p.balance)
        .fold(0.0, f64::max);

    let chart_bars: Vec<Element<Message>> = samples
        .iter()
        .map(|payment| {
            let bar_height = (payment.balance / max_balance * 100.0).max(1.0);
            container(text(""))
                .width(Length::Fixed(4.0))
                .height(Length::Fixed(bar_height as f32))
                .padding(0)
                .into()
        })
        .collect();

    let chart_display = row(chart_bars).spacing(1);

    column![
        text("Remaining Loan Balance Over Time - 120 months").size(11),
        text("Watch the principal decrease as you pay down the mortgage").size(9),
        container(chart_display)
            .width(Length::Fixed(500.0))
            .height(Length::Fixed(120.0)),
    ]
    .spacing(5)
    .padding(10)
    .into()
}

/// Creates a pie chart style visualization of total principal vs total interest
fn build_payment_composition_chart(monthly_payment: f64, total_interest: f64) -> Element<'static, Message> {
    let total_principal = monthly_payment * 360.0 - total_interest;
    let total_amount = total_principal + total_interest;

    let principal_percent = total_principal / total_amount * 100.0;
    let interest_percent = total_interest / total_amount * 100.0;

    let principal_width = principal_percent * 2.5; // Scale to reasonable width
    let interest_width = interest_percent * 2.5;

    column![
        text("Total Payment Composition - 30 Year Mortgage").size(11),
        row![
            container(text(""))
                .width(Length::Fixed(principal_width as f32))
                .height(Length::Fixed(60.0))
                .padding(0),
            container(text(""))
                .width(Length::Fixed(interest_width as f32))
                .height(Length::Fixed(60.0))
                .padding(0),
        ].spacing(0),
        row![
            column![
                text("Principal").size(11),
                text(format!("${:.0}", total_principal)).size(11),
                text(format!("{:.1}%", principal_percent)).size(10),
            ]
            .spacing(3),
            space(),
            column![
                text("Interest").size(11),
                text(format!("${:.0}", total_interest)).size(11),
                text(format!("{:.1}%", interest_percent)).size(10),
            ]
            .spacing(3)
            .align_x(Alignment::End),
        ]
        .spacing(15)
        .align_y(Alignment::Center),
    ]
    .spacing(5)
    .padding(10)
    .into()
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