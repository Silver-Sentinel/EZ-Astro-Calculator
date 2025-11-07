# Technical Reference - Calculations

## Fortuna (Part of Fortune) Calculation

### Formula
```
Day Birth (Sun above horizon):
Fortuna = ASC + Moon - Sun

Night Birth (Sun below horizon):  
Fortuna = ASC + Sun - Moon
```

### Rust Implementation
```rust
pub fn calculate_fortuna(
    ascendant: f64,
    sun_position: f64,
    moon_position: f64,
    is_day_birth: bool
) -> f64 {
    let fortuna = if is_day_birth {
        ascendant + moon_position - sun_position
    } else {
        ascendant + sun_position - moon_position
    };
    
    // Normalize to 0-360 range
    let mut result = fortuna % 360.0;
    if result < 0.0 {
        result += 360.0;
    }
    result
}
```

### Determining Day vs Night Birth
```rust
fn is_day_birth(sun_position: f64, ascendant: f64) -> bool {
    // Sun is above horizon if between ASC and DESC (ASC + 180)
    let descendant = (ascendant + 180.0) % 360.0;
    
    // Check if sun is in the "day" half of the chart
    if ascendant < descendant {
        sun_position >= ascendant && sun_position < descendant
    } else {
        sun_position >= ascendant || sun_position < descendant
    }
}
```

---

## Vertex Calculation

### Formula
The Vertex is calculated using:
```
Vertex = arctan(sin(RAMC) / (cos(RAMC) * cos(ε) - tan(lat) * sin(ε)))
```

Where:
- RAMC = Right Ascension of Midheaven
- ε (epsilon) = Obliquity of the ecliptic (~23.4°)
- lat = Geographic latitude

### Simplified Approach
For practical use, the Vertex can be approximated as:
```
Vertex ≈ 180° - MC (roughly opposite the MC, adjusted for latitude)
```

### Rust Implementation
```rust
pub fn calculate_vertex(
    mc_degrees: f64,
    latitude: f64,
    ramc: f64
) -> f64 {
    // Convert to radians
    let lat_rad = latitude.to_radians();
    let ramc_rad = ramc.to_radians();
    let epsilon = 23.4392811; // Obliquity of ecliptic
    let epsilon_rad = epsilon.to_radians();
    
    // Calculate vertex
    let numerator = ramc_rad.sin();
    let denominator = ramc_rad.cos() * epsilon_rad.cos() 
                     - lat_rad.tan() * epsilon_rad.sin();
    
    let vertex_rad = numerator.atan2(denominator);
    let mut vertex_deg = vertex_rad.to_degrees();
    
    // Normalize to 0-360
    if vertex_deg < 0.0 {
        vertex_deg += 360.0;
    }
    
    vertex_deg
}
```

---

## Synastry House Overlay Calculation

### Concept
Calculate Person A's planets in Person B's house system (and vice versa).

### Process
1. Get Person A's planetary positions (absolute ecliptic longitude)
2. Get Person B's house cusps
3. Determine which of Person B's houses each of Person A's planets falls into
4. Repeat in reverse (Person B's planets in Person A's houses)

### Rust Implementation
```rust
pub struct SynastryChart {
    pub person1_in_person2_houses: Vec<PlanetInHouse>,
    pub person2_in_person1_houses: Vec<PlanetInHouse>,
    pub inter_aspects: Vec<Aspect>,
}

pub struct PlanetInHouse {
    pub person: String,
    pub planet: CelestialBody,
    pub position: f64,
    pub house_number: u8,
    pub house_owner: String,
}

pub fn calculate_synastry(
    chart1: &Chart,
    person1_name: &str,
    chart2: &Chart,
    person2_name: &str
) -> SynastryChart {
    // Person 1's planets in Person 2's houses
    let p1_in_p2 = calculate_planets_in_houses(
        &chart1.bodies,
        &chart2.house_cusps,
        person1_name,
        person2_name
    );
    
    // Person 2's planets in Person 1's houses
    let p2_in_p1 = calculate_planets_in_houses(
        &chart2.bodies,
        &chart1.house_cusps,
        person2_name,
        person1_name
    );
    
    // Inter-chart aspects
    let aspects = find_inter_chart_aspects(&chart1.bodies, &chart2.bodies);
    
    SynastryChart {
        person1_in_person2_houses: p1_in_p2,
        person2_in_person1_houses: p2_in_p1,
        inter_aspects: aspects,
    }
}

fn calculate_planets_in_houses(
    planets: &[CelestialBody],
    house_cusps: &[f64; 12],
    planet_owner: &str,
    house_owner: &str
) -> Vec<PlanetInHouse> {
    let mut result = Vec::new();
    
    for planet in planets {
        let house_num = determine_house(planet.position, house_cusps);
        result.push(PlanetInHouse {
            person: planet_owner.to_string(),
            planet: planet.clone(),
            position: planet.position,
            house_number: house_num,
            house_owner: house_owner.to_string(),
        });
    }
    
    result
}

fn determine_house(planet_position: f64, cusps: &[f64; 12]) -> u8 {
    // Find which house the planet falls into
    for i in 0..12 {
        let cusp_start = cusps[i];
        let cusp_end = if i == 11 {
            cusps[0] // Wrap around to 1st house cusp
        } else {
            cusps[i + 1]
        };
        
        // Handle the wrap-around case (12th to 1st house)
        if cusp_start > cusp_end {
            if planet_position >= cusp_start || planet_position < cusp_end {
                return (i + 1) as u8;
            }
        } else {
            if planet_position >= cusp_start && planet_position < cusp_end {
                return (i + 1) as u8;
            }
        }
    }
    
    1 // Default to 1st house if calculation fails
}
```

