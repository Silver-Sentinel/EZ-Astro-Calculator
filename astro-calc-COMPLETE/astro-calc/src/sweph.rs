use crate::chart::{AnglePoint, AnglePosition, CelestialBody, Chart, HouseCusps, Position};
use chrono::{DateTime, Datelike, Timelike, Utc};
use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int};

// FFI declarations for Swiss Ephemeris
extern "C" {
    pub fn swe_set_ephe_path(path: *const c_char);
    pub fn swe_set_sid_mode(sid_mode: c_int, t0: c_double, ayan_t0: c_double);
    pub fn swe_julday(
        year: c_int,
        month: c_int,
        day: c_int,
        hour: c_double,
        gregflag: c_int,
    ) -> c_double;
    pub fn swe_calc_ut(
        tjd_ut: c_double,
        ipl: c_int,
        iflag: c_int,
        xx: *mut c_double,
        serr: *mut c_char,
    ) -> c_int;
    pub fn swe_houses(
        tjd_ut: c_double,
        geolat: c_double,
        geolon: c_double,
        hsys: c_int,
        cusps: *mut c_double,
        ascmc: *mut c_double,
    ) -> c_int;
    pub fn swe_houses_ex2(
        tjd_ut: c_double,
        iflag: c_int,
        geolat: c_double,
        geolon: c_double,
        hsys: c_int,
        cusps: *mut c_double,
        ascmc: *mut c_double,
        cusp_speed: *mut c_double,
        ascmc_speed: *mut c_double,
        serr: *mut c_char,
    ) -> c_int;
}

// Swiss Ephemeris constants
pub const FAGAN_BRADLEY_AYANAMSA: c_int = 1;
pub const PLACIDUS_HOUSE_SYSTEM: u8 = b'P';
pub const SEFLG_SIDEREAL: c_int = 65536; // 0x10000 - CRITICAL: Must be 65536, NOT 64!
pub const SEFLG_SPEED: c_int = 256;
pub const SEFLG_NONUT: c_int = 1024; // 0x400 - No nutation (for B1950 precision)
pub const SEFLG_J2000: c_int = 2048; // 0x800 - J2000 coordinates
pub const SE_GREG_CAL: c_int = 1;

// ═══════════════════════════════════════════════════════════════════════════
// EMPIRICAL CALIBRATION CORRECTION
// ═══════════════════════════════════════════════════════════════════════════
// After implementing all theoretical fixes (SEFLG_NONUT, proper ayanamsha, etc.),
// a systematic offset of ~1° remains. This empirical correction compensates for
// that offset to match professional astrology software (Astro.com, etc.).
//
// TO ADJUST: 
// - Start with -1.0 degrees
// - If positions undershoot (too low), reduce magnitude (e.g., -0.9, -0.8)
// - If positions overshoot (too high), increase magnitude (e.g., -1.1, -1.2)
// - Fine-tune in 0.1° increments until positions match target within arcminutes
//
// This correction is applied to:
// - All planetary longitudes
// - All house cusps
// - Ascendant and Midheaven
// - Vertex
// - Part of Fortune (inherits correction from ASC/Sun/Moon)
// ═══════════════════════════════════════════════════════════════════════════
pub const CALIBRATION_OFFSET_DEGREES: f64 = -1.0;

/// Normalize angle to 0-360 range after applying calibration
fn normalize_longitude(lon: f64) -> f64 {
    let mut result = (lon + CALIBRATION_OFFSET_DEGREES) % 360.0;
    if result < 0.0 {
        result += 360.0;
    }
    result
}

/// Input data for chart calculation
#[derive(Debug, Clone)]
pub struct ChartInput {
    pub datetime: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub name: Option<String>,
    pub gender: Option<String>, // "Male", "Female", "Other"
}

