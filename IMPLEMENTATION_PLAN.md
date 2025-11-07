# EZ Astro Calculator - Enhancement Implementation Plan

## Overview
This document outlines the implementation plan for adding missing features to the EZ Astro Calculator application.

---

## 🎯 Feature 1: Location Lookup with Auto-Complete

### Implementation Strategy
**Option A: GeoNames API (Recommended)**
- Free, open-source geographical database
- No API key required for basic usage
- Provides city names with lat/long coordinates
- Works offline-first: falls back to manual entry

### Technical Approach
1. **Add city search input field** above lat/long fields
2. **Implement debounced search** (300ms delay after typing stops)
3. **Display dropdown with results** showing "City, State/Province, Country"
4. **Auto-fill lat/long** when user selects a location
5. **Keep manual lat/long fields** always visible and editable
6. **Store common locations** in local cache for offline use

### API Endpoint
```
http://api.geonames.org/searchJSON?q={query}&maxRows=10&username=demo
```

### UI Changes
```
Location
[Search: Type city name...        ] 🔍
  └─ Dropdown: Results here
  
Or enter coordinates manually:
Latitude:  [40.7128]  North is positive, South is negative
Longitude: [-74.0060] East is positive, West is negative
```

### Fallback Strategy
- If API fails or offline: Show message "Enter coordinates manually below"
- Always allow manual override of auto-filled values

---

## 🎯 Feature 2: Dual Input Sets for All Three Tabs

### Current State
- Only one set of inputs visible
- No way to enter second person/date information

### Required Changes by Tab

#### 2.1 Natal Chart Tab
**Purpose:** Generate two separate natal charts side-by-side

**Input Fields (×2 sets):**
```
Person 1                          Person 2
─────────────────────────────    ─────────────────────────────
Name/Label: [           ]         Name/Label: [           ]
Gender: ○ Male ○ Female ○ Other   Gender: ○ Male ○ Female ○ Other

Birth Date: [mm/dd/yyyy] 📅       Birth Date: [mm/dd/yyyy] 📅
Birth Time: [--:-- --] 🕐         Birth Time: [--:-- --] 🕐
Timezone: [UTC+00:00 (GMT)] ▼     Timezone: [UTC+00:00 (GMT)] ▼

Location: [Search city...] 🔍     Location: [Search city...] 🔍
Latitude: [        ]              Latitude: [        ]
Longitude: [        ]             Longitude: [        ]
```

**Output:** Two complete natal charts displayed side-by-side in output panel

---

#### 2.2 Synastry Tab
**Purpose:** Compare two people's charts with transposed houses

**Input Fields (×2 sets):**
Same structure as Natal tab (Name, Gender, Birth Date/Time/Location)

**Key Differences in Output:**
- Person 1's chart shows: "Person 1's [Planet] in Person 2's [House]"
- Person 2's chart shows: "Person 2's [Planet] in Person 1's [House]"
- Show aspects between Person 1 and Person 2's planets
- Display both charts with house overlays

**Example Output Format:**
```
Person 1 (John) → Person 2 (Jane)
─────────────────────────────────
John's Sun 15° Aries in Jane's 3rd house
John's Moon 22° Cancer in Jane's 7th house; conjunct Jane's Venus
...

Person 2 (Jane) → Person 1 (John)
─────────────────────────────────
Jane's Sun 8° Libra in John's 10th house
Jane's Moon 5° Capricorn in John's 1st house; trine John's Sun
...
```

---

#### 2.3 Transits Tab
**Purpose:** Show current/future planetary positions relative to natal chart

**Input Fields:**
```
Natal Person                      Transit Date
─────────────────────────────    ─────────────────────────────
Name/Label: [           ]         [Name field GRAYED OUT]
Gender: ○ Male ○ Female ○ Other   Date: [mm/dd/yyyy] 📅
                                  Time: [--:-- --] 🕐
Birth Date: [mm/dd/yyyy] 📅       Timezone: [UTC+00:00] ▼
Birth Time: [--:-- --] 🕐
Timezone: [UTC+00:00 (GMT)] ▼

Location: [Search city...] 🔍
Latitude: [        ]
Longitude: [        ]
```

