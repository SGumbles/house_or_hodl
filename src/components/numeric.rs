use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum NumberKind {
    Integer,
    Float,
}

#[derive(Clone, Copy, PartialEq)]
pub struct FormatOptions {
    pub min: f64,
    pub max: f64,
    pub kind: NumberKind,
}

impl FormatOptions {
    pub const fn integer(min: f64, max: f64) -> Self {
        assert!(min <= max, "min must be <= max");
        Self {
            min,
            max,
            kind: NumberKind::Integer,
        }
    }

    pub const fn float(min: f64, max: f64) -> Self {
        assert!(min <= max, "min must be <= max");
        Self {
            min,
            max,
            kind: NumberKind::Float,
        }
    }
}

fn is_allowed_numeric_input(input: &str, format: FormatOptions) -> bool {
    if input.is_empty() {
        return true;
    }

    if input == "-" {
        return format.min < 0.0;
    }

    let mut chars = input.chars();
    let is_negative = matches!(chars.next(), Some('-'));
    let remainder = if is_negative { &input[1..] } else { input };

    if is_negative && format.min >= 0.0 {
        return false;
    }

    if remainder.is_empty() {
        return false;
    }

    if !remainder
        .chars()
        .all(|c| c.is_ascii_digit() || (c == '.' && format.kind == NumberKind::Float))
    {
        return false;
    }

    let dot_count = remainder.chars().filter(|c| *c == '.').count();
    if dot_count > 1 {
        return false;
    }

    if format.kind == NumberKind::Integer && dot_count > 0 {
        return false;
    }

    true
}

#[component]
pub fn NumericInput(
    label: String,
    mut value: Signal<Option<f64>>,
    format: FormatOptions,
) -> Element {
    let mut text = use_signal(|| value().map(|v| v.to_string()).unwrap_or_default());

    rsx! {
        div {
            class: "flex flex-col gap-1",
            label {
                class: "text-sm",
                "{label}"
            }
            input {
                r#type: "text",
                class: "rounded max-w-40 border border-slate-300 px-2 py-1 text-amber-50 font-mono",
                value: text(),
                oninput: move |e| {
                    let next = e.value();

                    if !is_allowed_numeric_input(&next, format) {
                        return;
                    }

                    if next.is_empty() {
                        *text.write() = next;
                        *value.write() = None;
                    } else if let Ok(parsed) = next.parse::<f64>() {
                        if parsed < format.min || parsed > format.max {
                            return;
                        }

                        *text.write() = next;
                        *value.write() = Some(parsed);
                    } else {
                        // Keep intermediary values like "-" and "1." while editing.
                        *text.write() = next;
                    }
                }
            }
        }
    }
}