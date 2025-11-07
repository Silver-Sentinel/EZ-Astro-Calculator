use crate::chart::{AnglePoint, CelestialBody, Chart};
use serde::{Deserialize, Serialize};

/// Aspect types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectType {
    // Major aspects
    Conjunction,
    Sextile,
    Square,
    Trine,
    Opposition,
    
    // Minor aspects
    SemiSextile,
    SemiSquare,
    Quintile,
    Sesquiquadrate,
    Quincunx,
}

impl AspectType {
    pub fn angle(&self) -> f64 {
        match self {
            AspectType::Conjunction => 0.0,
            AspectType::SemiSextile => 30.0,
            AspectType::SemiSquare => 45.0,
            AspectType::Sextile => 60.0,
            AspectType::Quintile => 72.0,
            AspectType::Square => 90.0,
            AspectType::Trine => 120.0,
            AspectType::Sesquiquadrate => 135.0,
            AspectType::Quincunx => 150.0,
            AspectType::Opposition => 180.0,
        }
    }

    /// Get standard orb for this aspect (Astro.com standards)
    pub fn standard_orb(&self) -> f64 {
        match self {
            AspectType::Conjunction | AspectType::Opposition => 8.0, // Base for planets (Sun/Moon get +2° bonus)
            AspectType::Square | AspectType::Trine => 8.0,
            AspectType::Sextile => 6.0,
            AspectType::SemiSextile | AspectType::SemiSquare | 
            AspectType::Sesquiquadrate | AspectType::Quincunx => 2.0,
            AspectType::Quintile => 2.0,
        }
    }

    pub fn symbol(&self) -> &str {
        match self {
            AspectType::Conjunction => "conjunct",
            AspectType::Sextile => "sextile",
            AspectType::Square => "square",
            AspectType::Trine => "trine",
            AspectType::Opposition => "opposite",
            AspectType::SemiSextile => "semi-sextile",
            AspectType::SemiSquare => "semi-square",
            AspectType::Quintile => "quintile",
            AspectType::Sesquiquadrate => "sesquiquadrate",
            AspectType::Quincunx => "quincunx",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            AspectType::Conjunction,
            AspectType::Sextile,
            AspectType::Square,
            AspectType::Trine,
            AspectType::Opposition,
            AspectType::SemiSextile,
            AspectType::SemiSquare,
            AspectType::Quintile,
            AspectType::Sesquiquadrate,
            AspectType::Quincunx,
        ]
    }
}

/// An entity that can be involved in aspects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AspectEntity {
    Body(CelestialBody),
    Angle(AnglePoint),
}

impl AspectEntity {
    pub fn to_string(&self) -> String {
        match self {
            AspectEntity::Body(body) => body.to_string().to_string(),
            AspectEntity::Angle(angle) => angle.to_string().to_string(),
        }
    }
}

/// A calculated aspect between two entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aspect {
    pub entity1: AspectEntity,
    pub entity2: AspectEntity,
    pub aspect_type: AspectType,
    pub orb: f64, // Actual orb in degrees
}

impl Aspect {
    pub fn description(&self) -> String {
        format!(
            "{} {} {}",
            self.aspect_type.symbol(),
            self.entity2.to_string(),
            if self.orb < 1.0 {
                format!("(exact {:.1}°)", self.orb)
            } else {
                String::new()
            }
        )
    }
}

/// Calculate angular separation between two longitudes
fn angular_separation(lon1: f64, lon2: f64) -> f64 {
    let mut diff = (lon2 - lon1).abs();
    if diff > 180.0 {
        diff = 360.0 - diff;
    }
    diff
}

/// Check if two longitudes form an aspect
fn check_aspect(
    lon1: f64,
    lon2: f64,
    entity1: &AspectEntity,
    entity2: &AspectEntity,
) -> Option<Aspect> {
    let separation = angular_separation(lon1, lon2);

    for aspect_type in AspectType::all() {
        let target_angle = aspect_type.angle();
        let actual_orb = (separation - target_angle).abs();
        
        // Get allowed orb based on entities involved
        let mut allowed_orb = aspect_type.standard_orb();
        
        // Sun and Moon get +2° bonus on major aspects
        if matches!(aspect_type, AspectType::Conjunction | AspectType::Opposition | 
                   AspectType::Square | AspectType::Trine) {
            let has_luminary = matches!(entity1, AspectEntity::Body(CelestialBody::Sun) | AspectEntity::Body(CelestialBody::Moon))
                || matches!(entity2, AspectEntity::Body(CelestialBody::Sun) | AspectEntity::Body(CelestialBody::Moon));
            if has_luminary {
                allowed_orb += 2.0;
            }
        }

        if actual_orb <= allowed_orb {
            return Some(Aspect {
                entity1: entity1.clone(),
                entity2: entity2.clone(),
                aspect_type,
                orb: actual_orb,
            });
        }
    }

    None
}

/// Calculate all aspects in a chart
pub fn calculate_aspects(chart: &Chart) -> Vec<(AspectEntity, Vec<Aspect>)> {
    let mut result = Vec::new();

    // Get all entities with their longitudes
    let mut entities: Vec<(AspectEntity, f64)> = Vec::new();

    // Add bodies
    for pos in &chart.positions {
        entities.push((AspectEntity::Body(pos.body), pos.longitude));
    }

    // Add angles
    for angle_pos in &chart.angles {
        entities.push((AspectEntity::Angle(angle_pos.angle), angle_pos.longitude));
    }

    // Calculate aspects for each entity
    for (i, (entity1, lon1)) in entities.iter().enumerate() {
        let mut aspects_for_entity = Vec::new();

        // Check aspects with all other entities
        for (entity2, lon2) in entities.iter().skip(i + 1) {
            if let Some(aspect) = check_aspect(*lon1, *lon2, entity1, entity2) {
                aspects_for_entity.push(aspect);
            }
        }

        // Sort by aspect type (major first, then by angle)
        aspects_for_entity.sort_by(|a, b| {
            let order_a = a.aspect_type.angle();
            let order_b = b.aspect_type.angle();
            order_a.partial_cmp(&order_b).unwrap()
        });

        if !aspects_for_entity.is_empty() {
            result.push((entity1.clone(), aspects_for_entity));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angular_separation() {
        assert!((angular_separation(10.0, 20.0) - 10.0).abs() < 0.01);
        assert!((angular_separation(350.0, 10.0) - 20.0).abs() < 0.01);
        assert!((angular_separation(180.0, 0.0) - 180.0).abs() < 0.01);
    }

    #[test]
    fn test_aspect_detection() {
        let entity1 = AspectEntity::Body(CelestialBody::Sun);
        let entity2 = AspectEntity::Body(CelestialBody::Moon);

        // Exact conjunction
        let aspect = check_aspect(0.0, 0.0, &entity1, &entity2);
        assert!(aspect.is_some());
        assert_eq!(aspect.unwrap().aspect_type, AspectType::Conjunction);

        // Sextile with Sun/Moon (should work with 8° orb)
        let aspect = check_aspect(0.0, 68.0, &entity1, &entity2);
        assert!(aspect.is_some());
        assert_eq!(aspect.unwrap().aspect_type, AspectType::Sextile);
    }
}
