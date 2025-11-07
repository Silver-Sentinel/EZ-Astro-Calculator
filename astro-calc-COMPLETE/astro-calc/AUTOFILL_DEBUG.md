# 🐛 Auto-Fill Debugging Guide

## Quick Test: Is the dropdown working AT ALL?

1. Open the app
2. Open Developer Console (F12 or Ctrl+Shift+I)
3. Type in the location search field: "Los Angeles"
4. Watch the console for messages

## Expected Console Messages:

**If working:**
```
Location search results: [Array of locations]
Auto-filling with: Los Angeles, CA, United States
```

**If API fails:**
```
Location search failed: [error message]
```

## Common Issues & Fixes:

### Issue #1: GeoNames API Rate Limited (Most Likely)
**Symptom:** Dropdown says "No results found" OR no dropdown appears at all

**Fix:** Register free GeoNames account (30 seconds):
1. Go to: https://www.geonames.org/login
2. Create free account
3. Enable "Free Web Services" in account settings
4. Open `src/main.rs` in your editor
5. Find line ~67: `username=demo`
6. Replace with: `username=YOUR_USERNAME`
7. Recompile: `cargo tauri dev`

### Issue #2: Text Selection Not Working
**Symptom:** Dropdown appears with results, but typing doesn't auto-fill

**Workaround:** This is a known limitation in some Tauri builds. The dropdown still works - just click a result instead of using auto-fill.

### Issue #3: Network/CORS Issues
**Symptom:** Console shows CORS or network errors

**Fix:** Check `tauri.conf.json` has proper remote access configured (should already be set up).

---

## Quick Fix: Force Recompile

Sometimes the dist/index.html changes don't get picked up. Force a clean rebuild:

```bash
cd "E:\Claude Projects\EZ Astro Calculator\astro-calc-COMPLETE\astro-calc"
cargo clean
cargo tauri dev
```

---

## Test With Console Logging Enabled

I'll create an enhanced debug version with detailed logging...
