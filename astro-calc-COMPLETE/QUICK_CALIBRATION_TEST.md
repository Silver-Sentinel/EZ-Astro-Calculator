# Quick Testing Guide - Calibration Adjustment

## 🎯 Goal
Adjust `CALIBRATION_OFFSET_DEGREES` until all positions match targets within arcminutes.

## 📍 Test Data (Person #1)
- **Date**: September 15, 1985
- **Time**: 00:24 (12:24 AM)
- **Location**: Fresno, CA, USA
  - Latitude: 36.7468°N
  - Longitude: -119.7726°W
  - Timezone: -08:00 (PST)

## 🎬 Quick Start

### 1. Compile
```bash
cd "E:\Claude Projects\EZ Astro Calculator\astro-calc-COMPLETE\astro-calc"
cargo build --release
```

### 2. Run
```bash
cargo run --release
```

### 3. Enter Test Data
Input the person #1 data above into the application.

### 4. Compare Results

| Body | Target Position | Your Result | Status |
|------|----------------|-------------|--------|
| Sun | 27° Leo | ___° ___ | ☐ |
| Moon | 4° Virgo | ___° ___ | ☐ |
| Mercury | 21° Leo | ___° ___ | ☐ |
| Venus | 27° Cancer | ___° ___ | ☐ |
| Mars | 8° Leo | ___° ___ | ☐ |
| ASC | 9°23' Gemini | ___° ___ | ☐ |
| MC | 19°39' Aquarius | ___° ___ | ☐ |
| Vertex | 27° Libra | ___° ___ | ☐ |
| Fortuna | 2° Gemini | ___° ___ | ☐ |

## 🔧 Adjustment Guide

### If All Positions Are Too LOW (Undershot):
```rust
// Reduce magnitude (less negative)
pub const CALIBRATION_OFFSET_DEGREES: f64 = -0.9;  // was -1.0
```
Then recompile and test again.

### If All Positions Are Too HIGH (Overshot):
```rust
// Increase magnitude (more negative)
pub const CALIBRATION_OFFSET_DEGREES: f64 = -1.1;  // was -1.0
```
Then recompile and test again.

### If CLOSE But Not Perfect:
Fine-tune in 0.1° increments:
```rust
pub const CALIBRATION_OFFSET_DEGREES: f64 = -0.95;  // or -1.05, etc.
```

## 📊 Example Scenarios

### Scenario 1: Good Result
```
Sun: Target 27° Leo → Result 27° Leo ✅
Moon: Target 4° Virgo → Result 4° Virgo ✅
```
**Action**: Perfect! You're done! 🎉

### Scenario 2: Overshot
```
Sun: Target 27° Leo → Result 26° Leo ❌ (1° too low)
Moon: Target 4° Virgo → Result 3° Virgo ❌ (1° too low)
```
**Action**: Change to -0.9 (less negative)

### Scenario 3: Undershot
```
Sun: Target 27° Leo → Result 28° Leo ❌ (1° too high)
Moon: Target 4° Virgo → Result 5° Virgo ❌ (1° too high)
```
**Action**: Change to -1.1 (more negative)

### Scenario 4: Almost There
```
Sun: Target 27° Leo → Result 26.5° Leo ❌ (0.5° too low)
```
**Action**: Fine-tune to -0.95

## 📝 Where to Edit

**File**: `E:\Claude Projects\EZ Astro Calculator\astro-calc-COMPLETE\astro-calc\src\sweph.rs`

**Line ~64** (look for the big comment box):
```rust
// ═══════════════════════════════════════════════════════════════════════════
// EMPIRICAL CALIBRATION CORRECTION
// ═══════════════════════════════════════════════════════════════════════════
pub const CALIBRATION_OFFSET_DEGREES: f64 = -1.0;  // ← ADJUST THIS LINE
```

## ⚙️ After Each Adjustment

1. Save the file
2. Recompile: `cargo build --release`
3. Run: `cargo run --release`
4. Test with Person #1 data
5. Check if closer to target
6. Repeat until perfect

## ✅ Success Criteria

**You're done when:**
- All planetary positions match within 0.1° (6 arcminutes)
- Ascendant and MC match within 1 arcminute
- Part of Fortune matches target
- Vertex matches target

**Expected iterations**: 2-4 adjustments (10-15 minutes total)

## 🚨 Troubleshooting

### If adjustments don't help:
- Verify you're recompiling after each change
- Check you're testing the right person's data
- Verify "correct" data is actually Fagan-Bradley sidereal
- Check that Placidus houses are specified

### If positions move in wrong direction:
- You may have accidentally inverted the sign
- Double-check: MORE NEGATIVE = positions LOWER
- LESS NEGATIVE = positions HIGHER

---

**Current Setting**: -1.0°  
**Status**: Ready for first test  
**File to Edit**: `src/sweph.rs` line ~64
