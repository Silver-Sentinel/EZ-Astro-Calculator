# 🌟 EZ Astro Calculator - FINAL VERSION 🌟

## 📦 YOUR COMPLETE PACKAGE

### Executables (Ready to Use!)
📍 Location: `E:\Claude Projects\EZ Astro Calculator\astro-calc-COMPLETE\astro-calc\target\release\`

1. **astro-calc.exe** (10.78 MB) - Standalone executable
   - Just double-click to run!
   - No installation needed

2. **Astro Calculator_0.1.0_x64_en-US.msi** (3.76 MB)
   - Professional MSI installer
   - Location: `target\release\bundle\msi\`

3. **Astro Calculator_0.1.0_x64-setup.exe** (2.5 MB)
   - Lightweight NSIS installer
   - Location: `target\release\bundle\nsis\`

---

## ✨ FEATURES COMPLETED

### 🎯 Core Functionality
✅ **Dual Natal Charts** - Calculate two people simultaneously
✅ **Synastry** - Relationship analysis with transposed houses
✅ **Transits** - Current planetary positions vs natal chart

### 🔍 Location Search
✅ **GeoNames API Integration**
   - Type any city name
   - Auto-fills coordinates
   - Real-time search with 300ms debounce
   - Offline fallback to manual entry

### 📄 Batch Processing (NEW!)
✅ **TXT File Import**
   - Process multiple charts at once
   - Simple CSV format
   - Perfect for research!

**Format:**
```
Name,Gender,YYYY-MM-DD,HH:MM,Timezone,Lat,Lon
John Doe,Male,1990-03-21,14:30,-05:00,40.7128,-74.0060
Jane Smith,Female,1992-09-15,08:15,-08:00,34.0522,-118.2437
```

**Sample File:** `SAMPLE_BATCH.txt` (in project root)

### 🎨 User Interface
✅ Split-panel layout (40% inputs, 60% output)
✅ Resizable output textarea
✅ One-click copy to clipboard
✅ Name & Gender fields
✅ Professional purple gradient theme
✅ Your custom icon
✅ **Social media links in footer** (NEW!)
   - Links to your X profile (@AquarianRising0)
   - Links to your YouTube channel

### 🔬 Calculations
✅ Fagan-Bradley Sidereal zodiac
✅ Placidus Houses
✅ Day/Night birth Fortuna (Part of Fortune)
✅ Vertex calculation
✅ All major aspects
✅ All planets + Chiron + True Node
✅ Angles (AC, MC)

---

## 🚀 HOW TO USE

### Single Chart Mode:
1. Open the app
2. Choose tab: Natal / Synastry / Transits
3. Type city name in "Search Location" (coordinates auto-fill)
4. Enter birth date, time, timezone
5. Click "Calculate"
6. Copy results with one click!

### Batch Mode:
1. Create a TXT file with format:
   ```
   Name,Gender,YYYY-MM-DD,HH:MM,Timezone,Lat,Lon
   ```
2. Open Natal Chart tab
3. Click "Choose TXT File" in Batch Import section
4. Select your file
5. All charts calculate automatically!
6. Results appear in output panel

---

## 📝 TXT FILE FORMAT

**Required Fields (7 total):**
1. **Name** - Any text (e.g., "John Doe")
2. **Gender** - Male, Female, or Other
3. **Date** - YYYY-MM-DD format (e.g., 1990-03-21)
4. **Time** - HH:MM format (e.g., 14:30)
5. **Timezone** - +/-HH:MM format (e.g., -05:00 for EST)
6. **Latitude** - Decimal degrees (e.g., 40.7128)
7. **Longitude** - Decimal degrees (e.g., -74.0060)

**Example:**
```
John Doe,Male,1990-03-21,14:30,-05:00,40.7128,-74.0060
Jane Smith,Female,1992-09-15,08:15,-08:00,34.0522,-118.2437
Alex Johnson,Other,1985-07-04,12:00,-06:00,41.8781,-87.6298
```

**Tips:**
- No spaces after commas (or trim them)
- Use negative for West longitude, South latitude
- Timezone format: -05:00 (EST), -08:00 (PST), +00:00 (GMT)
- One person per line
- No header row needed

---

## 🎓 COMMON TIMEZONES

| Location | Timezone |
|----------|----------|
| EST (New York) | -05:00 |
| CST (Chicago) | -06:00 |
| MST (Denver) | -07:00 |
| PST (Los Angeles) | -08:00 |
| GMT (London) | +00:00 |
| CET (Paris) | +01:00 |
| IST (India) | +05:30 |
| JST (Tokyo) | +09:00 |
| AEST (Sydney) | +10:00 |

---

## 🌐 SOCIAL LINKS

**Made by Aquarian Rising**
- X (Twitter): https://www.x.com/@AquarianRising0
- YouTube: https://www.youtube.com/@Aquarian_Rising

*(Links are clickable in the app footer!)*

---

## 🛠️ TECHNICAL SPECS

**Backend:**
- Rust + Swiss Ephemeris
- Fagan-Bradley Ayanamsa
- Placidus house system
- High-precision calculations

**Frontend:**
- Tauri 2.0
- Modern HTML5/CSS3/JavaScript
- GeoNames API for location search
- Async batch processing

**File Size:**
- Standalone EXE: 10.78 MB
- Includes all dependencies
- No external files needed (except ephemeris data, which is bundled)

---

## 📁 PROJECT STRUCTURE

```
E:\Claude Projects\EZ Astro Calculator\
├── astro-calc-COMPLETE\astro-calc\
│   ├── target\release\
│   │   ├── astro-calc.exe          ← YOUR MAIN APP
│   │   └── bundle\
│   │       ├── msi\                ← MSI Installer
│   │       └── nsis\               ← NSIS Installer
│   ├── dist\index.html             ← UI Source
│   ├── src\                        ← Rust Source
│   └── swisseph files...
├── SAMPLE_BATCH.txt                ← Example batch file
├── IMPLEMENTATION_PLAN.md          ← Development docs
├── CHECKLIST.md
├── LAYOUT_REFERENCE.md
└── TECHNICAL_REFERENCE.md
```

---

## 🎉 YOU'RE DONE!

This is your **COMPLETE, PRODUCTION-READY** sidereal astrology calculator!

### What You Can Do:
✅ Calculate unlimited charts
✅ Batch process for research
✅ Share with friends
✅ Use for client readings
✅ Promote on your social media

### Share It:
The app includes your social links, so anyone who uses it can find you on X and YouTube!

---

## 💡 TIPS FOR USE

1. **Location Search** - Type just 2-3 letters, wait for dropdown
2. **Batch Processing** - Keep TXT files under 100 entries for speed
3. **Copy Results** - Click copy button or select text manually
4. **Timezone** - Always use +/- format (e.g., -05:00)
5. **Coordinates** - North/East are positive, South/West are negative

---

**Enjoy your new astrology calculator! 🌟**

Made with ❤️ by Aquarian Rising
Built with Rust 🦀 + Tauri + Swiss Ephemeris
