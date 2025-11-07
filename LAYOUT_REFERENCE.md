# Layout Reference - Visual Guide

## Current Layout (Before)
```
┌─────────────────────────────────────────────────────────┐
│  [Natal Chart] [Synastry] [Transits]                    │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  Birth Date: [        ]                                  │
│  Birth Time: [        ]     Timezone: [      ]          │
│  Latitude: [        ]       Longitude: [      ]         │
│                                                          │
│              [Calculate Natal Chart]                     │
│                                                          │
│  ─────────────────────────────────────────────────      │
│                                                          │
│  Output appears here below inputs (requires scrolling)  │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

**Problems:**
- Only one set of inputs
- Output below inputs (can't see both)
- Can't copy partial results easily
- No location search


## New Layout (After)
```
┌────────────────────────────────────────────────────────────────────────────────┐
│  🌟 Astrological Chart Calculator                                              │
│  Fagan-Bradley Sidereal • Placidus Houses                                     │
│                                                                                │
│  [📊 Natal Chart] [💑 Synastry] [🔄 Transits]                                │
├──────────────────────────────────┬─────────────────────────────────────────────┤
│                                  │                                             │
│  INPUT PANEL (40%)               │  OUTPUT PANEL (60%)                         │
│  ══════════════════               │  ═══════════════════                        │
│                                  │                                             │
│  ▼ Person 1                      │  ┌───────────────────────────────────────┐ │
│  ├─ Name: [John Doe        ]     │  │ Chart Output Text                     │ │
│  ├─ Gender: ⦿ Male ○ Female      │  │                                       │ │
│  │          ○ Other               │  │ Sun 15° Aries in 3rd house           │ │
│  ├─ Birth Date: [03/21/1990] 📅  │  │ Moon 22° Cancer in 7th house; ...    │ │
│  ├─ Birth Time: [14:30   ] 🕐    │  │ Mercury 8° Pisces in 2nd house       │ │
│  ├─ Timezone: [GMT-5:00 (EST)] ▼ │  │ ...                                   │ │
│  │                                │  │                                       │ │
│  ├─ Location:                     │  │ (User can select and copy            │ │
│  │   [New York, NY       ] 🔍     │  │  any portion of this text)           │ │
│  │   ↓ Results dropdown           │  │                                       │ │
│  │   Or enter manually:           │  │ (Resizable by dragging edges)        │ │
│  ├─ Latitude:  [40.7128  ]        │  │                                       │ │
│  └─ Longitude: [-74.0060 ]        │  └───────────────────────────────────────┘ │
│                                  │                                             │
│  ▼ Person 2                      │  [📋 Copy to Clipboard]                     │
│  ├─ Name: [Jane Smith      ]     │                                             │
│  ├─ Gender: ○ Male ⦿ Female      │                                             │
│  │          ○ Other               │                                             │
│  ├─ Birth Date: [09/15/1992] 📅  │                                             │
│  ├─ Birth Time: [08:15   ] 🕐    │                                             │
│  ├─ Timezone: [GMT-8:00 (PST)] ▼ │                                             │
│  │                                │                                             │
│  ├─ Location:                     │                                             │
│  │   [Los Angeles, CA    ] 🔍     │                                             │
│  ├─ Latitude:  [34.0522  ]        │                                             │
│  └─ Longitude: [-118.2437]        │                                             │
│                                  │                                             │
│         [Calculate Chart]         │                                             │
│                                  │                                             │
└──────────────────────────────────┴─────────────────────────────────────────────┘
```

## Tab-Specific Behavior

### Natal Chart Tab
```
Person 1                Person 2
[All fields enabled]    [All fields enabled]

Output:
════════════════════════════════════════
Person 1: John Doe
────────────────────────────────────────
Sun 15° Aries in 3rd house
Moon 22° Cancer in 7th house; ...
[Full natal chart]

════════════════════════════════════════
Person 2: Jane Smith
────────────────────────────────────────
Sun 22° Virgo in 9th house
Moon 10° Taurus in 5th house; ...
[Full natal chart]
```

### Synastry Tab
```
Person 1                Person 2
[All fields enabled]    [All fields enabled]

Output:
════════════════════════════════════════
John → Jane (John's planets in Jane's houses)
────────────────────────────────────────
John's Sun 15° Aries in Jane's 3rd house
John's Moon 22° Cancer in Jane's 7th house; conjunct Jane's Venus
...

════════════════════════════════════════
Jane → John (Jane's planets in John's houses)
────────────────────────────────────────
Jane's Sun 22° Virgo in John's 9th house
Jane's Moon 10° Taurus in John's 5th house; trine John's Sun
...
```

### Transits Tab
```
Natal Person            Transit Date
[All fields enabled]    [Name GRAYED OUT]
                        [Date/Time enabled]

