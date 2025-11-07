# Quick Testing Reference Card - SEFLG_NONUT Fix

## 🎯 Quick Test Instructions

1. **Compile**: `cd "E:\Claude Projects\EZ Astro Calculator\astro-calc-COMPLETE\astro-calc" && cargo build --release`

2. **Run**: `cargo run --release`

3. **Test with these exact inputs**:

---

## Test Case #1: Night Birth (Main Test Case)
**Input:**
- Date: September 15, 1985
- Time: 00:24 AM (12:24 AM)
- Location: Fresno, CA (36.7468°N, -119.7726°W)
- Timezone: -7 (PDT)

**Expected Results (Compare with Astro.com using Fagan-Bradley):**
```
BEFORE FIX → AFTER FIX (Expected)

Sun:     28° Leo → 27° Leo
Moon:    5° Virgo → 4° Virgo  
Mercury: 6° Virgo → 5° Virgo
Venus:   20° Cancer → 19° Cancer
Mars:    6° Leo → 5° Leo

Ascendant: Should shift ~1° toward correct value
All Houses: Should shift ~1° toward correct values
Part of Fortune: ~2° Gemini (should be unchanged - already fixed)
Vertex: ~27° Libra (should be unchanged - already fixed)
```

**Key Check**: Compare the Sun and Moon positions with Astro.com's Fagan-Bradley sidereal chart. They should now match within 6 arcminutes (0.1°).

---

## Test Case #2: Day Birth
**Input:**
- Date: June 28, 1971
- Time: 7:30 AM
- Location: Pretoria, South Africa (-25.7479°S, 28.2293°E)
- Timezone: +2 (SAST)

**Key Check**: All planetary positions should match Astro.com within arcminutes.

---

## Test Case #3: Day Birth (Morning)
**Input:**
- Date: January 23, 1952
- Time: 8:15 AM
- Location: Richmond, CA (37.9358°N, -122.3477°W)
- Timezone: -8 (PST)

**Key Check**: All planetary positions should match Astro.com within arcminutes.

---

## 📊 What Changed?

### Before Fix:
- Swiss Ephemeris was using **true equinox** (with nutation)
- This is correct for tropical astrology
- But wrong for Fagan-Bradley sidereal

### After Fix:  
- Swiss Ephemeris now uses **mean equinox** (without nutation)
- This is the correct reference frame for Fagan-Bradley
- Adds `SEFLG_NONUT` flag to both planet and house calculations

### Expected Shift:
- All positions shift by ~0.8-1.0 degrees
- Direction depends on date and nutation cycle
- Should now align with Astro.com and professional software

---

## ✅ Success Criteria

**The fix is working correctly if:**

1. Sun position in Test Case #1 is now ~27° Leo (not 28°)
2. Moon position in Test Case #1 is now ~4° Virgo (not 5°)
3. All positions match Astro.com within 6 arcminutes (0.1°)
4. Part of Fortune stays at ~2° Gemini (unchanged)
5. Vertex stays at ~27° Libra (unchanged)

---

## 🔧 If Something Goes Wrong

**Compilation Error?**
- Check that SEFLG_NONUT constant is defined (line 51: `pub const SEFLG_NONUT: c_int = 1024;`)
- Verify the flag is used in both lines 150 and 183

**Positions Still Off?**
- Check that you're using Fagan-Bradley on Astro.com for comparison
- Make sure timezone is correct
- Verify coordinates are accurate

**Part of Fortune or Vertex Broken?**
- These should be unchanged - previous fixes should still work
- If broken, may need to review previous session changes

---

## 📝 Notes

- This fix affects ALL calculations equally
- It's a systematic shift in reference frame
- The magnitude is ~1° but can vary slightly by date
- This is the final piece to achieve professional-grade accuracy

---

*Created: November 7, 2025*
*Purpose: Quick reference for testing SEFLG_NONUT implementation*
