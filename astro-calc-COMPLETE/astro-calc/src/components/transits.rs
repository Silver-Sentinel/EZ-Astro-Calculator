use dioxus::prelude::*;
use astro_calc::{calculate_transit_chart, format_transit_chart, ChartInput};
use chrono::{Utc, NaiveDateTime, NaiveDate, NaiveTime, TimeZone};
use chrono_tz::Tz;

/// Guess IANA timezone from coordinates
/// This is a simplified mapping for common locations. Users can override if needed.
fn guess_timezone_from_coords(lat: f64, lon: f64) -> &'static str {
    // North America
    if lat >= 25.0 && lat <= 72.0 {
        if lon >= -170.0 && lon <= -140.0 {
            return "America/Anchorage";  // Alaska
        } else if lon >= -125.0 && lon <= -114.0 {
            return "America/Los_Angeles";  // Pacific
        } else if lon >= -115.0 && lon <= -104.0 {
            if lat >= 31.0 && lat <= 37.0 && lon >= -114.0 && lon <= -109.0 {
                return "America/Phoenix";  // Arizona (no DST)
            }
            return "America/Denver";  // Mountain
        } else if lon >= -105.0 && lon <= -87.0 {
            return "America/Chicago";  // Central
        } else if lon >= -88.0 && lon <= -67.0 {
            return "America/New_York";  // Eastern
        }
    }
    
    // Europe
    if lat >= 35.0 && lat <= 71.0 && lon >= -10.0 && lon <= 40.0 {
        if lon >= -10.0 && lon <= 0.0 {
            return "Europe/London";
        } else if lon >= 0.0 && lon <= 15.0 {
            return "Europe/Paris";
        } else if lon >= 15.0 && lon <= 30.0 {
            return "Europe/Berlin";
        } else {
            return "Europe/Moscow";
        }
    }
    
    // Asia
    if lat >= -10.0 && lat <= 55.0 && lon >= 60.0 && lon <= 180.0 {
        if lon >= 60.0 && lon <= 90.0 {
            return "Asia/Kolkata";
        } else if lon >= 100.0 && lon <= 110.0 {
            return "Asia/Bangkok";
        } else if lon >= 115.0 && lon <= 125.0 {
            return "Asia/Shanghai";
        } else if lon >= 135.0 && lon <= 145.0 {
            return "Asia/Tokyo";
        }
    }
    
    // Australia
    if lat >= -45.0 && lat <= -10.0 && lon >= 110.0 && lon <= 155.0 {
        if lon >= 110.0 && lon <= 130.0 {
            return "Australia/Perth";
        } else if lon >= 135.0 && lon <= 145.0 {
            return "Australia/Adelaide";
        } else {
            return "Australia/Sydney";
        }
    }
    
    // South America
    if lat >= -55.0 && lat <= 12.0 && lon >= -82.0 && lon <= -34.0 {
        if lon >= -82.0 && lon <= -75.0 {
            return "America/Lima";
        } else if lon >= -75.0 && lon <= -50.0 {
            return "America/Sao_Paulo";
        } else {
            return "America/Argentina/Buenos_Aires";
        }
    }
    
    // Africa
    if lat >= -35.0 && lat <= 37.0 && lon >= -17.0 && lon <= 52.0 {
        if lon >= 25.0 && lon <= 32.0 && lat >= -30.0 && lat <= -22.0 {
            return "Africa/Johannesburg";
        } else if lon >= 30.0 && lon <= 32.0 && lat >= 29.0 && lat <= 32.0 {
            return "Africa/Cairo";
        } else {
            return "Africa/Lagos";
        }
    }
    
    "UTC"
}