---

## Transit Calculation

### Concept
Compare current planetary positions to natal chart positions.

### Key Requirements
1. **Natal chart includes:** All planets + Fortuna + Vertex + AC + MC
2. **Transit objects include:** Only planets (Sun through Pluto)
3. **Exclude from transits:** Fortuna, Vertex, AC, MC (these don't move)

### Rust Implementation
```rust
pub struct TransitChart {
    pub natal_chart: Chart,
    pub natal_fortuna: f64,
    pub natal_vertex: f64,
    pub transit_date: String,
    pub transiting_planets: Vec<TransitPlanet>,
}

pub struct TransitPlanet {
    pub planet: CelestialBody,
    pub position: f64,
    pub house_in_natal: u8,
    pub aspects_to_natal: Vec<Aspect>,
}

pub fn calculate_transits(
    natal_chart: &Chart,
    transit_jd: f64,
    sweph_path: &str
) -> Result<TransitChart, String> {
    // Calculate transit planetary positions
    let transit_positions = calculate_positions_for_date(transit_jd, sweph_path)?;
    
    // For each transiting planet, find:
    // 1. Which natal house it's in
    // 2. Aspects to natal planets
    let transiting_planets: Vec<TransitPlanet> = transit_positions
        .iter()
        .filter(|p| !is_angle_or_point(p)) // Exclude non-planetary objects
        .map(|planet| {
            let house = determine_house(planet.position, &natal_chart.house_cusps);
            let aspects = find_aspects_to_natal(planet, &natal_chart.bodies);
            
            TransitPlanet {
                planet: planet.clone(),
                position: planet.position,
                house_in_natal: house,
                aspects_to_natal: aspects,
            }
        })
        .collect();
    
    Ok(TransitChart {
        natal_chart: natal_chart.clone(),
        natal_fortuna: natal_chart.fortuna,
        natal_vertex: natal_chart.vertex,
        transit_date: format_date(transit_jd),
        transiting_planets,
    })
}

fn is_angle_or_point(body: &CelestialBody) -> bool {
    // Exclude angles and calculated points from transits
    matches!(body.body_type, 
        BodyType::Fortuna | BodyType::Vertex | 
        BodyType::Ascendant | BodyType::Midheaven)
}

fn find_aspects_to_natal(
    transit_planet: &CelestialBody,
    natal_planets: &[CelestialBody]
) -> Vec<Aspect> {
    let mut aspects = Vec::new();
    
    for natal_planet in natal_planets {
        if let Some(aspect) = calculate_aspect(
            transit_planet.position,
            natal_planet.position
        ) {
            aspects.push(Aspect {
                body1: transit_planet.clone(),
                body2: natal_planet.clone(),
                aspect_type: aspect,
                orb: calculate_orb(transit_planet.position, natal_planet.position),
            });
        }
    }
    
    aspects
}
```

---

## Output Formatting

### Natal Chart Format
```
════════════════════════════════════════
Person: John Doe (Male)
Birth: March 21, 1990, 14:30 EST
Location: New York, NY (40.7128°N, 74.0060°W)
════════════════════════════════════════

PLANETS
────────────────────────────────────────
Sun      15° Aries      3rd house
Moon     22° Cancer     7th house; conjunct Venus
Mercury   8° Pisces     2nd house
Venus    18° Gemini     6th house; square Mars
Mars     25° Virgo      9th house
Jupiter  10° Sagittarius 12th house
Saturn    5° Capricorn  1st house; trine Sun
Uranus   12° Capricorn  1st house
Neptune  14° Capricorn  1st house; conjunct Uranus
Pluto    18° Scorpio    11th house

CALCULATED POINTS
────────────────────────────────────────
Fortuna  10° Taurus     4th house
Vertex   18° Scorpio    11th house

ANGLES
────────────────────────────────────────
Ascendant  5° Aquarius
Midheaven 12° Scorpio

HOUSE CUSPS
────────────────────────────────────────
1st:  5° Aquarius    7th: 5° Leo
2nd: 15° Pisces      8th: 15° Virgo
3rd: 20° Aries       9th: 20° Libra
4th: 12° Scorpio    10th: 12° Taurus
5th:  8° Sagittarius 11th: 8° Gemini
6th:  2° Capricorn  12th: 2° Cancer
```

### Synastry Format
```
════════════════════════════════════════
SYNASTRY: John → Jane
(John's planets in Jane's houses)
════════════════════════════════════════
John's Sun 15° Aries in Jane's 3rd house
John's Moon 22° Cancer in Jane's 7th house; conjunct Jane's Venus
John's Mercury 8° Pisces in Jane's 2nd house; sextile Jane's Moon
...

════════════════════════════════════════
SYNASTRY: Jane → John
(Jane's planets in John's houses)
════════════════════════════════════════
Jane's Sun 22° Virgo in John's 9th house
Jane's Moon 10° Taurus in John's 5th house; trine John's Sun
Jane's Venus 18° Gemini in John's 6th house; conjunct John's Moon
...

INTER-CHART ASPECTS
────────────────────────────────────────
John's Sun conjunct Jane's Mars (orb: 2.3°)
John's Venus trine Jane's Neptune (orb: 1.8°)
Jane's Moon square John's Saturn (orb: 3.1°)
...
```

### Transit Format
```
════════════════════════════════════════
NATAL CHART: John Doe
════════════════════════════════════════
Sun 15° Aries in 3rd house
Moon 22° Cancer in 7th house
Mercury 8° Pisces in 2nd house
...
Fortuna 10° Taurus in 4th house
Vertex 18° Scorpio in 11th house
Ascendant 5° Aquarius
Midheaven 12° Scorpio

════════════════════════════════════════
TRANSITS FOR October 30, 2025, 14:30 GMT
════════════════════════════════════════
☉ Sun      6° Scorpio     in natal 11th house
  • Trine natal Moon (orb: 2.1°)
  • Square natal Mercury (orb: 3.5°)

☽ Moon    14° Pisces     in natal 3rd house
  • Conjunct natal Mercury (orb: 1.2°)

☿ Mercury 25° Libra      in natal 10th house
  • Opposition natal Mars (orb: 2.8°)

...

[Note: Fortuna, Vertex, AC, MC not shown as transits]
```

---

## Aspect Orbs

### Standard Orbs (in degrees)
```rust
pub fn get_orb_for_aspect(aspect_type: AspectType, body1: CelestialBody, body2: CelestialBody) -> f64 {
    // Tighter orbs for outer planets
    let base_orb = match aspect_type {
        AspectType::Conjunction => 8.0,
        AspectType::Opposition => 8.0,
        AspectType::Trine => 8.0,
        AspectType::Square => 7.0,
        AspectType::Sextile => 6.0,
    };
    
    // Reduce orb if outer planets involved
    if is_outer_planet(&body1) || is_outer_planet(&body2) {
        base_orb * 0.75 // 75% of base orb
    } else {
        base_orb
    }
}

fn is_outer_planet(body: &CelestialBody) -> bool {
    matches!(body.body_type,
        BodyType::Uranus | BodyType::Neptune | BodyType::Pluto)
}
```

---

## House System

**Using: Placidus**
- Most commonly used house system
- Time-based division
- Houses vary in size based on latitude

Swiss Ephemeris house system code: `'P'`

```rust
pub const HOUSE_SYSTEM: char = 'P'; // Placidus
```

---

## Zodiac System

**Using: Fagan-Bradley Sidereal**
- Ayanamsa value: ~24.1° (varies by date)
- Swiss Ephemeris ayanamsa code: `SE_SIDM_FAGAN_BRADLEY`

```rust
pub const AYANAMSA: i32 = SE_SIDM_FAGAN_BRADLEY; // 0
```

---

This technical reference provides all formulas and implementation details needed for tomorrow's coding session. 🚀
