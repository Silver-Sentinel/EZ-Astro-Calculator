# Swiss Ephemeris Implementation Guide

This guide will walk you through completing the Swiss Ephemeris integration when you're back at your PC.

## Step 1: Link the Swiss Ephemeris Library

### Option A: Using build.rs (Recommended)

Add to your `Cargo.toml`:

```toml
[build-dependencies]
cc = "1.0"
```

Update `build.rs`:

```rust
fn main() {
    // Link Swiss Ephemeris
    println!("cargo:rustc-link-search=native=/path/to/swisseph/lib");
    println!("cargo:rustc-link-lib=swe");
    
    tauri_build::build()
}
```

### Option B: System-wide Installation

**Linux:**
```bash
sudo cp libswe.so /usr/local/lib/
sudo ldconfig
```

**macOS:**
```bash
sudo cp libswe.dylib /usr/local/lib/
```

**Windows:**
Place `swe.dll` in the same directory as your executable or in System32.

## Step 2: Uncomment FFI Declarations in src/sweph.rs

Find these lines at the bottom of `src/sweph.rs` and uncomment them:

```rust
use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int};

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
}

pub const SEFLG_SIDEREAL: c_int = 64;
pub const SEFLG_SPEED: c_int = 256;
pub const SE_GREG_CAL: c_int = 1;
```

## Step 3: Implement calculate_chart()

Replace the stub implementation with this working code:

```rust
use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int};

pub fn calculate_chart(input: &ChartInput) -> Result<Chart, String> {
    let mut chart = Chart::new();
    
    // Set ephemeris path - UPDATE THIS PATH
    let ephe_path = CString::new("/path/to/your/ephemeris/data").unwrap();
    unsafe {
        swe_set_ephe_path(ephe_path.as_ptr());
    }
    
    // Set Fagan-Bradley sidereal mode
    unsafe {
        swe_set_sid_mode(FAGAN_BRADLEY_AYANAMSA, 0.0, 0.0);
    }
    
    // Convert DateTime to components
    let year = input.datetime.year();
    let month = input.datetime.month() as c_int;
    let day = input.datetime.day() as c_int;
    let hour = input.datetime.hour() as f64
        + input.datetime.minute() as f64 / 60.0
        + input.datetime.second() as f64 / 3600.0;
    
    // Calculate Julian Day
    let jd = unsafe {
        swe_julday(year, month, day, hour, SE_GREG_CAL)
    };
    
    // Calculate houses first (we need ASC and MC)
    let mut cusps = [0.0; 13];
    let mut ascmc = [0.0; 10];
    let house_result = unsafe {
        swe_houses(
            jd,
            input.latitude,
            input.longitude,
            b'P' as c_int,  // Placidus
            cusps.as_mut_ptr(),
            ascmc.as_mut_ptr(),
        )
    };
    
    if house_result < 0 {
        return Err("Failed to calculate houses".to_string());
    }
    
    // Store house cusps (skip index 0, use 1-12)
    let mut house_cusps = [0.0; 12];
    for i in 0..12 {
        house_cusps[i] = cusps[i + 1];
    }
    chart.houses = HouseCusps::new(house_cusps);
    
    // Store angles
    let asc = ascmc[0];
    let mc = ascmc[1];
    let vertex = ascmc[3];
    
    chart.angles.push(AnglePosition {
        angle: AnglePoint::Ascendant,
        longitude: asc,
    });
    chart.angles.push(AnglePosition {
        angle: AnglePoint::Midheaven,
        longitude: mc,
    });
    
    // Calculate each planet
    let mut sun_lon = 0.0;
    let mut moon_lon = 0.0;
    
    for body in CelestialBody::all() {
        // Skip special points that need separate calculation
        if matches!(body, CelestialBody::Fortuna | CelestialBody::Vertex) {
            continue;
        }
        
        let planet_id = match body {
            CelestialBody::Sun => 0,
            CelestialBody::Moon => 1,
            CelestialBody::Mercury => 2,
            CelestialBody::Venus => 3,
            CelestialBody::Mars => 4,
            CelestialBody::Jupiter => 5,
            CelestialBody::Saturn => 6,
            CelestialBody::Uranus => 7,
            CelestialBody::Neptune => 8,
            CelestialBody::Pluto => 9,
            CelestialBody::TrueNode => 11,
            CelestialBody::Chiron => 15,
            _ => continue,
        };
        
        let mut xx = [0.0; 6];
        let mut serr = [0i8; 256];
        
        let result = unsafe {
            swe_calc_ut(
                jd,
                planet_id,
                SEFLG_SIDEREAL | SEFLG_SPEED,
                xx.as_mut_ptr(),
                serr.as_mut_ptr(),
            )
        };
        
        if result < 0 {
            return Err(format!("Failed to calculate {}", body.to_string()));
        }
        
        let longitude = xx[0];
        let speed = xx[3];
        
        // Store Sun and Moon for Fortuna calculation
        if matches!(body, CelestialBody::Sun) {
            sun_lon = longitude;
        }
        if matches!(body, CelestialBody::Moon) {
            moon_lon = longitude;
        }
        
        let house = chart.houses.get_house(longitude);
        
        chart.positions.push(Position {
            body,
            longitude,
            retrograde: speed < 0.0,
            house,
        });
    }
    
    // Calculate Part of Fortune: ASC + Moon - Sun
    let mut fortuna_lon = (asc + moon_lon - sun_lon) % 360.0;
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
```

