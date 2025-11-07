# Astrological Chart Calculator

A standalone desktop application for calculating Fagan-Bradley sidereal astrological charts with Placidus houses.

## Features

- 🌟 **Fagan-Bradley Sidereal** calculations
- 🏠 **Placidus House System**
- 📊 **Complete aspect analysis** (major and minor aspects)
- 📋 **Copy-paste friendly output** for AI prompts
- 💻 **Standalone executable** (no installation required)
- 🎨 **Clean, modern interface**

## Aspect Orbs (Astro.com Standards)

**Major Aspects:**
- Conjunction/Opposition: 10° (Sun/Moon), 8° (planets)
- Square/Trine: 8°
- Sextile: 6°

**Minor Aspects:**
- Semi-sextile (30°): 2°
- Semi-square (45°): 2°
- Quintile (72°): 2°
- Sesquiquadrate (135°): 2°
- Quincunx (150°): 2°

## Setup Instructions

### Prerequisites

1. **Rust** - Install from https://rustup.rs/
2. **Node.js** (optional, for Tauri CLI) - https://nodejs.org/
3. **Swiss Ephemeris Library** - Download from https://www.astro.com/swisseph/

### Swiss Ephemeris Setup

1. Download the Swiss Ephemeris C library:
   - Linux: `libswe.so`
   - Windows: `swe.dll`
   - macOS: `libswe.dylib`

2. Download ephemeris data files (at minimum):
   - `seas_18.se1` (main planets)
   - `semo_18.se1` (Moon)
   - `sepl_18.se1` (outer planets)

3. Place the library and data files in your project directory or system path

### Completing the Implementation

**YOU NEED TO COMPLETE:** The Swiss Ephemeris integration in `src/sweph.rs`

The file has detailed TODO comments showing exactly what to implement:

```rust
// 1. Uncomment the FFI declarations at the bottom
// 2. Implement calculate_chart() function:
//    - Call swe_set_ephe_path()
//    - Call swe_set_sid_mode(FAGAN_BRADLEY_AYANAMSA, 0, 0)
//    - Calculate Julian Day with swe_julday()
//    - Call swe_calc_ut() for each planet
//    - Call swe_houses() with 'P' for Placidus
//    - Calculate Part of Fortune: ASC + Moon - Sun
//    - Calculate Vertex from houses
```

### Building and Running

#### Development Mode

```bash
# Install Tauri CLI (first time only)
cargo install tauri-cli

# Run in development
cargo tauri dev
```

#### Build Standalone Executable

```bash
# Build for release
cargo tauri build

# The executable will be in: target/release/bundle/
# Windows: .exe in msi or nsis folder
# macOS: .app or .dmg in dmg folder
# Linux: AppImage or .deb in appimage or deb folder
```

## Project Structure

```
astro-calc/
├── src/
│   ├── chart.rs           # Data structures for planets, signs, houses
│   ├── aspects.rs         # Aspect calculation engine
│   ├── formatter.rs       # Text output formatter
│   ├── sweph.rs          # Swiss Ephemeris integration (TODO: implement)
│   ├── lib.rs            # Module exports
│   └── main.rs           # Tauri entry point
├── dist/
│   └── index.html        # HTML interface
├── src-tauri/
│   └── tauri.conf.json   # Tauri configuration
├── Cargo.toml            # Rust dependencies
└── README.md             # This file
```

## Swiss Ephemeris Integration Guide

### Example: Calculating a Planet Position

```rust
use std::ffi::CString;

// Set ephemeris path
let path = CString::new("/path/to/ephe/data").unwrap();
unsafe { swe_set_ephe_path(path.as_ptr()); }

// Set sidereal mode (Fagan-Bradley)
unsafe { swe_set_sid_mode(1, 0.0, 0.0); }

// Calculate Julian Day
let jd = unsafe {
    swe_julday(year, month, day, hour, SE_GREG_CAL)
};

// Calculate planet
let mut xx = [0.0; 6];
let mut serr = [0i8; 256];
let result = unsafe {
    swe_calc_ut(
        jd,
        SE_SUN,  // or other planet ID
        SEFLG_SIDEREAL | SEFLG_SPEED,
        xx.as_mut_ptr(),
        serr.as_mut_ptr()
    )
};

if result >= 0 {
    let longitude = xx[0];  // Sidereal longitude
    let speed = xx[3];
    let retrograde = speed < 0.0;
}
```

### Swiss Ephemeris Planet IDs

```
SE_SUN = 0
SE_MOON = 1
SE_MERCURY = 2
SE_VENUS = 3
SE_MARS = 4
SE_JUPITER = 5
SE_SATURN = 6
SE_URANUS = 7
SE_NEPTUNE = 8
SE_PLUTO = 9
SE_TRUE_NODE = 11
SE_CHIRON = 15
```

### Calculating Houses

```rust
let mut cusps = [0.0; 13];  // cusps[1..13] are house cusps
let mut ascmc = [0.0; 10];
unsafe {
    swe_houses(
        jd,
        latitude,
        longitude,
        b'P' as i32,  // Placidus
        cusps.as_mut_ptr(),
        ascmc.as_mut_ptr()
    )
};

let ascendant = ascmc[0];
let mc = ascmc[1];
let vertex = ascmc[3];
```

### Calculating Part of Fortune

```rust
// Part of Fortune = ASC + Moon - Sun (diurnal chart)
let fortuna = (ascendant + moon_lon - sun_lon) % 360.0;
if fortuna < 0.0 {
    fortuna += 360.0;
}
```

## Output Format

The application outputs chart data in this format:

```
Sun 11 Gemini H12; sextile Moon, conjunct Mercury and AC, square Uranus.
Moon 13 Leo H2; sextile Sun, square Saturn, trine MC.
Mercury 19 Gemini H1; conjunct Sun and AC, sextile Moon.
...

House #s > Degree > Signs Respective to House #s:

House 1/7 19 Gemini/Sagittarius
House 2/8 24 Cancer/Capricorn
...
```

This format is optimized for:
- Easy reading
- Copy-pasting into AI prompts
- Quick aspect analysis

## Testing

Run tests with:

```bash
cargo test
```

Tests are included for:
- Angular separation calculations
- Aspect detection
- Position formatting
- Sign calculations

## Troubleshooting

### "Swiss Ephemeris integration not yet implemented"

This is expected! You need to implement the `calculate_chart()` function in `src/sweph.rs` following the guide above.

### "Cannot find libswe"

Make sure the Swiss Ephemeris library is:
1. In your system library path, OR
2. In the project directory, OR
3. Linked via `build.rs` or `Cargo.toml`

### Build Errors

If you get compilation errors:
1. Make sure Rust is up to date: `rustup update`
2. Clean and rebuild: `cargo clean && cargo build`
3. Check that all dependencies are installed

## License

This project uses the Swiss Ephemeris library, which is dual-licensed:
- GPL for non-commercial use
- Commercial license available from Astrodienst AG

See https://www.astro.com/swisseph/ for details.

## Next Steps

1. ✅ Project structure created
2. ✅ Core calculation engine built
3. ✅ Aspect detection implemented
4. ✅ Text formatter completed
5. ✅ HTML interface designed
6. ⏳ **YOU NEED TO DO:** Implement Swiss Ephemeris integration in `src/sweph.rs`
7. ⏳ Test with real birth data
8. ⏳ Build standalone executable

Happy calculating! 🌟
