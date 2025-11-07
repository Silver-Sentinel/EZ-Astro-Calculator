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
    let mut transit_time = use_signal(|| String::from("12:00"));
    let mut transit_timezone = use_signal(|| String::from("-05:00"));
    
    // Results state
    let mut results = use_signal(|| String::from(""));
    let mut error_message = use_signal(|| String::from(""));
    let mut is_calculating = use_signal(|| false);

    let calculate = move |_| {
        spawn(async move {
            is_calculating.set(true);
            error_message.set(String::new());
            
            // Validate natal person
            if name.read().is_empty() || birth_date.read().is_empty() || birth_time.read().is_empty() {
                error_message.set("Please complete all natal chart fields".to_string());
                is_calculating.set(false);
                return;
            }
            
            if transit_date.read().is_empty() {
                error_message.set("Please enter a transit date".to_string());
                is_calculating.set(false);
                return;
            }
            
            let lat: f64 = match latitude.read().parse() {
                Ok(v) => v,
                Err(_) => {
                    error_message.set("Invalid latitude format".to_string());
                    is_calculating.set(false);
                    return;
                }
            };
            
            let lon: f64 = match longitude.read().parse() {
                Ok(v) => v,
                Err(_) => {
                    error_message.set("Invalid longitude format".to_string());
                    is_calculating.set(false);
                    return;
                }
            };
            
            // Build datetime strings
            let natal_datetime_str = format!("{}T{}:00{}", birth_date.read(), birth_time.read(), timezone.read());
            let transit_datetime_str = format!("{}T{}:00{}", transit_date.read(), transit_time.read(), transit_timezone.read());
            
            // Parse datetimes
            let natal_dt = match DateTime::parse_from_rfc3339(&natal_datetime_str) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    error_message.set(format!("Invalid natal datetime: {}", e));
                    is_calculating.set(false);
                    return;
                }
            };
            
            let transit_dt = match DateTime::parse_from_rfc3339(&transit_datetime_str) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    error_message.set(format!("Invalid transit datetime: {}", e));
                    is_calculating.set(false);
                    return;
                }
            };
            
            // Create natal chart input (using your existing struct!)
            let natal_input = ChartInput::new(natal_dt, lat, lon)
                .with_name(name.read().clone())
                .with_gender(gender.read().clone());
            
            // Calculate transits (calling your existing function!)
            match calculate_transit_chart(&natal_input, &transit_dt) {
                Ok((natal_chart, transit_chart)) => {
                    let output = format_transit_chart(&natal_chart, &transit_chart);
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
            
            // Left Panel - Input Form
            div { class: "input-panel",
                
                // NATAL CHART SECTION
                h2 { "Natal Chart" }
                
                div { class: "form-group",
                    label { "Name" }
                    input {
                        r#type: "text",
                        value: "{name}",
                        oninput: move |evt| name.set(evt.value()),
                        placeholder: "Enter name"
                    }
                }
                
                div { class: "form-group",
                    label { "Gender" }
                    div { class: "radio-group",
                        label { class: "radio-label",
                            input {
                                r#type: "radio",
                                name: "gender",
                                checked: *gender.read() == "Male",
                                onchange: move |_| gender.set("Male".to_string())
                            }
                            "Male"
                        }
                        label { class: "radio-label",
                            input {
                                r#type: "radio",
                                name: "gender",
                                checked: *gender.read() == "Female",
                                onchange: move |_| gender.set("Female".to_string())
                            }
                            "Female"
                        }
                        label { class: "radio-label",
                            input {
                                r#type: "radio",
                                name: "gender",
                                checked: *gender.read() == "Other",
                                onchange: move |_| gender.set("Other".to_string())
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
                            value: "{birth_date}",
                            oninput: move |evt| birth_date.set(evt.value())
                        }
                    }
                    div { class: "form-group",
                        label { "Birth Time" }
                        input {
                            r#type: "time",
                            value: "{birth_time}",
                            oninput: move |evt| birth_time.set(evt.value())
                        }
                    }
                }
                
                div { class: "form-group",
                    label { "Timezone" }
                    input {
                        r#type: "text",
                        value: "{timezone}",
                        oninput: move |evt| timezone.set(evt.value()),
                        placeholder: "+/-HH:MM"
                    }
                }
                
                div { class: "form-row",
                    div { class: "form-group",
                        label { "Latitude" }
                        input {
                            r#type: "text",
                            value: "{latitude}",
                            oninput: move |evt| latitude.set(evt.value()),
                            placeholder: "e.g., 36.7477"
                        }
                    }
                    div { class: "form-group",
                        label { "Longitude" }
                        input {
                            r#type: "text",
                            value: "{longitude}",
                            oninput: move |evt| longitude.set(evt.value()),
                            placeholder: "e.g., -119.7724"
                        }
                    }
                }
                
                // TRANSIT DATE SECTION
                h3 { "Transit Date" }
                
                div { class: "form-row",
                    div { class: "form-group",
                        label { "Transit Date" }
                        input {
                            r#type: "date",
                            value: "{transit_date}",
                            oninput: move |evt| transit_date.set(evt.value())
                        }
                    }
                    div { class: "form-group",
                        label { "Transit Time" }
                        input {
                            r#type: "time",
                            value: "{transit_time}",
                            oninput: move |evt| transit_time.set(evt.value())
                        }
                    }
                }
                
                div { class: "form-group",
                    label { "Transit Timezone" }
                    input {
                        r#type: "text",
                        value: "{transit_timezone}",
                        oninput: move |evt| transit_timezone.set(evt.value()),
                        placeholder: "+/-HH:MM"
                    }
                    p { class: "hint", "Current planetary positions for this date/time" }
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
                        "Calculate Transits"
                    }
                }
            }
            
            // Right Panel - Results
            div { class: "results-panel",
                div { class: "results-display",
                    if results.read().is_empty() {
                        p { class: "results-placeholder",
                            "Enter natal chart data and transit date, then click Calculate Transits"
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
