use dioxus::{prelude::*};

#[derive(Clone, Copy, PartialEq)]
pub enum NumberKind {
    Integer,
    Float,
}

#[derive(Copy, Clone, PartialEq)]
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

fn is_allowed_numeric_input(input: &str, format: FormatOptions) -> Result<f64,()> {
    if input.is_empty() {
        return Ok(0.0);
    }
    let parsed = input.parse::<f64>().map_err(|_|())?
                    .clamp(format.min, format.max);
    match format.kind{
        NumberKind::Integer => Ok(parsed.trunc()),
        NumberKind::Float => Ok(parsed),
    }        
}

#[component]
pub fn NumericInput(
    label: String,
    mut value: Signal<f64>,
    format: FormatOptions,
) -> Element {
    let text_value = use_memo(move || value.to_string());
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
                value: "{text_value}",
                oninput: move |e| {
                    if let Ok(parsed) = is_allowed_numeric_input(&e.value(), format) {
                        *value.write() = parsed;
                    }
                }
            }
        }
    }
}