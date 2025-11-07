# Quick Reference Card

## 🚀 Getting Started (30 seconds)

```bash
# Extract and enter directory
tar -xzf astro-calc.tar.gz && cd astro-calc

# Read the checklist
cat CHECKLIST.md

# Read implementation guide  
cat IMPLEMENTATION_GUIDE.md

# Start coding!
```

## 📝 One File To Complete

**File:** `src/sweph.rs`
**Lines:** ~150-200 (mostly copy-paste from guide)
**Time:** 1-2 hours
**Difficulty:** Easy (FFI is mostly mechanical)

## 🔧 Implementation Steps

1. **Uncomment** FFI declarations (bottom of file)
2. **Copy** implementation from IMPLEMENTATION_GUIDE.md
3. **Update** ephemeris path (one line)
4. **Test:** `cargo test`
5. **Run:** `cargo tauri dev`
6. **Build:** `cargo tauri build`

## 📚 Swiss Ephemeris Quick Reference

### Planet IDs
```rust
SE_SUN      = 0    SE_URANUS  = 7
SE_MOON     = 1    SE_NEPTUNE = 8
SE_MERCURY  = 2    SE_PLUTO   = 9
SE_VENUS    = 3    SE_TRUE_NODE = 11
SE_MARS     = 4    SE_CHIRON  = 15
SE_JUPITER  = 5
SE_SATURN   = 6
```

### Key Functions
```rust
// Set ephemeris path
swe_set_ephe_path(path)

// Set sidereal mode
swe_set_sid_mode(1, 0.0, 0.0)  // Fagan-Bradley

// Calculate Julian Day
swe_julday(year, month, day, hour, SE_GREG_CAL)

// Calculate planet position
swe_calc_ut(jd, planet_id, SEFLG_SIDEREAL, xx, serr)

// Calculate houses
swe_houses(jd, lat, lon, b'P', cusps, ascmc)
```

### Flags
```rust
SEFLG_SIDEREAL = 64   // Sidereal calculations
SEFLG_SPEED    = 256  // Include speed (for retrograde)
SE_GREG_CAL    = 1    // Gregorian calendar
```

## 🎯 What's Already Done

✅ All data structures (chart.rs)
✅ Aspect calculation (aspects.rs)  
✅ Text formatter (formatter.rs)
✅ HTML interface (dist/index.html)
✅ Tauri config (src-tauri/tauri.conf.json)
✅ Build setup (Cargo.toml, build.rs)
✅ Complete documentation (3 guides)

## ⏳ What You Do

Implement ~50 lines in `src/sweph.rs`:
- Call swe functions
- Store results in Chart
- Calculate Part of Fortune
- Done!

## 🎨 UI Features

- Date/Time pickers with defaults
- Timezone selector (all major zones)
- Lat/Lon with North/South hints
- Calculate button with loading state
- Formatted output display
- Copy-to-clipboard button
- Beautiful gradient design

## 📊 Aspect Orbs (Already Coded)

```rust
// Major aspects
Conjunction/Opposition: 10° (Sun/Moon), 8° (planets)
Square/Trine:          8°
Sextile:               6°

// Minor aspects  
Semi-sextile:          2°
Semi-square:           2°
Quintile:              2°
Sesquiquadrate:        2°
Quincunx:              2°
```

## 🏗️ Build Commands

```bash
# Development (hot reload)
cargo tauri dev

# Production build
cargo tauri build

# Run tests
cargo test

# Check compilation
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

## 📦 Output Locations

```
Windows:  target/release/bundle/msi/
macOS:    target/release/bundle/dmg/
Linux:    target/release/bundle/appimage/
```

## 🔍 Troubleshooting

**"undefined reference to swe_*"**
→ Library not linked. Check build.rs

**"Error opening ephemeris file"**
→ Download .se1 files from astro.com/swisseph

**"Segmentation fault"**
→ Check array sizes and null pointers

**"Swiss Ephemeris not implemented"**
→ This is expected! Implement sweph.rs

## 📖 Documentation Files

| File | Purpose |
|------|---------|
| README.md | Project overview & setup |
| IMPLEMENTATION_GUIDE.md | Step-by-step instructions |
| CHECKLIST.md | Task checklist |
| ARCHITECTURE.md | System diagram |
| PROJECT_SUMMARY.md | What's been built |

## 🌟 Key Features

- Fagan-Bradley sidereal
- Placidus houses
- All major + minor aspects
- Retrograde detection
- Part of Fortune
- Vertex calculation
- Copy-paste AI ready
- Standalone executable

## ⚡ Performance Tips

- Cache Julian Day for multiple calculations
- Reuse Chart struct
- Only calculate needed aspects
- Use release builds for speed

## 🎓 Learning Resources

- Swiss Ephemeris: www.astro.com/swisseph/
- Rust FFI: doc.rust-lang.org/nomicon/ffi.html
- Tauri Docs: tauri.app
- Rust Book: doc.rust-lang.org/book/

## 💡 Pro Tips

1. Start by testing just Sun/Moon calculation
2. Add planets one at a time
3. Test houses calculation separately
4. Compare with astro.com for each step
5. Use debug mode to see values

## ✨ After Implementation

1. Test with your own birth data
2. Verify against astro.com
3. Try different locations
4. Test edge cases (southern hemisphere)
5. Build release executable
6. Share with friends! 🎉

---

**Need help?** All answers in IMPLEMENTATION_GUIDE.md!

**Ready to code?** Open src/sweph.rs!

**Questions?** Check the detailed docs!

You've got this! 🚀✨