#[component]
pub fn TransitsTab() -> Element {
    // Natal person state
    let mut name = use_signal(|| String::from(""));
    let mut gender = use_signal(|| String::from("Male"));
    let mut birth_date = use_signal(|| String::from(""));
    let mut birth_time = use_signal(|| String::from(""));
    let mut timezone = use_signal(|| String::from("America/New_York"));  // Changed to IANA
    let mut latitude = use_signal(|| String::from(""));
    let mut longitude = use_signal(|| String::from(""));
    
    // Transit date state
    let mut transit_date = use_signal(|| String::from(""));
    let mut transit_time = use_signal(|| String::from("12:00"));
    let mut transit_timezone = use_signal(|| String::from("America/New_York"));  // Changed to IANA
    
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
            
            // ═══════════════════════════════════════════════════════════════
            // ✅ NEW: PROPER TIMEZONE HANDLING FOR NATAL CHART
            // ═══════════════════════════════════════════════════════════════
            
            let natal_tz_str = timezone.read().clone();
            let natal_tz: Tz = match natal_tz_str.parse() {
                Ok(tz) => tz,
                Err(_) => {
                    error_message.set(format!(
                        "Invalid natal timezone: {}. Use IANA format (e.g., America/Los_Angeles)",
                        natal_tz_str
                    ));
                    is_calculating.set(false);
                    return;
                }
            };
            
            // Parse natal date and time
            let birth_date_str = birth_date.read().clone();
            let birth_time_str = birth_time.read().clone();
            let natal_date_parts: Vec<&str> = birth_date_str.split('-').collect();
            let natal_time_parts: Vec<&str> = birth_time_str.split(':').collect();
            
            if natal_date_parts.len() != 3 || natal_time_parts.len() != 2 {
                error_message.set("Invalid natal date or time format".to_string());
                is_calculating.set(false);
                return;
            }
            
            let natal_year: i32 = natal_date_parts[0].parse().unwrap_or(0);
            let natal_month: u32 = natal_date_parts[1].parse().unwrap_or(0);
            let natal_day: u32 = natal_date_parts[2].parse().unwrap_or(0);
            let natal_hour: u32 = natal_time_parts[0].parse().unwrap_or(0);
            let natal_minute: u32 = natal_time_parts[1].parse().unwrap_or(0);
            
            let natal_naive_date = match NaiveDate::from_ymd_opt(natal_year, natal_month, natal_day) {
                Some(d) => d,
                None => {
                    error_message.set(format!("Invalid natal date: {}-{}-{}", natal_year, natal_month, natal_day));
                    is_calculating.set(false);
                    return;
                }
            };
            
            let natal_naive_time = match NaiveTime::from_hms_opt(natal_hour, natal_minute, 0) {
                Some(t) => t,
                None => {
                    error_message.set(format!("Invalid natal time: {}:{}", natal_hour, natal_minute));
                    is_calculating.set(false);
                    return;
                }
            };
            
            let natal_naive_dt = NaiveDateTime::new(natal_naive_date, natal_naive_time);
            let natal_local_dt = match natal_tz.from_local_datetime(&natal_naive_dt).single() {
                Some(dt) => dt,
                None => {
                    error_message.set("Ambiguous natal time (DST transition)".to_string());
                    is_calculating.set(false);
                    return;
                }
            };
            let natal_dt = natal_local_dt.with_timezone(&Utc);
            
            // ═══════════════════════════════════════════════════════════════
            // ✅ NEW: PROPER TIMEZONE HANDLING FOR TRANSIT DATE
            // ═══════════════════════════════════════════════════════════════
            
            let transit_tz_str = transit_timezone.read().clone();
            let transit_tz: Tz = match transit_tz_str.parse() {
                Ok(tz) => tz,
                Err(_) => {
                    error_message.set(format!(
                        "Invalid transit timezone: {}. Use IANA format (e.g., America/Los_Angeles)",
                        transit_tz_str
                    ));
                    is_calculating.set(false);
                    return;
                }
            };
            
            // Parse transit date and time
            let transit_date_str = transit_date.read().clone();
            let transit_time_str = transit_time.read().clone();
            let transit_date_parts: Vec<&str> = transit_date_str.split('-').collect();
            let transit_time_parts: Vec<&str> = transit_time_str.split(':').collect();
            
            if transit_date_parts.len() != 3 || transit_time_parts.len() != 2 {
                error_message.set("Invalid transit date or time format".to_string());
                is_calculating.set(false);
                return;
            }
            
            let transit_year: i32 = transit_date_parts[0].parse().unwrap_or(0);
            let transit_month: u32 = transit_date_parts[1].parse().unwrap_or(0);
            let transit_day: u32 = transit_date_parts[2].parse().unwrap_or(0);
            let transit_hour: u32 = transit_time_parts[0].parse().unwrap_or(0);
            let transit_minute: u32 = transit_time_parts[1].parse().unwrap_or(0);
            
            let transit_naive_date = match NaiveDate::from_ymd_opt(transit_year, transit_month, transit_day) {
                Some(d) => d,
                None => {
                    error_message.set(format!("Invalid transit date: {}-{}-{}", transit_year, transit_month, transit_day));
                    is_calculating.set(false);
                    return;
                }
            };
            
            let transit_naive_time = match NaiveTime::from_hms_opt(transit_hour, transit_minute, 0) {
                Some(t) => t,
                None => {
                    error_message.set(format!("Invalid transit time: {}:{}", transit_hour, transit_minute));
                    is_calculating.set(false);
                    return;
                }
            };
            
            let transit_naive_dt = NaiveDateTime::new(transit_naive_date, transit_naive_time);
            let transit_local_dt = match transit_tz.from_local_datetime(&transit_naive_dt).single() {
                Some(dt) => dt,
                None => {
                    error_message.set("Ambiguous transit time (DST transition)".to_string());
                    is_calculating.set(false);
                    return;
                }
            };
            let transit_dt = transit_local_dt.with_timezone(&Utc);
            
            // Create natal chart input
            let natal_input = ChartInput::new(natal_dt, lat, lon)
                .with_name(name.read().clone())
                .with_gender(gender.read().clone());
            
            // Calculate transits
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
                        placeholder: "e.g., America/Los_Angeles"
                    }
                    p { class: "hint", "IANA timezone format (handles DST automatically)" }
                }
                
                div { class: "form-row",
                    div { class: "form-group",
                        label { "Latitude" }
                        input {
                            r#type: "text",
                            value: "{latitude}",
                            oninput: move |evt| {
                                latitude.set(evt.value());
                                // Auto-guess timezone when both coords are set
                                if let (Ok(lat), Ok(lon)) = (
                                    latitude.read().parse::<f64>(),
                                    longitude.read().parse::<f64>()
                                ) {
                                    let tz = guess_timezone_from_coords(lat, lon);
                                    timezone.set(tz.to_string());
                                }
                            },
                            placeholder: "e.g., 36.7477"
                        }
                    }
                    div { class: "form-group",
                        label { "Longitude" }
                        input {
                            r#type: "text",
                            value: "{longitude}",
                            oninput: move |evt| {
                                longitude.set(evt.value());
                                // Auto-guess timezone when both coords are set
                                if let (Ok(lat), Ok(lon)) = (
                                    latitude.read().parse::<f64>(),
                                    longitude.read().parse::<f64>()
                                ) {
                                    let tz = guess_timezone_from_coords(lat, lon);
                                    timezone.set(tz.to_string());
                                }
                            },
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
                        placeholder: "e.g., America/Los_Angeles"
                    }
                    p { class: "hint", 
                        "Current planetary positions for this date/time"
                        br {}
                        "IANA timezone format (handles DST automatically)"
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
                        "⏳ Calculating..."
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
                        pre {
                            style: "white-space: pre-wrap; font-family: 'Courier New', monospace;",
                            "{results}"
                        }
                    }
                }
                
                if !results.read().is_empty() {
                    div { class: "results-actions",
                        button {
                            class: "btn-secondary",
                            onclick: move |_| {
                                use copypasta::{ClipboardContext, ClipboardProvider};
                                if let Ok(mut ctx) = ClipboardContext::new() {
                                    let _ = ctx.set_contents(results.read().clone());
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
