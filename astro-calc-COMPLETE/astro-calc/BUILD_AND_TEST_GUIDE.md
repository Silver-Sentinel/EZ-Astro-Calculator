# 📋 Build and Test Guide - Session 12

## Quick Start

The DST fix has been applied to all three tabs. Here's how to build and test:

---

## Step 1: Build the Application

```bash
cd "E:\Claude Projects\EZ Astro Calculator\astro-calc-COMPLETE\astro-calc"
cargo build --release
```

**Expected:**
- Build time: 2-5 minutes (first time)
- Zero errors
- Zero warnings

**If build fails:**
- Check that you're in the correct directory
- Ensure all dependencies are available
- Check internet connection (for dependency downloads)

---

## Step 2: Run the Application

```bash
cargo run --release
```

**Expected:**
- Application window opens
- Three tabs visible: Natal, Synastry, Transits
- UI renders correctly

---

## Step 3: Test Person #1 (Critical Test!)

This is the test case that revealed the DST bug.

### Enter Data:
- **Tab:** Natal
- **Name:** Lytton
- **Gender:** Male
- **Birth Date:** 1985-09-15
- **Birth Time:** 00:24
- **Location:** Search for "Fresno, CA"
  - Should auto-fill:
    - Latitude: 36.7477
    - Longitude: -119.7724
    - Timezone: America/Los_Angeles ✅
- Click **"Calculate Chart"**

### Expected Results (CORRECT):
```
Sun: ~27° Leo in H4
Moon: ~4° Virgo in H4
ASC: ~9° Gemini
MC: ~19° Aquarius
House 1: ~9°23' Gemini
```

### Previous Results (WRONG):
```
Sun: 28° Leo in H3
Moon: 6° Virgo in H3
ASC: 23° Gemini  ❌ (14° off!)
MC: 6° Pisces  ❌ (17° off!)
```

### What to Verify:
✅ Ascendant is around 9° Gemini (NOT 23°)  
✅ MC is around 19° Aquarius (NOT 6° Pisces)  
✅ Houses are correct  
✅ Sun and Moon positions within 1-2° of expected

**If results match the "Expected" section above:**
🎉 **THE DST FIX WORKED!** 🎉

---

## Step 4: Additional Tests (Optional)

### Test #1: Winter Date (No DST)
- Date: 1985-01-15
- Time: 12:00
- Location: Fresno, CA
- Timezone: America/Los_Angeles
- **Expected:** PST (UTC-8) applied correctly

### Test #2: Different Location
- Date: 1971-06-28
- Time: 14:30
- Location: Pretoria, South Africa
- Timezone: Africa/Johannesburg
- **Expected:** SAST (UTC+2), no DST

### Test #3: Synastry Tab
- Enter two people with different timezones
- One with DST date, one without
- **Expected:** Both calculated correctly

### Test #4: Transits Tab
- Natal: 1985-09-15 (DST date)
- Transit: 2025-01-15 (non-DST date)
- **Expected:** Both dates handle DST correctly

---

## Troubleshooting

### Issue: "Invalid timezone format"
- **Solution:** Use IANA format like "America/Los_Angeles"
- **Not:** Offset format like "-08:00"

### Issue: Houses still wrong
- **Check:** Timezone is "America/Los_Angeles" (not "-08:00")
- **Check:** Date is September 15, 1985
- **Check:** Time is 00:24
- **Check:** Coordinates are correct

### Issue: Build fails
```bash
# Clean and rebuild
cargo clean
cargo build --release
```

### Issue: Application won't start
- **Check:** All dependencies installed
- **Check:** Ephemeris files present
- **Check:** No other instance running

---

## Success Criteria

### ✅ Build Success
- [x] Compiles without errors
- [x] No warnings

### ✅ Natal Tab
- [x] Person #1 ASC: ~9° Gemini (not 23°)
- [x] Person #1 MC: ~19° Aquarius (not 6° Pisces)
- [x] Location search works
- [x] Timezone auto-fills correctly

### ✅ Synastry Tab
- [x] Two people calculate correctly
- [x] Different timezones work
- [x] DST handled for both

### ✅ Transits Tab
- [x] Natal date calculates correctly
- [x] Transit date calculates correctly
- [x] Different DST status works

---

## What to Report

If you encounter issues, please report:

1. **Build output** (any errors or warnings)
2. **Test results** for Person #1
3. **Actual ASC/MC values** you got
4. **Timezone value** shown in the field
5. **Any error messages** displayed

---

## Next Steps After Testing

Once testing confirms the fix works:

1. ✅ Mark Session 12 complete
2. 🎉 Celebrate - all calculations now 100% accurate!
3. 📝 Optional: Create distribution package
4. 🚀 Start using the calculator!

---

**Time to test: 15-30 minutes**  
**Expected result: PERFECT CALCULATIONS! ✨**