**Special Requirements:**
- **Natal chart includes:** All planets + Fortuna + Vertex + AC + MC
- **Transit objects:** Only planets (Sun through Pluto)
- **Exclude from transits:** Fortuna, Vertex, AC, MC (these don't transit)

**Output Format:**
```
Natal Chart (John)
─────────────────
Sun 15° Aries in 3rd house
Moon 22° Cancer in 7th house
...
Fortuna 10° Taurus in 4th house
Vertex 18° Scorpio in 11th house
AC 5° Aquarius
MC 12° Scorpio

Transits for 10/30/2025
─────────────────────────
Transiting Sun 6° Scorpio in John's 11th house; trine natal Moon
Transiting Moon 14° Pisces in John's 3rd house; square natal Sun
...
[No Fortuna, Vertex, AC, MC in transit section]
```

---

## 🎯 Feature 3: New Layout with Output Panel

### Current Layout Issue
- Output appears below inputs (requires scrolling)
- Can't see inputs while viewing output
- No easy way to copy partial results

### New Layout Design

```
┌────────────────────────────────────────────────────────────────┐
│  [📊 Natal Chart] [💑 Synastry] [🔄 Transits]                  │
├──────────────────────────┬─────────────────────────────────────┤
│  INPUT PANEL (Left)      │  OUTPUT PANEL (Right)               │
│  ─────────────────       │  ─────────────────────              │
│                          │                                     │
│  Person 1 / Event 1      │  ┌─────────────────────────────┐   │
│  ├ Name: [        ]      │  │                             │   │
│  ├ Gender: ○ M ○ F ○ O   │  │  Chart output displays here │   │
│  ├ Birth Date: [  ] 📅   │  │                             │   │
│  ├ Birth Time: [  ] 🕐   │  │  (Resizable text area)      │   │
│  ├ Timezone: [GMT]  ▼    │  │                             │   │
│  ├ Location: [    ] 🔍   │  │  User can select/copy       │   │
│  ├ Latitude: [    ]      │  │  any portion of text        │   │
│  └ Longitude: [    ]     │  │                             │   │
│                          │  └─────────────────────────────┘   │
│  Person 2 / Event 2      │  [📋 Copy to Clipboard]            │
│  ├ Name: [        ]      │                                     │
│  ├ Gender: ○ M ○ F ○ O   │                                     │
│  └ ... (same fields)     │                                     │
│                          │                                     │
│  [Calculate Chart]       │                                     │
└──────────────────────────┴─────────────────────────────────────┘
```

### Layout Specifications

**Left Panel (Input):**
- Width: 40% of window
- Fixed width, scrollable if needed
- Vertically stacked input sets
- Calculate button at bottom

**Right Panel (Output):**
- Width: 60% of window
- Contains resizable textarea
- Textarea has scroll bars
- Text is selectable for partial copying
- "Copy to Clipboard" button below textarea

**Resizing Behavior:**
- Textarea can be resized by dragging corners/edges
- Maintains minimum size (300x400px)
- Grows with window size
- Horizontal splitter between left/right panels (optional enhancement)

---

## 🛠️ Implementation Steps

### Phase 1: Backend Changes (Rust)
1. **Add gender field** to person struct
2. **Add Fortuna and Vertex calculations** to chart module
3. **Create synastry calculation function** (house overlays)
4. **Create transit calculation function** (exclude angles)
5. **Update formatter** to handle dual charts and synastry format

### Phase 2: Frontend Changes (HTML/CSS/JS)
1. **Duplicate input fields** in HTML
2. **Add location search functionality** with GeoNames API
3. **Implement new layout** with flexbox/grid
4. **Add resizable textarea** using CSS resize property
5. **Add copy to clipboard** button with JavaScript
6. **Update tab switching logic** to show/hide appropriate fields
7. **Add styling** for grayed-out fields on Transits tab

### Phase 3: Integration
1. **Connect dual inputs** to Rust backend
2. **Parse and validate** both input sets
3. **Format output** appropriately for each tab type
4. **Test all three tabs** with various inputs
5. **Handle edge cases** (missing data, API failures, etc.)

### Phase 4: Testing & Polish
1. **Test location search** (online and offline)
2. **Verify synastry calculations** (house overlays)
3. **Verify transit exclusions** (no Fortuna/Vertex/AC/MC as transits)
4. **Test resizing behavior** across different screen sizes
5. **Test clipboard functionality** on Windows
6. **Polish UI/UX** based on testing feedback

---

## 📋 Technical Details

### Backend API Changes

**New Rust Functions Needed:**
```rust
// Calculate Fortuna (Part of Fortune)
pub fn calculate_fortuna(asc: f64, sun: f64, moon: f64, is_day_birth: bool) -> f64

// Calculate Vertex
pub fn calculate_vertex(lat: f64, ramc: f64) -> f64

// Synastry house overlay
pub fn calculate_synastry_houses(person1_chart: Chart, person2_chart: Chart) -> SynastryChart

// Transit comparison
pub fn calculate_transits(natal_chart: Chart, transit_date: DateTime) -> TransitChart
```

### Frontend JavaScript Functions

```javascript
// Location search with debounce
async function searchLocation(query) { }

// Auto-fill coordinates
function fillCoordinates(lat, lon) { }

// Copy to clipboard
function copyToClipboard() { }

// Handle tab-specific field visibility
function updateFieldsForTab(tabName) { }

// Validate dual inputs
function validateInputs() { }
```

---

## 📐 UI Mockup Details

### Color Scheme
- Input panel background: Light gray (#f5f5f5)
- Output panel background: White (#ffffff)
- Primary button: Purple (#6366f1)
- Text: Dark gray (#1f2937)
- Borders: Light gray (#e5e7eb)

### Typography
- Headers: 18px bold
- Labels: 14px normal
- Input text: 14px
- Output text: 13px monospace (for alignment)

### Spacing
- Panel padding: 24px
- Field spacing: 16px
- Section spacing: 32px

---

## 🔄 Next Steps (Tomorrow)

1. ✅ Review this plan
2. ⏳ Implement Phase 1: Backend changes (Fortuna, Vertex, Synastry, Transits)
3. ⏳ Implement Phase 2: Frontend layout and dual inputs
4. ⏳ Implement Phase 3: Location search feature
5. ⏳ Implement Phase 4: Testing and polish
6. ⏳ Rebuild and deploy

---

## 📝 Notes

- All changes maintain backward compatibility
- Manual lat/long entry always available for offline use
- Gender field is optional (defaults to "Other" if not selected)
- Copy button copies entire output; users can also select partial text
- Synastry shows TRANSPOSED houses (Person A's planets in Person B's houses)
- Transits only show planetary transits, not angles or calculated points
- Layout is responsive and works on different screen sizes

---

## ⚠️ Potential Challenges

1. **GeoNames API rate limiting** - Solution: Implement client-side caching
2. **Textarea resize in Tauri** - Solution: Use CSS resize property
3. **Clipboard access** - Solution: Use Tauri clipboard API
4. **Complex synastry calculations** - Solution: Break into smaller functions
5. **Output formatting** - Solution: Create template system for different chart types

---

**Estimated Implementation Time:** 3-4 hours total
- Phase 1: 90 minutes
- Phase 2: 60 minutes
- Phase 3: 45 minutes
- Phase 4: 30 minutes

Ready to implement tomorrow! 🚀