## Step 4: Test Your Implementation

Create a simple test file `tests/integration_test.rs`:

```rust
use astro_calc::*;
use chrono::Utc;

#[test]
fn test_chart_calculation() {
    let input = ChartInput {
        datetime: Utc::now(),
        latitude: 40.7128,   // New York
        longitude: -74.0060,
    };
    
    let result = calculate_chart(&input);
    assert!(result.is_ok(), "Chart calculation should succeed");
    
    let chart = result.unwrap();
    assert!(!chart.positions.is_empty(), "Should have planet positions");
    assert!(!chart.angles.is_empty(), "Should have angle positions");
}
```

Run the test:

```bash
cargo test
```

## Step 5: Handle Ephemeris Path Configuration

For better user experience, allow the ephemeris path to be configured:

### Option A: Environment Variable

```rust
use std::env;

let ephe_path = env::var("SWEPH_PATH")
    .unwrap_or_else(|_| "/usr/local/share/sweph".to_string());
let path_cstr = CString::new(ephe_path).unwrap();
unsafe {
    swe_set_ephe_path(path_cstr.as_ptr());
}
```

### Option B: Configuration File

Create a `config.toml`:

```toml
[ephemeris]
path = "/path/to/ephemeris/data"
```

Then use the `toml` crate to load it:

```toml
# In Cargo.toml
[dependencies]
toml = "0.8"
```

```rust
use std::fs;

let config_str = fs::read_to_string("config.toml")?;
let config: Config = toml::from_str(&config_str)?;
```

## Step 6: Error Handling Improvements

Add better error messages:

```rust
if result < 0 {
    let error_msg = unsafe {
        let slice = std::slice::from_raw_parts(
            serr.as_ptr() as *const u8,
            serr.iter().position(|&c| c == 0).unwrap_or(255)
        );
        String::from_utf8_lossy(slice).to_string()
    };
    return Err(format!("Swiss Ephemeris error: {}", error_msg));
}
```

## Step 7: Build and Test the Full Application

```bash
# Test calculations
cargo test

# Run in development mode
cargo tauri dev

# Build release executable
cargo tauri build
```

## Troubleshooting

### "undefined reference to swe_*"

The Swiss Ephemeris library isn't linked properly. Check your `build.rs` or library path.

### "Error opening ephemeris file"

The ephemeris data files aren't in the path you specified. Download them from:
https://www.astro.com/ftp/swisseph/ephe/

You need at minimum:
- `seas_18.se1`
- `semo_18.se1`
- `sepl_18.se1`

### Segmentation Fault

Usually caused by:
1. Null pointer passed to C functions
2. Buffer sizes too small
3. Uninitialized arrays

Double-check all array initializations and CString creation.

## Resources

- Swiss Ephemeris Documentation: https://www.astro.com/swisseph/swephprg.htm
- Swiss Ephemeris Source: https://github.com/aloistr/swisseph
- Rust FFI Guide: https://doc.rust-lang.org/nomicon/ffi.html

## Next Steps After Implementation

1. Test with known birth data and compare with astro.com
2. Add more detailed error messages
3. Consider adding configuration UI for ephemeris path
4. Add data validation (lat/lon ranges, etc.)
5. Implement caching for better performance
6. Add unit tests for edge cases

Good luck with the implementation! 🚀
