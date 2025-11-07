use dioxus::prelude::*;
use astro_calc::{calculate_chart, format_natal_chart, ChartInput};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use urlencoding::encode;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LocationResult {
    name: String,
    country: String,
    admin1: String,
    latitude: f64,
    longitude: f64,
}

// Location search function (same as your Tauri command, but called directly!)
async fn search_location(query: String) -> Result<Vec<LocationResult>, String> {
    let url = format!(
        "http://api.geonames.org/searchJSON?q={}&maxRows=10&username=demo",
        encode(&query)
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    
    let data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    
    let mut results = Vec::new();
    
    if let Some(geonames) = data["geonames"].as_array() {
        for location in geonames {
            if let (Some(name), Some(country), Some(lat), Some(lon)) = (
                location["name"].as_str(),
                location["countryName"].as_str(),
                location["lat"].as_str(),
                location["lng"].as_str(),
            ) {
                let admin1 = location["adminName1"].as_str().unwrap_or("");
                
                results.push(LocationResult {
                    name: name.to_string(),
                    country: country.to_string(),
                    admin1: admin1.to_string(),
                    latitude: lat.parse().unwrap_or(0.0),
                    longitude: lon.parse().unwrap_or(0.0),
                });
            }
        }
    }

    Ok(results)
}

#[component]
pub fn NatalTab() -> Element {
    // Form state
    let mut name = use_signal(|| String::from(""));
    let mut gender = use_signal(|| String::from("Male"));
    let mut birth_date = use_signal(|| String::from(""));
    let mut birth_time = use_signal(|| String::from(""));
    let mut timezone = use_signal(|| String::from("-05:00"));
    let mut location_search = use_signal(|| String::from(""));
    let mut latitude = use_signal(|| String::from(""));
    let mut longitude = use_signal(|| String::from(""));
    
    // Location search state
    let mut search_results = use_signal(|| Vec::<LocationResult>::new());
    let mut show_results = use_signal(|| false);
    
    // Results state
    let mut results = use_signal(|| String::from(""));
    let mut error_message = use_signal(|| String::from(""));
    let mut is_calculating = use_signal(|| false);

    // Location search effect
    let _search_effect = use_resource(move || {
        let query = location_search.read().clone();
        async move {
            if query.len() >= 3 {
                match search_location(query).await {
                    Ok(results) => {
                        search_results.set(results);
                        show_results.set(true);
                    }
                    Err(_) => {
                        search_results.set(Vec::new());
                    }
                }
            } else {
                show_results.set(false);
            }
        }
    });

    let calculate = move |_| {
        spawn(async move {
            is_calculating.set(true);
            error_message.set(String::new());
            
            // Validation (same as your Tauri command!)
            if name.read().is_empty() {
                error_message.set("Please enter a name".to_string());
                is_calculating.set(false);
                return;
            }
            
            if birth_date.read().is_empty() || birth_time.read().is_empty() {
                error_message.set("Please enter birth date and time".to_string());
                is_calculating.set(false);
                return;
            }
            
            let lat_str = latitude.read().clone();
            let lon_str = longitude.read().clone();
            
            if lat_str.is_empty() || lon_str.is_empty() {
                error_message.set("Please enter location coordinates".to_string());
                is_calculating.set(false);
                return;
            }
            
            // Parse coordinates
            let lat = match lat_str.parse::<f64>() {
                Ok(v) => v,
                Err(_) => {
                    error_message.set("Invalid latitude format".to_string());
                    is_calculating.set(false);
                    return;
                }
            };
            
            let lon = match lon_str.parse::<f64>() {
                Ok(v) => v,
                Err(_) => {
                    error_message.set("Invalid longitude format".to_string());
                    is_calculating.set(false);
                    return;
                }
            };
            
            // Validate coordinates
            if lat < -90.0 || lat > 90.0 {
                error_message.set(format!("Invalid latitude: {}. Must be between -90 and +90.", lat));
                is_calculating.set(false);
                return;
            }
            
            if lon < -180.0 || lon > 180.0 {
                error_message.set(format!("Invalid longitude: {}. Must be between -180 and +180.", lon));
                is_calculating.set(false);
                return;
            }
            
            // Build ISO 8601 datetime string
            let datetime_str = format!("{}T{}:00{}", 
                birth_date.read(), 
                birth_time.read(), 
                timezone.read()
            );
            
            // Parse datetime
            let dt = match DateTime::parse_from_rfc3339(&datetime_str) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    error_message.set(format!("Invalid datetime: {}. Please check date, time, and timezone format.", e));
                    is_calculating.set(false);
                    return;
                }
            };
            
            // Create chart input (using your existing struct!)
            let input = ChartInput::new(dt, lat, lon)
                .with_name(name.read().clone())
                .with_gender(gender.read().clone());
            
            // Calculate chart (calling your existing function directly!)
            match calculate_chart(&input) {
                Ok(chart) => {
                    // Format output (using your existing formatter!)
                    let mut output = String::new();
                    output.push_str(&format!("═══════════════════════════════════════\n"));
                    output.push_str(&format!("{} ({})\n", name.read(), gender.read()));
                    output.push_str(&format!("═══════════════════════════════════════\n\n"));
                    output.push_str(&format_natal_chart(&chart));
                    
                    results.set(output);
                }
                Err(e) => {
                    error_message.set(format!("Calculation error: {}", e));
                }
            }
            
            is_calculating.set(false);
        });
    };

    let mut select_location = move |location: LocationResult| {
        latitude.set(location.latitude.to_string());
        longitude.set(location.longitude.to_string());
        location_search.set(format!("{}, {}, {}", location.name, location.admin1, location.country));
        show_results.set(false);
    };

    rsx! {
        div { class: "form-container",
            
            // Left Panel - Input Form
            div { class: "input-panel",
                h2 { "Person 1" }
                
                // Name
                div { class: "form-group",
                    label { "Name" }
                    input {
                        r#type: "text",
                        value: "{name}",
                        oninput: move |evt| name.set(evt.value()),
                        placeholder: "Enter name"
                    }
                }
                
                // Gender
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
                
                // Birth Date & Time
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
                
                // Timezone
                div { class: "form-group",
                    label { "Timezone" }
                    input {
                        r#type: "text",
                        value: "{timezone}",
                        oninput: move |evt| timezone.set(evt.value()),
                        placeholder: "+/-HH:MM (e.g., -05:00 for EST)"
                    }
                    p { class: "hint", "Format: +/-HH:MM (e.g., -05:00 for EST)" }
                }
                
                // Location Search
                div { class: "form-group location-search",
                    label { "Search Location 🔍" }
                    input {
                        r#type: "text",
                        value: "{location_search}",
                        oninput: move |evt| {
                            location_search.set(evt.value());
                        },
                        placeholder: "Type city name..."
                    }
                    
                    if *show_results.read() && !search_results.read().is_empty() {
                        div { class: "location-results",
                            for result in search_results.read().iter() {
                                div {
                                    class: "location-item",
                                    onclick: {
                                        let result = result.clone();
                                        move |_| select_location(result.clone())
                                    },
                                    "{result.name}, {result.admin1}, {result.country}"
                                }
                            }
                        }
                    }
                    
                    p { class: "hint", "Or enter coordinates manually below" }
                }
                
                // Coordinates
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
                        "Calculate Chart"
                    }
                }
            }
            
            // Right Panel - Results
            div { class: "results-panel",
                div { class: "results-display",
                    if results.read().is_empty() {
                        p { class: "results-placeholder",
                            "Enter birth information and click Calculate Chart to see results"
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
                                // Copy to clipboard functionality
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
