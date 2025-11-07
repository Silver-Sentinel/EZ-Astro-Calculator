use serde::{Deserialize, Serialize};

/// Zodiac signs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZodiacSign {
    Aries,
    Taurus,
    Gemini,
    Cancer,
    Leo,
    Virgo,
    Libra,
    Scorpio,
    Sagittarius,
    Capricorn,
    Aquarius,
    Pisces,
}

impl ZodiacSign {
    pub fn from_longitude(lon: f64) -> Self {
        let sign_num = (lon / 30.0).floor() as i32;
        match sign_num % 12 {
            0 => ZodiacSign::Aries,
            1 => ZodiacSign::Taurus,
            2 => ZodiacSign::Gemini,
            3 => ZodiacSign::Cancer,
            4 => ZodiacSign::Leo,
            5 => ZodiacSign::Virgo,
            6 => ZodiacSign::Libra,
            7 => ZodiacSign::Scorpio,
            8 => ZodiacSign::Sagittarius,
            9 => ZodiacSign::Capricorn,
            10 => ZodiacSign::Aquarius,
            11 => ZodiacSign::Pisces,
            _ => ZodiacSign::Aries, // Should never happen
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            ZodiacSign::Aries => "Aries",
            ZodiacSign::Taurus => "Taurus",
            ZodiacSign::Gemini => "Gemini",
            ZodiacSign::Cancer => "Cancer",
            ZodiacSign::Leo => "Leo",
            ZodiacSign::Virgo => "Virgo",
            ZodiacSign::Libra => "Libra",
            ZodiacSign::Scorpio => "Scorpio",
            ZodiacSign::Sagittarius => "Sagittarius",
            ZodiacSign::Capricorn => "Capricorn",
            ZodiacSign::Aquarius => "Aquarius",
            ZodiacSign::Pisces => "Pisces",
        }
    }
}

/// Celestial body types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CelestialBody {
    Sun,
    Moon,
    Mercury,
    Venus,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
    Pluto,
    TrueNode,
    Chiron,
    Fortuna,
    Vertex,
}

impl CelestialBody {
    pub fn to_string(&self) -> &str {
        match self {
            CelestialBody::Sun => "Sun",
            CelestialBody::Moon => "Moon",
            CelestialBody::Mercury => "Mercury",
            CelestialBody::Venus => "Venus",
            CelestialBody::Mars => "Mars",
            CelestialBody::Jupiter => "Jupiter",
            CelestialBody::Saturn => "Saturn",
            CelestialBody::Uranus => "Uranus",
            CelestialBody::Neptune => "Neptune",
            CelestialBody::Pluto => "Pluto",
            CelestialBody::TrueNode => "True Node",
            CelestialBody::Chiron => "Chiron",
            CelestialBody::Fortuna => "Fortuna",
            CelestialBody::Vertex => "Vertex",
        }
    }

    /// Get default orb for this body in aspects
    pub fn orb(&self) -> f64 {
        match self {
            CelestialBody::Sun | CelestialBody::Moon => 10.0,
            _ => 8.0, // planets use 8° for major aspects
        }
    }

    /// All bodies to calculate
    pub fn all() -> Vec<Self> {
        vec![
            CelestialBody::Sun,
            CelestialBody::Moon,
            CelestialBody::Mercury,
            CelestialBody::Venus,
            CelestialBody::Mars,
            CelestialBody::Jupiter,
            CelestialBody::Saturn,
            CelestialBody::Uranus,
            CelestialBody::Neptune,
            CelestialBody::Pluto,
            CelestialBody::TrueNode,
            CelestialBody::Chiron,
            CelestialBody::Fortuna,
            CelestialBody::Vertex,
        ]
    }
}

/// Angle points
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnglePoint {
    Ascendant,
    Midheaven,
}

impl AnglePoint {
    pub fn to_string(&self) -> &str {
        match self {
            AnglePoint::Ascendant => "AC",
            AnglePoint::Midheaven => "MC",
        }
    }
}

/// A celestial position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub body: CelestialBody,
    pub longitude: f64,  // Ecliptic longitude (0-360)
    pub retrograde: bool,
    pub house: u8,       // House number (1-12)
}

impl Position {
    pub fn sign(&self) -> ZodiacSign {
        ZodiacSign::from_longitude(self.longitude)
    }

    pub fn degree_in_sign(&self) -> f64 {
        self.longitude % 30.0
    }
}

/// Angle positions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnglePosition {
    pub angle: AnglePoint,
    pub longitude: f64,
}

impl AnglePosition {
    pub fn sign(&self) -> ZodiacSign {
        ZodiacSign::from_longitude(self.longitude)
    }

    pub fn degree_in_sign(&self) -> f64 {
        self.longitude % 30.0
    }
}

/// House cusps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseCusps {
    pub cusps: [f64; 12], // Longitude of each house cusp (1-12)
}

impl HouseCusps {
    pub fn new(cusps: [f64; 12]) -> Self {
        Self { cusps }
    }

    pub fn get_house(&self, longitude: f64) -> u8 {
        // Find which house a longitude falls into
        for i in 0..12 {
            let current = self.cusps[i];
            let next = if i == 11 { self.cusps[0] } else { self.cusps[i + 1] };
            
            if next > current {
                if longitude >= current && longitude < next {
                    return (i + 1) as u8;
                }
            } else {
                // Handle wrap around 0°
                if longitude >= current || longitude < next {
                    return (i + 1) as u8;
                }
            }
        }
        1 // Default to house 1
    }
}

/// Complete astrological chart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    pub positions: Vec<Position>,
    pub angles: Vec<AnglePosition>,
    pub houses: HouseCusps,
    pub name: Option<String>,
    pub gender: Option<String>,
}

impl Chart {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            angles: Vec::new(),
            houses: HouseCusps::new([0.0; 12]),
            name: None,
            gender: None,
        }
    }

    pub fn with_metadata(mut self, name: Option<String>, gender: Option<String>) -> Self {
        self.name = name;
        self.gender = gender;
        self
    }

    pub fn get_position(&self, body: CelestialBody) -> Option<&Position> {
        self.positions.iter().find(|p| p.body == body)
    }

    pub fn get_angle(&self, angle: AnglePoint) -> Option<&AnglePosition> {
        self.angles.iter().find(|a| a.angle == angle)
    }
}
