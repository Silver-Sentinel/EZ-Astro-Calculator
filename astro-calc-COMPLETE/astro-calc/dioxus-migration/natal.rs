use dioxus::prelude::*;
use astro_calc::{calculate_chart, format_natal_chart, ChartInput};
use chrono::{DateTime, Utc};
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
    let mut timezone = use_signal(|| String::from("-05:00"));
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
    // CHART CALCULATION
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
            
            // Create chart input
            let input = ChartInput::new(dt, lat, lon)
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

    let select_location = move |location: LocationResult| {
        latitude.set(location.latitude.to_string());
        longitude.set(location.longitude.to_string());
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
