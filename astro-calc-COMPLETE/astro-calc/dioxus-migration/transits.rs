use dioxus::prelude::*;
use astro_calc::{calculate_transit_chart, format_transit_chart, ChartInput};
use chrono::{DateTime, Utc};

#[component]
pub fn TransitsTab() -> Element {
    // Natal person state
    let mut name = use_signal(|| String::from(""));
    let mut gender = use_signal(|| String::from("Male"));
    let mut birth_date = use_signal(|| String::from(""));
    let mut birth_time = use_signal(|| String::from(""));
    let mut timezone = use_signal(|| String::from("-05:00"));
    let mut latitude = use_signal(|| String::from(""));
    let mut longitude = use_signal(|| String::from(""));
    
    // Transit date state
    let mut transit_date = use_signal(|| String::from(""));
    let mut transit_time = use_signal(|| String::from(""));
    let mut transit_timezone = use_signal(|| String::from("-05:00"));
    
    // Results state
    let mut results = use_signal(|| String::from(""));
    let mut error_message = use_signal(|| String::from(""));
    let mut is_calculating = use_signal(|| false);

    let calculate = move |_| {
        spawn(async move {
            is_calculating.set(true);
            error_message.set(String::new());
            
            // Build datetime strings
            let natal_datetime_str = format!("{}T{}:00{}", birth_date.read(), birth_time.read(), timezone.read());
            let transit_datetime_str = format!("{}T{}:00{}", transit_date.read(), transit_time.read(), transit_timezone.read());
            
            // Parse
            let natal_dt = DateTime::parse_from_rfc3339(&natal_datetime_str).unwrap().with_timezone(&Utc);
            let transit_dt = DateTime::parse_from_rfc3339(&transit_datetime_str).unwrap().with_timezone(&Utc);
            let lat: f64 = latitude.read().parse().unwrap();
            let lon: f64 = longitude.read().parse().unwrap();
            
            // Calculate
            let natal_input = ChartInput::new(natal_dt, lat, lon)
                .with_name(name.read().clone())
                .with_gender(gender.read().clone());
            
            match calculate_transit_chart(&natal_input, &transit_dt) {
                Ok((natal_chart, transit_chart)) => {
                    let output = format_transit_chart(&natal_chart, &transit_chart);
                    results.set(output);
                }
                Err(e) => {
                    error_message.set(format!("Error: {}", e));
                }
            }
            
            is_calculating.set(false);
        });
    };

    rsx! {
        div { class: "form-container",
            div { class: "input-panel",
                h2 { "Natal Chart" }
                // Add natal person form fields
                
                h3 { "Transit Date" }
                // Add transit date/time fields
                
                button {
                    class: "btn-primary",
                    onclick: calculate,
                    "Calculate Transits"
                }
            }
            div { class: "results-panel",
                div { class: "results-display",
                    if results.read().is_empty() {
                        p { class: "results-placeholder", "Enter natal data and transit date" }
                    } else {
                        "{results}"
                    }
                }
            }
        }
    }
}
