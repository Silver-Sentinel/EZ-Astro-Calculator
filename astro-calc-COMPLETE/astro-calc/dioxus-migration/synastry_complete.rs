use dioxus::prelude::*;
use astro_calc::{calculate_synastry_charts, format_synastry_chart, ChartInput};
use chrono::{DateTime, Utc};

#[component]
pub fn SynastryTab() -> Element {
    // Person 1 form state
    let mut name1 = use_signal(|| String::from(""));
    let mut gender1 = use_signal(|| String::from("Male"));
    let mut birth_date1 = use_signal(|| String::from(""));
    let mut birth_time1 = use_signal(|| String::from(""));
    let mut timezone1 = use_signal(|| String::from("-05:00"));
    let mut latitude1 = use_signal(|| String::from(""));
    let mut longitude1 = use_signal(|| String::from(""));
    
    // Person 2 form state
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
            
            // Validate Person 1
            if name1.read().is_empty() || birth_date1.read().is_empty() || birth_time1.read().is_empty() {
                error_message.set("Please complete all fields for Person 1".to_string());
                is_calculating.set(false);
                return;
            }
            
            let lat1: f64 = match latitude1.read().parse() {
                Ok(v) => v,
                Err(_) => {
                    error_message.set("Invalid latitude for Person 1".to_string());
                    is_calculating.set(false);
                    return;
                }
            };
            
            let lon1: f64 = match longitude1.read().parse() {
                Ok(v) => v,
                Err(_) => {
                    error_message.set("Invalid longitude for Person 1".to_string());
                    is_calculating.set(false);
                    return;
                }
            };
            
            // Validate Person 2
            if name2.read().is_empty() || birth_date2.read().is_empty() || birth_time2.read().is_empty() {
                error_message.set("Please complete all fields for Person 2".to_string());
                is_calculating.set(false);
                return;
            }
            
            let lat2: f64 = match latitude2.read().parse() {
                Ok(v) => v,
                Err(_) => {
                    error_message.set("Invalid latitude for Person 2".to_string());
                    is_calculating.set(false);
                    return;
                }
            };
            
            let lon2: f64 = match longitude2.read().parse() {
                Ok(v) => v,
                Err(_) => {
                    error_message.set("Invalid longitude for Person 2".to_string());
                    is_calculating.set(false);
                    return;
                }
            };
            
            // Build datetime strings
            let datetime_str1 = format!("{}T{}:00{}", birth_date1.read(), birth_time1.read(), timezone1.read());
            let datetime_str2 = format!("{}T{}:00{}", birth_date2.read(), birth_time2.read(), timezone2.read());
            
            // Parse datetimes
            let dt1 = match DateTime::parse_from_rfc3339(&datetime_str1) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    error_message.set(format!("Invalid datetime for Person 1: {}", e));
                    is_calculating.set(false);
                    return;
                }
            };
            
            let dt2 = match DateTime::parse_from_rfc3339(&datetime_str2) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    error_message.set(format!("Invalid datetime for Person 2: {}", e));
                    is_calculating.set(false);
                    return;
                }
            };
            
            // Create chart inputs (using your existing struct!)
            let input1 = ChartInput::new(dt1, lat1, lon1)
                .with_name(name1.read().clone())
                .with_gender(gender1.read().clone());
            
            let input2 = ChartInput::new(dt2, lat2, lon2)
                .with_name(name2.read().clone())
                .with_gender(gender2.read().clone());
            
            // Calculate synastry (calling your existing function!)
            match calculate_synastry_charts(&input1, &input2) {
                Ok((chart1, chart2)) => {
                    let output = format_synastry_chart(&chart1, &name1.read(), &chart2, &name2.read());
                    results.set(output);
                }
                Err(e) => {
                    error_message.set(format!("Calculation error: {}", e));
                }
            }
            
            is_calculating.set(false);
        });
    };

    rsx! {
        div { class: "form-container",
            
            // Left Panel - Input Forms for both people
            div { class: "input-panel",
                
                // PERSON 1
                h2 { "Person 1" }
                
                div { class: "form-group",
                    label { "Name" }
                    input {
                        r#type: "text",
                        value: "{name1}",
                        oninput: move |evt| name1.set(evt.value()),
                        placeholder: "Enter name"
                    }
                }
                
                div { class: "form-group",
                    label { "Gender" }
                    div { class: "radio-group",
                        label { class: "radio-label",
                            input {
                                r#type: "radio",
                                name: "gender1",
                                checked: *gender1.read() == "Male",
                                onchange: move |_| gender1.set("Male".to_string())
                            }
                            "Male"
                        }
                        label { class: "radio-label",
                            input {
                                r#type: "radio",
                                name: "gender1",
                                checked: *gender1.read() == "Female",
                                onchange: move |_| gender1.set("Female".to_string())
                            }
                            "Female"
                        }
                        label { class: "radio-label",
                            input {
                                r#type: "radio",
                                name: "gender1",
                                checked: *gender1.read() == "Other",
                                onchange: move |_| gender1.set("Other".to_string())
                            }
                            "Other"
                        }
                    }
                }
                
                div { class: "form-row",
                    div { class: "form-group",
                        label { "Birth Date" }
                        input {
                            r#type: "date",
                            value: "{birth_date1}",
                            oninput: move |evt| birth_date1.set(evt.value())
                        }
                    }
                    div { class: "form-group",
                        label { "Birth Time" }
                        input {
                            r#type: "time",
                            value: "{birth_time1}",
                            oninput: move |evt| birth_time1.set(evt.value())
                        }
                    }
                }
                
                div { class: "form-group",
                    label { "Timezone" }
                    input {
                        r#type: "text",
                        value: "{timezone1}",
                        oninput: move |evt| timezone1.set(evt.value()),
                        placeholder: "+/-HH:MM"
                    }
                }
                
                div { class: "form-row",
                    div { class: "form-group",
                        label { "Latitude" }
                        input {
                            r#type: "text",
                            value: "{latitude1}",
                            oninput: move |evt| latitude1.set(evt.value()),
                            placeholder: "e.g., 36.7477"
                        }
                    }
                    div { class: "form-group",
                        label { "Longitude" }
                        input {
                            r#type: "text",
                            value: "{longitude1}",
                            oninput: move |evt| longitude1.set(evt.value()),
                            placeholder: "e.g., -119.7724"
                        }
                    }
                }
                
                // PERSON 2
                h3 { "Person 2" }
                
                div { class: "form-group",
                    label { "Name" }
                    input {
                        r#type: "text",
                        value: "{name2}",
                        oninput: move |evt| name2.set(evt.value()),
                        placeholder: "Enter name"
                    }
                }
                
                div { class: "form-group",
                    label { "Gender" }
                    div { class: "radio-group",
                        label { class: "radio-label",
                            input {
                                r#type: "radio",
                                name: "gender2",
                                checked: *gender2.read() == "Male",
                                onchange: move |_| gender2.set("Male".to_string())
                            }
                            "Male"
                        }
                        label { class: "radio-label",
                            input {
                                r#type: "radio",
                                name: "gender2",
                                checked: *gender2.read() == "Female",
                                onchange: move |_| gender2.set("Female".to_string())
                            }
                            "Female"
                        }
                        label { class: "radio-label",
                            input {
                                r#type: "radio",
                                name: "gender2",
                                checked: *gender2.read() == "Other",
                                onchange: move |_| gender2.set("Other".to_string())
                            }
                            "Other"
                        }
                    }
                }
                
                div { class: "form-row",
                    div { class: "form-group",
                        label { "Birth Date" }
                        input {
                            r#type: "date",
                            value: "{birth_date2}",
                            oninput: move |evt| birth_date2.set(evt.value())
                        }
                    }
                    div { class: "form-group",
                        label { "Birth Time" }
                        input {
                            r#type: "time",
                            value: "{birth_time2}",
                            oninput: move |evt| birth_time2.set(evt.value())
                        }
                    }
                }
                
                div { class: "form-group",
                    label { "Timezone" }
                    input {
                        r#type: "text",
                        value: "{timezone2}",
                        oninput: move |evt| timezone2.set(evt.value()),
                        placeholder: "+/-HH:MM"
                    }
                }
                
                div { class: "form-row",
                    div { class: "form-group",
                        label { "Latitude" }
                        input {
                            r#type: "text",
                            value: "{latitude2}",
                            oninput: move |evt| latitude2.set(evt.value()),
                            placeholder: "e.g., 40.7128"
                        }
                    }
                    div { class: "form-group",
                        label { "Longitude" }
                        input {
                            r#type: "text",
                            value: "{longitude2}",
                            oninput: move |evt| longitude2.set(evt.value()),
                            placeholder: "e.g., -74.0060"
                        }
                    }
                }
                
                // Error Message
                if !error_message.read().is_empty() {
                    div { class: "error-message",
                        "{error_message}"
                    }
                }
                
                // Calculate Button
                button {
                    class: "btn-primary",
                    onclick: calculate,
                    disabled: *is_calculating.read(),
                    if *is_calculating.read() {
                        "Calculating..."
                    } else {
                        "Calculate Synastry"
                    }
                }
            }
            
            // Right Panel - Results
            div { class: "results-panel",
                div { class: "results-display",
                    if results.read().is_empty() {
                        p { class: "results-placeholder",
                            "Enter birth information for both people and click Calculate Synastry"
                        }
                    } else {
                        "{results}"
                    }
                }
                
                if !results.read().is_empty() {
                    div { class: "results-actions",
                        button {
                            class: "btn-secondary",
                            onclick: move |_| {
                                if let Ok(mut ctx) = copypasta::ClipboardContext::new() {
                                    let _ = copypasta::ClipboardProvider::set_contents(&mut ctx, results.read().clone());
                                }
                            },
                            "📋 Copy to Clipboard"
                        }
                    }
                }
            }
        }
    }
}
