use dioxus::prelude::*;
use astro_calc::{calculate_chart, format_natal_chart, ChartInput};
use chrono::{Utc, NaiveDateTime, NaiveDate, NaiveTime, TimeZone};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// ═══════════════════════════════════════════════════════════════════════════
// LOCATION SEARCH - PHOTON API (PRIMARY)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PhotonResponse {
    features: Vec<PhotonFeature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PhotonFeature {
    properties: PhotonProperties,
    geometry: PhotonGeometry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PhotonProperties {
    name: String,
    country: Option<String>,
    state: Option<String>,
    city: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PhotonGeometry {
    coordinates: Vec<f64>, // [longitude, latitude]
}

#[derive(Debug, Clone)]
struct LocationResult {
    display_name: String,
    latitude: f64,
    longitude: f64,
}

/// Guess IANA timezone from coordinates
/// This is a simplified mapping for common locations. Users can override if needed.
/// 
/// For production use, consider using a timezone lookup service like:
/// - https://timezonedb.com/api
/// - https://www.geonames.org/export/web-services.html
/// 
/// This function provides reasonable defaults for major regions.
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
            return "Europe/London";  // UK/Ireland/Portugal
        } else if lon >= 0.0 && lon <= 15.0 {
            return "Europe/Paris";  // Western Europe
        } else if lon >= 15.0 && lon <= 30.0 {
            return "Europe/Berlin";  // Central Europe
        } else {
            return "Europe/Moscow";  // Eastern Europe
        }
    }
    
    // Asia
    if lat >= -10.0 && lat <= 55.0 && lon >= 60.0 && lon <= 180.0 {
        if lon >= 60.0 && lon <= 90.0 {
            return "Asia/Kolkata";  // India
        } else if lon >= 100.0 && lon <= 110.0 {
            return "Asia/Bangkok";  // SE Asia
        } else if lon >= 115.0 && lon <= 125.0 {
            return "Asia/Shanghai";  // China
        } else if lon >= 135.0 && lon <= 145.0 {
            return "Asia/Tokyo";  // Japan
        }
    }
    
    // Australia
    if lat >= -45.0 && lat <= -10.0 && lon >= 110.0 && lon <= 155.0 {
        if lon >= 110.0 && lon <= 130.0 {
            return "Australia/Perth";  // Western
        } else if lon >= 135.0 && lon <= 145.0 {
            return "Australia/Adelaide";  // Central
        } else {
            return "Australia/Sydney";  // Eastern
        }
    }
    
    // South America
    if lat >= -55.0 && lat <= 12.0 && lon >= -82.0 && lon <= -34.0 {
        if lon >= -82.0 && lon <= -75.0 {
            return "America/Lima";  // Peru/Colombia
        } else if lon >= -75.0 && lon <= -50.0 {
            return "America/Sao_Paulo";  // Brazil
        } else {
            return "America/Argentina/Buenos_Aires";
        }
    }
    
    // Africa
    if lat >= -35.0 && lat <= 37.0 && lon >= -17.0 && lon <= 52.0 {
        if lon >= 25.0 && lon <= 32.0 && lat >= -30.0 && lat <= -22.0 {
            return "Africa/Johannesburg";  // South Africa
        } else if lon >= 30.0 && lon <= 32.0 && lat >= 29.0 && lat <= 32.0 {
            return "Africa/Cairo";  // Egypt
        } else {
            return "Africa/Lagos";  // West Africa
        }
    }
    
    // Default: UTC
    "UTC"
}