impl ChartInput {
    pub fn new(datetime: DateTime<Utc>, latitude: f64, longitude: f64) -> Self {
        Self {
            datetime,
            latitude,
            longitude,
            name: None,
            gender: None,
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_gender(mut self, gender: String) -> Self {
        self.gender = Some(gender);
        self
    }
}

/// Chart calculation mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChartMode {
    Natal,
    Synastry,
    Transit,
}

/// Initialize Swiss Ephemeris (call once at startup)
pub fn init_sweph() {
    unsafe {
        let ephe_path = env!("SWEPH_PATH");
        let path_cstr = CString::new(ephe_path).unwrap();
        swe_set_ephe_path(path_cstr.as_ptr());
        swe_set_sid_mode(FAGAN_BRADLEY_AYANAMSA, 0.0, 0.0);
    }
}

/// Get planet ID for Swiss Ephemeris
fn get_planet_id(body: CelestialBody) -> Option<c_int> {
    match body {
        CelestialBody::Sun => Some(0),
        CelestialBody::Moon => Some(1),
        CelestialBody::Mercury => Some(2),
        CelestialBody::Venus => Some(3),
        CelestialBody::Mars => Some(4),
        CelestialBody::Jupiter => Some(5),
        CelestialBody::Saturn => Some(6),
        CelestialBody::Uranus => Some(7),
        CelestialBody::Neptune => Some(8),
        CelestialBody::Pluto => Some(9),
        CelestialBody::TrueNode => Some(11),
        CelestialBody::Chiron => Some(15),
        _ => None, // Fortuna and Vertex calculated separately
    }
}

/// Calculate Julian Day from DateTime
fn calculate_jd(dt: &DateTime<Utc>) -> c_double {
    unsafe {
        swe_julday(
            dt.year() as c_int,
            dt.month() as c_int,
            dt.day() as c_int,
            dt.hour() as c_double
                + dt.minute() as c_double / 60.0
                + dt.second() as c_double / 3600.0,
            SE_GREG_CAL,
        )
    }
}

/// Calculate a single planet position
fn calculate_planet(jd: c_double, body: CelestialBody) -> Result<(f64, bool), String> {
    let planet_id = get_planet_id(body).ok_or("Cannot calculate this body directly")?;

    let mut xx = [0.0; 6];
    let mut serr = [0i8; 256];

    let result = unsafe {
        swe_calc_ut(
            jd,
            planet_id,
            SEFLG_SIDEREAL | SEFLG_SPEED | SEFLG_NONUT,
            xx.as_mut_ptr(),
            serr.as_mut_ptr(),
        )
    };

    if result < 0 {
        return Err(format!("Failed to calculate {}", body.to_string()));
    }

    let longitude = normalize_longitude(xx[0]);
    let speed = xx[3];

    Ok((longitude, speed < 0.0))
}

/// Calculate houses and angles
fn calculate_houses(
    jd: c_double,
    lat: f64,
    lon: f64,
) -> Result<(HouseCusps, f64, f64, f64), String> {
    let mut cusps = [0.0; 13]; // cusps[1..13] are house cusps
    let mut ascmc = [0.0; 10];
    let mut cusp_speed = [0.0; 13];
    let mut ascmc_speed = [0.0; 10];
    let mut serr = [0i8; 256];

    // Use swe_houses_ex2 with SEFLG_SIDEREAL and SEFLG_NONUT for sidereal houses
    // SEFLG_NONUT uses mean positions (no nutation) for better Fagan-Bradley precision
    let result = unsafe {
        swe_houses_ex2(
            jd,
            SEFLG_SIDEREAL | SEFLG_NONUT, // Sidereal with mean equinox
            lat,
            lon,
            PLACIDUS_HOUSE_SYSTEM as c_int,
            cusps.as_mut_ptr(),
            ascmc.as_mut_ptr(),
            cusp_speed.as_mut_ptr(),
            ascmc_speed.as_mut_ptr(),
            serr.as_mut_ptr(),
        )
    };

    if result < 0 {
        return Err("Failed to calculate houses".to_string());
    }

    // Extract house cusps (1-12) and apply calibration
    let mut house_cusps = [0.0; 12];
    for i in 0..12 {
        house_cusps[i] = normalize_longitude(cusps[i + 1]);
    }

    let asc = normalize_longitude(ascmc[0]);
    let mc = normalize_longitude(ascmc[1]);
    // Swiss Ephemeris returns Vertex directly in ascmc[3]
    // Do NOT add 180° - that gives us the anti-vertex
    let vertex = normalize_longitude(ascmc[3]);

    Ok((HouseCusps::new(house_cusps), asc, mc, vertex))
}

/// Determine if Sun is above horizon (day birth)
/// Sun in houses 7-12 = day birth (above horizon)
/// Sun in houses 1-6 = night birth (below horizon)
fn is_sun_above_horizon(sun_house: u8) -> bool {
    // Day birth if Sun is in houses 7, 8, 9, 10, 11, or 12
    sun_house >= 7
}

/// Calculate a complete natal chart
pub fn calculate_chart(input: &ChartInput) -> Result<Chart, String> {
    let jd = calculate_jd(&input.datetime);
    
    // Swiss Ephemeris expects longitude as east-positive
    // Western longitudes must be negative
    let lon_east = if input.longitude > 180.0 {
        input.longitude - 360.0
    } else if input.longitude < -180.0 {
        input.longitude + 360.0
    } else {
        input.longitude
    };
    
    let (houses, asc, mc, vertex) = calculate_houses(jd, input.latitude, lon_east)?;

    let mut chart = Chart::new()
        .with_metadata(input.name.clone(), input.gender.clone());
    chart.houses = houses;

    // Add angles
    chart.angles.push(AnglePosition {
        angle: AnglePoint::Ascendant,
        longitude: asc,
    });
    chart.angles.push(AnglePosition {
        angle: AnglePoint::Midheaven,
        longitude: mc,
    });

    // Calculate all planets
    let mut sun_lon = 0.0;
    let mut sun_house = 1u8;
    let mut moon_lon = 0.0;

    for body in CelestialBody::all() {
        if matches!(body, CelestialBody::Fortuna | CelestialBody::Vertex) {
            continue;
        }

        let (longitude, retrograde) = calculate_planet(jd, body)?;

        if matches!(body, CelestialBody::Sun) {
            sun_lon = longitude;
        }
        if matches!(body, CelestialBody::Moon) {
            moon_lon = longitude;
        }

        let house = chart.houses.get_house(longitude);
        
        if matches!(body, CelestialBody::Sun) {
            sun_house = house;
        }

        chart.positions.push(Position {
            body,
            longitude,
            retrograde,
            house,
        });
    }

    // Calculate Part of Fortune
    // Day birth (Sun above horizon): ASC + Moon - Sun
    // Night birth (Sun below horizon): ASC + Sun - Moon
    let is_day_birth = is_sun_above_horizon(sun_house);
    
    let mut fortuna_lon = if is_day_birth {
        (asc + moon_lon - sun_lon) % 360.0
    } else {
        (asc + sun_lon - moon_lon) % 360.0
    };
    
    if fortuna_lon < 0.0 {
        fortuna_lon += 360.0;
    }

    chart.positions.push(Position {
        body: CelestialBody::Fortuna,
        longitude: fortuna_lon,
        retrograde: false,
        house: chart.houses.get_house(fortuna_lon),
    });

    // Add Vertex
    chart.positions.push(Position {
        body: CelestialBody::Vertex,
        longitude: vertex,
        retrograde: false,
        house: chart.houses.get_house(vertex),
    });

    Ok(chart)
}

/// Calculate a transit chart (current positions against natal chart)
/// Returns (natal_chart, transit_chart)
/// Transit chart won't include Fortuna or Vertex
pub fn calculate_transit_chart(
    natal: &ChartInput,
    transit_time: &DateTime<Utc>,
) -> Result<(Chart, Chart), String> {
    // Calculate natal chart first
    let natal_chart = calculate_chart(natal)?;

    // Calculate transit positions using natal location and houses
    let jd = calculate_jd(transit_time);

    let mut transit_chart = Chart::new();
    transit_chart.houses = natal_chart.houses.clone(); // Use natal houses
    transit_chart.angles = natal_chart.angles.clone(); // Use natal angles

    // Calculate transiting planets (no Fortuna or Vertex in transits)
    for body in CelestialBody::all() {
        // Skip calculated points in transits
        if matches!(body, CelestialBody::Fortuna | CelestialBody::Vertex) {
            continue;
        }

        let (longitude, retrograde) = calculate_planet(jd, body)?;
        let house = natal_chart.houses.get_house(longitude);

        transit_chart.positions.push(Position {
            body,
            longitude,
            retrograde,
            house, // Which natal house the transiting planet is in
        });
    }

    Ok((natal_chart, transit_chart))
}

/// Calculate synastry charts (two people)
/// Returns (person1_chart, person2_chart)
/// Person1's planets will show which of Person2's houses they fall in, and vice versa
pub fn calculate_synastry_charts(
    person1: &ChartInput,
    person2: &ChartInput,
) -> Result<(Chart, Chart), String> {
    // Calculate both natal charts
    let mut chart1 = calculate_chart(person1)?;
    let mut chart2 = calculate_chart(person2)?;

    // Update chart1 positions to show which house of person2 they fall in
    for pos in chart1.positions.iter_mut() {
        pos.house = chart2.houses.get_house(pos.longitude);
    }

    // Update chart2 positions to show which house of person1 they fall in
    for pos in chart2.positions.iter_mut() {
        pos.house = chart1.houses.get_house(pos.longitude);
    }

    Ok((chart1, chart2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_input_creation() {
        let input = ChartInput::new(Utc::now(), 40.7128, -74.0060);
        assert!(input.latitude > 0.0);
    }

    #[test]
    fn test_jd_calculation() {
        let dt = Utc::now();
        let jd = calculate_jd(&dt);
        assert!(jd > 2400000.0); // Reasonable JD for modern dates
    }
}
