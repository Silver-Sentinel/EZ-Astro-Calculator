use dioxus::prelude::*;
use astro_calc::{calculate_synastry_charts, format_synastry_chart, ChartInput};
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
pub fn SynastryTab() -> Element {
    // Person 1 form state
    let mut name1 = use_signal(|| String::from(""));
    let mut gender1 = use_signal(|| String::from("Male"));
    let mut birth_date1 = use_signal(|| String::from(""));
    let mut birth_time1 = use_signal(|| String::from(""));
    let mut timezone1 = use_signal(|| String::from("America/New_York"));  // Changed to IANA
    let mut latitude1 = use_signal(|| String::from(""));
    let mut longitude1 = use_signal(|| String::from(""));
    
    // Person 2 form state
    let mut name2 = use_signal(|| String::from(""));
    let mut gender2 = use_signal(|| String::from("Female"));
    let mut birth_date2 = use_signal(|| String::from(""));
    let mut birth_time2 = use_signal(|| String::from(""));
    let mut timezone2 = use_signal(|| String::from("America/New_York"));  // Changed to IANA
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
            
            // ═══════════════════════════════════════════════════════════════
            // ✅ NEW: PROPER TIMEZONE HANDLING FOR PERSON 1
            // ═══════════════════════════════════════════════════════════════
            
            let tz1_str = timezone1.read().clone();
            let tz1: Tz = match tz1_str.parse() {
                Ok(tz) => tz,
                Err(_) => {
                    error_message.set(format!(
                        "Invalid timezone for Person 1: {}. Use IANA format (e.g., America/Los_Angeles)",
                        tz1_str
                    ));
                    is_calculating.set(false);
                    return;
                }
            };
            
            // Parse date and time for Person 1
            let birth_date1_str = birth_date1.read().clone();
            let birth_time1_str = birth_time1.read().clone();
            let date_parts1: Vec<&str> = birth_date1_str.split('-').collect();
            let time_parts1: Vec<&str> = birth_time1_str.split(':').collect();
            
            if date_parts1.len() != 3 || time_parts1.len() != 2 {
                error_message.set("Invalid date or time format for Person 1".to_string());
                is_calculating.set(false);
                return;
            }
            
            let year1: i32 = date_parts1[0].parse().unwrap_or(0);
            let month1: u32 = date_parts1[1].parse().unwrap_or(0);
            let day1: u32 = date_parts1[2].parse().unwrap_or(0);
            let hour1: u32 = time_parts1[0].parse().unwrap_or(0);
            let minute1: u32 = time_parts1[1].parse().unwrap_or(0);
            
            let naive_date1 = match NaiveDate::from_ymd_opt(year1, month1, day1) {
                Some(d) => d,
                None => {
                    error_message.set(format!("Invalid date for Person 1: {}-{}-{}", year1, month1, day1));
                    is_calculating.set(false);
                    return;
                }
            };
            
            let naive_time1 = match NaiveTime::from_hms_opt(hour1, minute1, 0) {
                Some(t) => t,
                None => {
                    error_message.set(format!("Invalid time for Person 1: {}:{}", hour1, minute1));
                    is_calculating.set(false);
                    return;
                }
            };
            
            let naive_dt1 = NaiveDateTime::new(naive_date1, naive_time1);
            let local_dt1 = match tz1.from_local_datetime(&naive_dt1).single() {
                Some(dt) => dt,
                None => {
                    error_message.set("Ambiguous time for Person 1 (DST transition)".to_string());
                    is_calculating.set(false);
                    return;
                }
            };
            let dt1 = local_dt1.with_timezone(&Utc);
            
            // ═══════════════════════════════════════════════════════════════
            // ✅ NEW: PROPER TIMEZONE HANDLING FOR PERSON 2
            // ═══════════════════════════════════════════════════════════════
            
            let tz2_str = timezone2.read().clone();
            let tz2: Tz = match tz2_str.parse() {
                Ok(tz) => tz,
                Err(_) => {
                    error_message.set(format!(
                        "Invalid timezone for Person 2: {}. Use IANA format (e.g., America/Los_Angeles)",
                        tz2_str
                    ));
                    is_calculating.set(false);
                    return;
                }
            };
            
            // Parse date and time for Person 2
            let birth_date2_str = birth_date2.read().clone();
            let birth_time2_str = birth_time2.read().clone();
            let date_parts2: Vec<&str> = birth_date2_str.split('-').collect();
            let time_parts2: Vec<&str> = birth_time2_str.split(':').collect();
            
            if date_parts2.len() != 3 || time_parts2.len() != 2 {
                error_message.set("Invalid date or time format for Person 2".to_string());
                is_calculating.set(false);
                return;
            }
            
            let year2: i32 = date_parts2[0].parse().unwrap_or(0);
            let month2: u32 = date_parts2[1].parse().unwrap_or(0);
            let day2: u32 = date_parts2[2].parse().unwrap_or(0);
            let hour2: u32 = time_parts2[0].parse().unwrap_or(0);
            let minute2: u32 = time_parts2[1].parse().unwrap_or(0);
            
            let naive_date2 = match NaiveDate::from_ymd_opt(year2, month2, day2) {
                Some(d) => d,
                None => {
                    error_message.set(format!("Invalid date for Person 2: {}-{}-{}", year2, month2, day2));
                    is_calculating.set(false);
                    return;
                }
            };
            
            let naive_time2 = match NaiveTime::from_hms_opt(hour2, minute2, 0) {
                Some(t) => t,
                None => {
                    error_message.set(format!("Invalid time for Person 2: {}:{}", hour2, minute2));
                    is_calculating.set(false);
                    return;
                }
            };
            
            let naive_dt2 = NaiveDateTime::new(naive_date2, naive_time2);
            let local_dt2 = match tz2.from_local_datetime(&naive_dt2).single() {
                Some(dt) => dt,
                None => {
                    error_message.set("Ambiguous time for Person 2 (DST transition)".to_string());
                    is_calculating.set(false);
                    return;
                }
            };
            let dt2 = local_dt2.with_timezone(&Utc);
            
            // Create chart inputs
            let input1 = ChartInput::new(dt1, lat1, lon1)
                .with_name(name1.read().clone())
                .with_gender(gender1.read().clone());
            
            let input2 = ChartInput::new(dt2, lat2, lon2)
                .with_name(name2.read().clone())
                .with_gender(gender2.read().clone());
            
            // Calculate synastry
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
                        placeholder: "e.g., America/Los_Angeles"
                    }
                    p { class: "hint", "IANA timezone format (handles DST automatically)" }
                }
                
                div { class: "form-row",
                    div { class: "form-group",
                        label { "Latitude" }
                        input {
                            r#type: "text",
                            value: "{latitude1}",
                            oninput: move |evt| {
                                latitude1.set(evt.value());
                                // Auto-guess timezone when both coords are set
                                if let (Ok(lat), Ok(lon)) = (
                                    latitude1.read().parse::<f64>(),
                                    longitude1.read().parse::<f64>()
                                ) {
                                    let tz = guess_timezone_from_coords(lat, lon);
                                    timezone1.set(tz.to_string());
                                }
                            },
                            placeholder: "e.g., 36.7477"
                        }
                    }
                    div { class: "form-group",
                        label { "Longitude" }
                        input {
                            r#type: "text",
                            value: "{longitude1}",
                            oninput: move |evt| {
                                longitude1.set(evt.value());
                                // Auto-guess timezone when both coords are set
                                if let (Ok(lat), Ok(lon)) = (
                                    latitude1.read().parse::<f64>(),
                                    longitude1.read().parse::<f64>()
                                ) {
                                    let tz = guess_timezone_from_coords(lat, lon);
                                    timezone1.set(tz.to_string());
                                }
                            },
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
                        placeholder: "e.g., America/New_York"
                    }
                    p { class: "hint", "IANA timezone format (handles DST automatically)" }
                }
                
                div { class: "form-row",
                    div { class: "form-group",
                        label { "Latitude" }
                        input {
                            r#type: "text",
                            value: "{latitude2}",
                            oninput: move |evt| {
                                latitude2.set(evt.value());
                                // Auto-guess timezone when both coords are set
                                if let (Ok(lat), Ok(lon)) = (
                                    latitude2.read().parse::<f64>(),
                                    longitude2.read().parse::<f64>()
                                ) {
                                    let tz = guess_timezone_from_coords(lat, lon);
                                    timezone2.set(tz.to_string());
                                }
                            },
                            placeholder: "e.g., 40.7128"
                        }
                    }
                    div { class: "form-group",
                        label { "Longitude" }
                        input {
                            r#type: "text",
                            value: "{longitude2}",
                            oninput: move |evt| {
                                longitude2.set(evt.value());
                                // Auto-guess timezone when both coords are set
                                if let (Ok(lat), Ok(lon)) = (
                                    latitude2.read().parse::<f64>(),
                                    longitude2.read().parse::<f64>()
                                ) {
                                    let tz = guess_timezone_from_coords(lat, lon);
                                    timezone2.set(tz.to_string());
                                }
                            },
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
                        "⏳ Calculating..."
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