/// Search locations using Photon API (primary, no auth needed, autocomplete-friendly)
async fn search_location_photon(query: &str) -> Result<Vec<LocationResult>, String> {
    let url = format!(
        "https://photon.komoot.io/api/?q={}&limit=10",
        urlencoding::encode(query)
    );

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                "Request timed out".to_string()
            } else if e.is_connect() {
                "Connection failed - check internet connection".to_string()
            } else {
                format!("Network error: {}", e)
            }
        })?;

    if !response.status().is_success() {
        return Err(format!("API returned status {}", response.status()));
    }

    let data: PhotonResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let mut results = Vec::new();
    
    for feature in data.features {
        let props = feature.properties;
        let coords = feature.geometry.coordinates;
        
        if coords.len() >= 2 {
            // Build display name from available components
            let mut parts = Vec::new();
            parts.push(props.name.clone());
            
            if let Some(city) = props.city {
                if city != props.name {
                    parts.push(city);
                }
            }
            
            if let Some(state) = props.state {
                parts.push(state);
            }
            
            if let Some(country) = props.country {
                parts.push(country);
            }
            
            let display_name = parts.join(", ");
            
            results.push(LocationResult {
                display_name,
                longitude: coords[0],
                latitude: coords[1],
            });
        }
    }

    if results.is_empty() {
        Err("No locations found".to_string())
    } else {
        Ok(results)
    }
}

/// Fallback to GeoNames API if Photon fails
/// Configured with user's GeoNames account: AquarianRising
async fn search_location_geonames(query: &str) -> Result<Vec<LocationResult>, String> {
    let username = "AquarianRising"; // Your GeoNames username
    
    let url = format!(
        "http://api.geonames.org/searchJSON?q={}&maxRows=10&username={}",
        urlencoding::encode(query),
        username
    );

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("GeoNames API returned status {}", response.status()));
    }

    let data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    
    // Check for API errors
    if let Some(status) = data.get("status") {
        if let Some(msg) = status.get("message").and_then(|m| m.as_str()) {
            return Err(format!("GeoNames error: {}. Please enable web services at http://www.geonames.org/manageaccount", msg));
        }
    }
    
    let mut results = Vec::new();
    
    if let Some(geonames) = data["geonames"].as_array() {
        for location in geonames {
            if let (Some(name), Some(country), Some(lat_str), Some(lon_str)) = (
                location["name"].as_str(),
                location["countryName"].as_str(),
                location["lat"].as_str(),
                location["lng"].as_str(),
            ) {
                let admin1 = location["adminName1"].as_str().unwrap_or("");
                
                if let (Ok(lat), Ok(lon)) = (lat_str.parse::<f64>(), lon_str.parse::<f64>()) {
                    let display_name = if admin1.is_empty() {
                        format!("{}, {}", name, country)
                    } else {
                        format!("{}, {}, {}", name, admin1, country)
                    };
                    
                    results.push(LocationResult {
                        display_name,
                        latitude: lat,
                        longitude: lon,
                    });
                }
            }
        }
    }

    if results.is_empty() {
        Err("No locations found".to_string())
    } else {
        Ok(results)
    }
}

