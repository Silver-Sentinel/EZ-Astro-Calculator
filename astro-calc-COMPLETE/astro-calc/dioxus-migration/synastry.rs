use dioxus::prelude::*;
use astro_calc::{calculate_synastry_charts, format_synastry_chart, ChartInput};
use chrono::{DateTime, Utc};

#[component]
pub fn SynastryTab() -> Element {
    // Person 1 state
    let mut name1 = use_signal(|| String::from(""));
    let mut gender1 = use_signal(|| String::from("Male"));
    let mut birth_date1 = use_signal(|| String::from(""));
    let mut birth_time1 = use_signal(|| String::from(""));
    let mut timezone1 = use_signal(|| String::from("-05:00"));
    let mut latitude1 = use_signal(|| String::from(""));
    let mut longitude1 = use_signal(|| String::from(""));
    
    // Person 2 state
    let mut name2 = use_signal(|| String::from(""));
    let mut gender2 = use_signal(|| String::from("Female"));
    let mut birth_date2 = use_signal(|| String::from(""));
    let mut birth_time2 = use_signal(|| String::from(""));
    let mut timezone2 = use_signal(|| String::from("-05:00"));
    let mut latitude2 = use_signal(|| String::from(""));
    let mut longitude2 = use_signal(|| String::from(""));
    
    // Results state
    let mut results = use_signal(|| String::from(""));
    let mut error_message = use_signal(|| String::from(""));
    let mut is_calculating = use_signal(|| false);

    let calculate = move |_| {
        spawn(async move {
            is_calculating.set(true);
            error_message.set(String::new());
            
            // Build datetime strings
            let datetime_str1 = format!("{}T{}:00{}", birth_date1.read(), birth_time1.read(), timezone1.read());
            let datetime_str2 = format!("{}T{}:00{}", birth_date2.read(), birth_time2.read(), timezone2.read());
            
            // Parse
            let dt1 = DateTime::parse_from_rfc3339(&datetime_str1).unwrap().with_timezone(&Utc);
            let dt2 = DateTime::parse_from_rfc3339(&datetime_str2).unwrap().with_timezone(&Utc);
            let lat1: f64 = latitude1.read().parse().unwrap();
            let lon1: f64 = longitude1.read().parse().unwrap();
            let lat2: f64 = latitude2.read().parse().unwrap();
            let lon2: f64 = longitude2.read().parse().unwrap();
            
            // Calculate
            let input1 = ChartInput::new(dt1, lat1, lon1).with_name(name1.read().clone()).with_gender(gender1.read().clone());
            let input2 = ChartInput::new(dt2, lat2, lon2).with_name(name2.read().clone()).with_gender(gender2.read().clone());
            
            match calculate_synastry_charts(&input1, &input2) {
                Ok((chart1, chart2)) => {
                    let output = format_synastry_chart(&chart1, &name1.read(), &chart2, &name2.read());
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
                h2 { "Person 1" }
                // Add full form fields here (similar to natal.rs)
                button {
                    class: "btn-primary",
                    onclick: calculate,
                    "Calculate Synastry"
                }
            }
            div { class: "results-panel",
                div { class: "results-display",
                    if results.read().is_empty() {
                        p { class: "results-placeholder", "Enter data for both people" }
                    } else {
                        "{results}"
                    }
                }
            }
        }
    }
}