Output:
════════════════════════════════════════
Natal Chart: John Doe
────────────────────────────────────────
Sun 15° Aries in 3rd house
Moon 22° Cancer in 7th house
...
Fortuna 10° Taurus in 4th house
Vertex 18° Scorpio in 11th house
AC 5° Aquarius
MC 12° Scorpio

════════════════════════════════════════
Transits for 10/30/2025 14:30 GMT
────────────────────────────────────────
Transiting Sun 6° Scorpio in 11th house; trine natal Moon
Transiting Moon 14° Pisces in 3rd house; square natal Sun
Transiting Mercury 25° Libra in 10th house
...
(No Fortuna, Vertex, AC, MC in transits)
```

## Location Search Feature

### How It Works:
```
1. User types: "New Y"
   ↓
2. After 300ms, API call to GeoNames
   ↓
3. Dropdown appears:
   ┌─────────────────────────────────┐
   │ New York, NY, USA               │
   │ New York Mills, MN, USA         │
   │ New Ykepa, Liberia              │
   └─────────────────────────────────┘
   ↓
4. User clicks "New York, NY, USA"
   ↓
5. Latitude and Longitude auto-fill:
   Latitude: [40.7128]
   Longitude: [-74.0060]
```

### Offline Fallback:
```
Location: [New York] 🔍
          ↓
[❌ Unable to search locations. Enter coordinates manually below.]

Or enter manually:
Latitude: [        ]
Longitude: [        ]
```

## Output Panel Features

### Resizable Textarea
- Drag corners to resize
- Maintains minimum: 300px × 400px
- Has scrollbars for overflow
- Monospace font for alignment

### Copy Functionality
1. **Full copy:** Click "Copy to Clipboard" button
2. **Partial copy:** Select text with mouse, Ctrl+C

### Text Format (Monospace for alignment)
```
Sun      15° Aries      in 3rd house
Moon     22° Cancer     in 7th house; conjunct Venus
Mercury   8° Pisces     in 2nd house
Venus    18° Gemini     in 6th house; square Mars
Mars     25° Virgo      in 9th house
Jupiter  10° Sagittarius in 12th house
Saturn    5° Capricorn  in 1st house
```

## Color Scheme

### Input Panel (Left)
- Background: Light gray (#f5f5f5)
- Text: Dark gray (#1f2937)
- Borders: Light border (#e5e7eb)
- Fields: White background

### Output Panel (Right)
- Background: White (#ffffff)
- Textarea: White with light border
- Button: Purple (#6366f1)
- Text: Dark gray, monospace

### Tab Buttons
- Active: Purple (#6366f1)
- Inactive: Light gray
- Hover: Slightly darker

## Responsive Behavior

### Desktop (1920px+)
- Left: 40% | Right: 60%
- Large textarea
- No scrolling on inputs

### Laptop (1366px)
- Left: 40% | Right: 60%
- Medium textarea
- Inputs may scroll if many

### Tablet (768px)
- Stack vertically
- Inputs: 100% width on top
- Output: 100% width below

## Implementation Priority

### Phase 1: Core Structure ⭐⭐⭐ (Must Have)
- Dual input sets
- New layout (left/right panels)
- Output textarea
- Copy button

### Phase 2: Calculations ⭐⭐⭐ (Must Have)
- Fortuna & Vertex
- Synastry (transposed houses)
- Transits (exclude angles)

### Phase 3: Location Search ⭐⭐ (Nice to Have)
- GeoNames API
- Dropdown results
- Auto-fill coordinates
- Offline fallback

### Phase 4: Polish ⭐ (Enhancement)
- Resizable splitter between panels
- Remember last location search
- Form validation messages
- Loading indicators

## Key Differences from Current Version

| Feature | Before | After |
|---------|--------|-------|
| Input sets | 1 | 2 |
| Layout | Vertical | Horizontal split |
| Output | Below (scroll) | Right panel (visible) |
| Copy | No easy way | Button + selectable text |
| Location | Manual coords only | Search + manual |
| Fortuna/Vertex | Missing | Included |
| Synastry | Not implemented | Transposed houses |
| Transits | Not implemented | Planets only |
| Gender field | Missing | Added |
| Name/Label | Missing | Added |

## File Changes Summary

### Major Edits:
1. **dist/index.html** - Complete restructure (30+ changes)
2. **src/chart.rs** - Add 4 new functions
3. **src/formatter.rs** - Update output format
4. **CSS** - New flexbox layout

### New Files:
None (all modifications to existing files)

---

This reference guide shows exactly what we're building tomorrow. Refer to IMPLEMENTATION_PLAN.md for detailed technical specifications. 🚀