/// Main search function with fallback logic
async fn search_locations(query: String) -> Result<Vec<LocationResult>, String> {
    // Try Photon first (no auth, designed for autocomplete)
    match search_location_photon(&query).await {
        Ok(results) => Ok(results),
        Err(photon_error) => {
            // Photon failed, try GeoNames as fallback
            tracing::warn!("Photon API failed: {}, trying GeoNames...", photon_error);
            search_location_geonames(&query).await
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// NATAL CHART COMPONENT
// ═══════════════════════════════════════════════════════════════════════════

#[component]
pub fn NatalTab() -> Element {
    // Form state
    let mut name = use_signal(|| String::from(""));
    let mut gender = use_signal(|| String::from("Male"));
    let mut birth_date = use_signal(|| String::from(""));
    let mut birth_time = use_signal(|| String::from(""));
    let mut timezone = use_signal(|| String::from("America/New_York"));  // Changed to IANA timezone
    let mut location_search = use_signal(|| String::from(""));
    let mut latitude = use_signal(|| String::from(""));
    let mut longitude = use_signal(|| String::from(""));
    
    // Location search state
    let mut show_results = use_signal(|| false);
    let mut search_error = use_signal(|| String::from(""));
    
    // Results state
    let mut results = use_signal(|| String::from(""));
    let mut error_message = use_signal(|| String::from(""));
    let mut is_calculating = use_signal(|| false);

    // ═══════════════════════════════════════════════════════════════════════
    // LOCATION SEARCH WITH PROPER REACTIVITY AND DEBOUNCING
    // ═══════════════════════════════════════════════════════════════════════
    
    let search_results = use_resource(move || async move {
        // ✓ CRITICAL: Read signal inside async block to create reactive dependency
        let query = location_search();
        
        // Clear previous errors
        search_error.set(String::new());
        
        // ✓ Debounce: wait before making request
        tokio::time::sleep(Duration::from_millis(300)).await;
        
        // Minimum query length check
        if query.len() < 3 {
            return Ok(Vec::new());
        }
        
        tracing::info!("Searching for location: {}", query);
        
        // Make the API call
        match search_locations(query).await {
            Ok(results) => {
                tracing::info!("Found {} locations", results.len());
                Ok(results)
            }
            Err(e) => {
                tracing::error!("Location search failed: {}", e);
                search_error.set(e.clone());
                Err(e)
            }
        }
    });

    // ═══════════════════════════════════════════════════════════════════════
    // CHART CALCULATION WITH PROPER DST HANDLING
    // ═══════════════════════════════════════════════════════════════════════

    let calculate = move |_| {
        spawn(async move {
            is_calculating.set(true);
            error_message.set(String::new());
            
            // Validation
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
            
            // ═══════════════════════════════════════════════════════════════
            // ✅ NEW: PROPER TIMEZONE HANDLING WITH DST SUPPORT
            // ═══════════════════════════════════════════════════════════════
            
            // Parse timezone string (IANA format like "America/Los_Angeles")
            let tz_str = timezone.read().clone();
            let tz: Tz = match tz_str.parse() {
                Ok(tz) => tz,
                Err(_) => {
                    error_message.set(format!(
                        "Invalid timezone: {}. Please use IANA timezone format (e.g., America/Los_Angeles)",
                        tz_str
                    ));
                    is_calculating.set(false);
                    return;
                }
            };
            
            // Parse date and time components
            let birth_date_str = birth_date.read().clone();
            let birth_time_str = birth_time.read().clone();
            let date_parts: Vec<&str> = birth_date_str.split('-').collect();
            let time_parts: Vec<&str> = birth_time_str.split(':').collect();
            
            if date_parts.len() != 3 || time_parts.len() != 2 {
                error_message.set("Invalid date or time format".to_string());
                is_calculating.set(false);
                return;
            }
            
            let year = match date_parts[0].parse::<i32>() {
                Ok(y) => y,
                Err(_) => {
                    error_message.set("Invalid year".to_string());
                    is_calculating.set(false);
                    return;
                }
            };
            
            let month = match date_parts[1].parse::<u32>() {
                Ok(m) => m,
                Err(_) => {
                    error_message.set("Invalid month".to_string());
                    is_calculating.set(false);
                    return;
                }
            };
            
            let day = match date_parts[2].parse::<u32>() {
                Ok(d) => d,
                Err(_) => {
                    error_message.set("Invalid day".to_string());
                    is_calculating.set(false);
                    return;
                }
            };
            
            let hour = match time_parts[0].parse::<u32>() {
                Ok(h) => h,
                Err(_) => {
                    error_message.set("Invalid hour".to_string());
                    is_calculating.set(false);
                    return;
                }
            };
            
            let minute = match time_parts[1].parse::<u32>() {
                Ok(m) => m,
                Err(_) => {
                    error_message.set("Invalid minute".to_string());
                    is_calculating.set(false);
                    return;
                }
            };
            
            // Build naive datetime (without timezone)
            let naive_date = match NaiveDate::from_ymd_opt(year, month, day) {
                Some(d) => d,
                None => {
                    error_message.set(format!("Invalid date: {}-{}-{}", year, month, day));
                    is_calculating.set(false);
                    return;
                }
            };
            
            let naive_time = match NaiveTime::from_hms_opt(hour, minute, 0) {
                Some(t) => t,
                None => {
                    error_message.set(format!("Invalid time: {}:{}", hour, minute));
                    is_calculating.set(false);
                    return;
                }
            };
            
            let naive_dt = NaiveDateTime::new(naive_date, naive_time);
            
            // Convert from local time in the specified timezone to UTC
            // This automatically handles DST! ✨
            let local_dt = match tz.from_local_datetime(&naive_dt).single() {
                Some(dt) => dt,
                None => {
                    error_message.set(format!(
                        "Ambiguous time (DST transition). Please verify date/time: {} {}",
                        birth_date.read(),
                        birth_time.read()
                    ));
                    is_calculating.set(false);
                    return;
                }
            };
            
            let utc_dt = local_dt.with_timezone(&Utc);
            
            // Create chart input
            let input = ChartInput::new(utc_dt, lat, lon)
                .with_name(name.read().clone())
                .with_gender(gender.read().clone());
            
            // Calculate chart
            match calculate_chart(&input) {
                Ok(chart) => {
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

    // ✅ NOW SETS PROPER IANA TIMEZONE!
    let mut select_location = move |location: LocationResult| {
        latitude.set(location.latitude.to_string());
        longitude.set(location.longitude.to_string());
        
        // ✅ NEW: Guess IANA timezone from coordinates (with DST support!)
        let tz_name = guess_timezone_from_coords(location.latitude, location.longitude);
        timezone.set(tz_name.to_string());
        
        location_search.set(location.display_name.clone());
        show_results.set(false);
        search_error.set(String::new());
    };

    // ═══════════════════════════════════════════════════════════════════════
    // RENDER UI
    // ═══════════════════════════════════════════════════════════════════════

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
                
                // Timezone (now using IANA format)
                div { class: "form-group",
                    label { "Timezone" }
                    input {
                        r#type: "text",
                        value: "{timezone}",
                        oninput: move |evt| timezone.set(evt.value()),
                        placeholder: "e.g., America/Los_Angeles"
                    }
                    p { class: "hint", 
                        "IANA timezone format (e.g., America/Los_Angeles, Europe/London)"
                        br {}
                        "Automatically handles DST for accurate calculations ✨"
                    }
                }
                
                // Location Search
                div { class: "form-group location-search",
                    label { "Search Location 🔍" }
                    input {
                        r#type: "text",
                        value: "{location_search}",
                        oninput: move |evt| {
                            let new_value = evt.value();
                            location_search.set(new_value);
                            if location_search.read().len() >= 3 {
                                show_results.set(true);
                            } else {
                                show_results.set(false);
                            }
                        },
                        onfocusout: move |_| {
                            // Delay hiding to allow clicking results
                            spawn(async move {
                                tokio::time::sleep(Duration::from_millis(200)).await;
                                show_results.set(false);
                            });
                        },
                        onfocus: move |_| {
                            if location_search.read().len() >= 3 {
                                show_results.set(true);
                            }
                        },
                        placeholder: "Type city name (min 3 characters)..."
                    }
                    
                    // Search status indicator
                    {match &*search_results.read_unchecked() {
                        None => rsx! {
                            p { class: "hint search-status", "🔄 Searching..." }
                        },
                        Some(Ok(_)) if !search_error.read().is_empty() => rsx! {
                            p { class: "hint error", "⚠️ {search_error}" }
                        },
                        _ => rsx! {}
                    }}
                    
                    // Dropdown results
                    if *show_results.read() {
                        {match &*search_results.read_unchecked() {
                            Some(Ok(locations)) if !locations.is_empty() => rsx! {
                                div { class: "location-results",
                                    for location in locations {
                                        div {
                                            class: "location-item",
                                            onclick: {
                                                let loc = location.clone();
                                                move |_| select_location(loc.clone())
                                            },
                                            "{location.display_name}"
                                            br {}
                                            small { 
                                                class: "coordinates",
                                                "Lat: {location.latitude:.4}, Lon: {location.longitude:.4}" 
                                            }
                                        }
                                    }
                                }
                            },
                            Some(Ok(_)) => rsx! {
                                div { class: "location-results",
                                    div { class: "location-item no-results",
                                        "No locations found. Try a different search term."
                                    }
                                }
                            },
                            Some(Err(e)) => rsx! {
                                div { class: "location-results",
                                    div { class: "location-item error",
                                        "⚠️ Search error: {e}"
                                    }
                                }
                            },
                            None => rsx! {
                                div { class: "location-results",
                                    div { class: "location-item loading",
                                        "🔄 Searching..."
                                    }
                                }
                            }
                        }}
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
                        "⏳ Calculating..."
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
                                // Copy to clipboard functionality
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
