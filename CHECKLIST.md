# Implementation Checklist - EZ Astro Calculator Enhancements

## 📋 Quick Reference for Tomorrow's Session

---

## Feature 1: Location Search (45 min)

### HTML Changes (dist/index.html)
- [ ] Add location search input field above lat/long
- [ ] Add dropdown container for search results
- [ ] Add "Or enter manually:" text above lat/long fields
- [ ] Style dropdown with CSS

### JavaScript Functions
- [ ] `searchLocation(query)` - API call to GeoNames
- [ ] `debounceSearch()` - 300ms delay implementation
- [ ] `displayResults(locations)` - populate dropdown
- [ ] `selectLocation(lat, lon, name)` - fill fields
- [ ] `clearDropdown()` - close results

### API Integration
- [ ] Test endpoint: `http://api.geonames.org/searchJSON?q=test&maxRows=10&username=demo`
- [ ] Add error handling for offline/failed requests
- [ ] Implement local caching (localStorage)

---

## Feature 2: Dual Input Sets (60 min)

### HTML Structure Changes
- [ ] Wrap existing inputs in "Person 1" container
- [ ] Duplicate entire input set for "Person 2"
- [ ] Add Name/Label field (text input)
- [ ] Add Gender selector (radio buttons: M/F/O)
- [ ] Ensure both sets have unique IDs

### Tab-Specific Logic
#### Natal Tab
- [ ] Show both input sets fully enabled
- [ ] Both generate separate charts

#### Synastry Tab
- [ ] Show both input sets fully enabled
- [ ] Output shows transposed houses

#### Transits Tab
- [ ] Set 1: Full natal person inputs
- [ ] Set 2: Gray out name field, show date/time only
- [ ] Add label "Transit Date" for set 2

### CSS Styling
- [ ] Style input containers vertically
- [ ] Add borders/spacing between sets
- [ ] Style grayed-out fields (.disabled)

---

## Feature 3: New Layout (60 min)

### HTML Layout Restructure
- [ ] Create flexbox container (left: inputs, right: output)
- [ ] Left panel: 40% width, scrollable
- [ ] Right panel: 60% width
- [ ] Add `<textarea>` for output (readonly, resizable)
- [ ] Add "Copy to Clipboard" button below textarea

### CSS Layout
```css
.container {
  display: flex;
  height: 100vh;
}

.input-panel {
  width: 40%;
  overflow-y: auto;
  padding: 24px;
  background: #f5f5f5;
}

.output-panel {
  width: 60%;
  padding: 24px;
  background: #ffffff;
}

#outputText {
  width: 100%;
  height: calc(100% - 60px);
  resize: both;
  min-height: 400px;
  min-width: 300px;
  font-family: 'Courier New', monospace;
  font-size: 13px;
  padding: 12px;
  border: 1px solid #e5e7eb;
}
```

### JavaScript
- [ ] `updateOutput(chartData)` - populate textarea
- [ ] `copyToClipboard()` - copy textarea content
- [ ] Test clipboard API in Tauri

---

## Feature 4: Backend Changes (90 min)

### New Rust Structs
```rust
struct PersonInput {
    name: String,
    gender: Gender, // enum: Male, Female, Other
    birth_date: String,
    birth_time: String,
    timezone: String,
    latitude: f64,
    longitude: f64,
}

enum Gender {
    Male,
    Female,
    Other,
}
```

### New Calculations (src/chart.rs)
- [ ] Add `calculate_fortuna()` function
  ```rust
  // Formula: ASC + Moon - Sun (day birth)
  // Formula: ASC + Sun - Moon (night birth)
  ```

- [ ] Add `calculate_vertex()` function
  ```rust
  // Vertex = MC + RAMC calculation
  // Uses geographic latitude
  ```

### Synastry Function (src/chart.rs)
- [ ] Create `calculate_synastry(person1: Chart, person2: Chart) -> SynastryOutput`
- [ ] Calculate Person 1's planets in Person 2's houses
- [ ] Calculate Person 2's planets in Person 1's houses
- [ ] Find inter-chart aspects

### Transit Function (src/chart.rs)
- [ ] Create `calculate_transits(natal: Chart, transit_date: JulianDay) -> TransitOutput`
- [ ] Calculate current planetary positions
- [ ] Compare to natal chart
- [ ] Exclude Fortuna, Vertex, AC, MC from transiting objects
- [ ] Include those points in natal chart only

### Update Formatter (src/formatter.rs)
- [ ] Add format for dual natal charts
- [ ] Add format for synastry (with house overlays)
- [ ] Add format for transits (natal + current positions)
- [ ] Update line wrapping for readability

---

## Testing Checklist (30 min)

### Location Search
- [ ] Search works online
- [ ] Falls back to manual offline
- [ ] Results populate correctly
- [ ] Coordinates auto-fill
- [ ] Manual entry still works

### Dual Inputs
- [ ] Both input sets collect data
- [ ] Natal tab: two separate charts
- [ ] Synastry tab: transposed houses
- [ ] Transits tab: name field grayed out correctly

### Layout
- [ ] Textarea resizes properly
- [ ] Copy button works
- [ ] Can select partial text
- [ ] Layout responsive
- [ ] No scrolling issues

### Calculations
- [ ] Fortuna calculates correctly
- [ ] Vertex calculates correctly
- [ ] Synastry shows correct house overlays
- [ ] Transits exclude angles/points
- [ ] All aspects appear correctly

### Edge Cases
- [ ] Empty fields handled gracefully
- [ ] Invalid dates rejected
- [ ] API timeout handled
- [ ] Very long names display correctly
- [ ] Special characters in names

---

## File Modifications Summary

### Files to Modify:
1. `dist/index.html` - Major UI restructure
2. `src/chart.rs` - Add Fortuna, Vertex, synastry, transits
3. `src/formatter.rs` - Update output formatting
4. `src/lib.rs` - Export new functions
5. `src/main.rs` - Handle dual inputs from frontend

### Files to Create:
- None (all changes to existing files)

### Dependencies to Add:
- None (GeoNames API is REST, no new Rust deps needed)

---

## Build & Deploy

- [ ] Test in dev mode: `cargo tauri dev`
- [ ] Fix any compilation errors
- [ ] Test all features in dev
- [ ] Build release: `cargo tauri build`
- [ ] Test release executable
- [ ] Copy to convenient location

---

## Priority Order

1. **Backend calculations** (Fortuna, Vertex, Synastry, Transits) - Must work first
2. **Dual input fields** - Core functionality
3. **New layout** - Better UX
4. **Location search** - Nice-to-have enhancement

If time is limited, implement in this order and save location search for last.

---

## Notes for Tomorrow

- Start with backend (Rust) first - it's the foundation
- Test calculations in dev mode before UI work
- UI changes are mostly HTML/CSS - faster to implement
- Location search can be added incrementally
- Keep original lat/long fields visible at all times
- Remember: Synastry = TRANSPOSED houses (key requirement)
- Transits: planets only, no angles as transiting objects

---

Ready to implement! See IMPLEMENTATION_PLAN.md for detailed specifications. 🚀
