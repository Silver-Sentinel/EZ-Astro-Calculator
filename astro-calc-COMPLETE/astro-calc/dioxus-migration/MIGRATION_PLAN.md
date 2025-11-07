# Migration Plan: Tauri → Dioxus Desktop

## Overview
Convert the existing Tauri app to Dioxus Desktop to eliminate JavaScript interop issues while keeping ALL the working Rust calculation code.

## What We're Keeping (99% of the code!)
✅ `src/chart.rs` - All chart data structures
✅ `src/sweph.rs` - Complete Swiss Ephemeris FFI bindings  
✅ `src/aspects.rs` - All aspect calculations
✅ `src/formatter.rs` - All output formatting
✅ All calculation logic and validation
✅ Swiss Ephemeris library and data files
✅ Location search with GeoNames API

## What We're Replacing
❌ `src/main.rs` - Replace Tauri commands with Dioxus app
❌ `dist/index.html` - Replace HTML/JS UI with Dioxus components
❌ `tauri.conf.json` - No longer needed
❌ Tauri dependencies in Cargo.toml

## Migration Steps

### Step 1: Update Cargo.toml
Remove:
- `tauri` dependencies
- `tauri-build` from build-dependencies

Add:
- `dioxus = "0.5"`
- `dioxus-desktop = "0.5"`
- `copypasta = "0.10"` (for clipboard)

Keep:
- All existing dependencies (serde, chrono, reqwest, tokio, etc.)
- The `[lib]` section (keep lib.rs as-is)
- Build dependencies for Swiss Ephemeris

### Step 2: Create New main.rs
- Remove Tauri-specific code
- Create Dioxus desktop window
- Import all existing modules (no changes needed!)
- Call calculation functions directly (no `invoke` needed)

### Step 3: Create Dioxus Components
Create these new files in `src/components/`:
- `natal.rs` - Natal chart form (replaces HTML form)
- `synastry.rs` - Synastry form
- `transits.rs` - Transits form
- `mod.rs` - Component exports

These will call your existing functions directly:
```rust
// No more invoke! Direct function calls:
let result = calculate_chart(&input)?;
let output = format_natal_chart(&result);
```

### Step 4: Handle Location Search
Keep the reqwest code, but call it directly from the UI:
```rust
// In the component:
let search_results = search_location(query).await;
```

### Step 5: Test Everything
- All calculations should work identically
- No JavaScript errors possible
- Native performance
- Same output format

## Time Estimate
- ⏱️ 30-60 minutes to update files
- ⏱️ 30 minutes testing
- ⏱️ **Total: 1-2 hours**

## Files to Create/Modify

### New Files (in dioxus-migration folder)
1. `new_main.rs` - Dioxus version of main.rs
2. `new_Cargo.toml` - Updated dependencies
3. `app.rs` - Main Dioxus app component
4. `components/natal.rs` - Natal chart UI
5. `components/synastry.rs` - Synastry UI
6. `components/transits.rs` - Transits UI
7. `components/mod.rs` - Module exports

### Files to Keep (NO CHANGES!)
- `src/lib.rs` ✅
- `src/chart.rs` ✅
- `src/sweph.rs` ✅
- `src/aspects.rs` ✅
- `src/formatter.rs` ✅
- `build.rs` ✅
- All Swiss Ephemeris files ✅

## Testing Plan
1. Build: `cargo build --release`
2. Run: `cargo run --release`
3. Test natal chart calculation
4. Test synastry calculation
5. Test transit calculation
6. Test location search
7. Test clipboard copy
8. Verify output format matches exactly

## Rollback Plan
If anything goes wrong, just keep using the Tauri version. We're not modifying your working code until we've tested the Dioxus version completely.

## Benefits of This Approach
- ✅ Eliminates JavaScript errors permanently
- ✅ Keeps all working calculation code
- ✅ Native UI performance
- ✅ Simpler debugging (one language)
- ✅ No web view overhead
- ✅ Easier maintenance

Ready to proceed?
