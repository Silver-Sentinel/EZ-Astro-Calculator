use crate::aspects::{AspectEntity, calculate_aspects};
use crate::chart::{AnglePoint, Chart, CelestialBody, Position, ZodiacSign};

/// Format a position in the user's requested format
fn format_position(pos: &Position, label_suffix: &str) -> String {
    let degree = pos.degree_in_sign().floor() as u32;
    let retro = if pos.retrograde { "r" } else { "" };
    let descending = if matches!(pos.body, CelestialBody::TrueNode) && pos.retrograde {
        "d"
    } else {
        ""
    };
    
    format!(
        "{}{}{} {} {}{}",
        degree,
        retro,
        descending,
        pos.sign().to_string(),
        label_suffix,
        pos.house
    )
}

/// Format aspects for an entity
fn format_aspects(aspects: &[crate::aspects::Aspect]) -> String {
    if aspects.is_empty() {
        return String::new();
    }

    let aspect_strs: Vec<String> = aspects
        .iter()
        .map(|a| format!("{} {}", a.aspect_type.symbol(), a.entity2.to_string()))
        .collect();

    aspect_strs.join(", ")
}

/// Format a natal chart
pub fn format_natal_chart(chart: &Chart) -> String {
    format_single_chart(chart, "H", None)
}

/// Format a transit chart with natal reference
pub fn format_transit_chart(natal: &Chart, transit: &Chart) -> String {
    let mut output = Vec::new();
    
    output.push("=== NATAL CHART ===".to_string());
    output.push(String::new());
    output.push(format_single_chart(natal, "H", None));
    output.push(String::new());
    output.push("=== TRANSITING POSITIONS ===".to_string());
    output.push(String::new());
    
    // Calculate aspects between transit planets and natal planets
    let mut combined_chart = transit.clone();
    // Add natal positions for aspect calculation
    for pos in &natal.positions {
        let natal_pos = pos.clone();
        // Mark as natal for aspect calculation by changing the body reference
        combined_chart.positions.push(natal_pos);
    }
    
    let all_aspects = calculate_aspects(&combined_chart);
    
    // Find aspects for each transiting planet
    let find_aspects = |entity: &AspectEntity| -> Vec<crate::aspects::Aspect> {
        all_aspects
            .iter()
            .find(|(e, _)| e == entity)
            .map(|(_, aspects)| aspects.clone())
            .unwrap_or_default()
    };

    // Format transiting planets
    for pos in &transit.positions {
        let entity = AspectEntity::Body(pos.body);
        let aspects = find_aspects(&entity);
        let aspect_str = format_aspects(&aspects);
        
        let line = if aspect_str.is_empty() {
            format!("Transiting {} {} (in natal house {}).", 
                pos.body.to_string(), 
                format_position_short(pos),
                pos.house)
        } else {
            format!("Transiting {} {} (in natal house {}); {}.", 
                pos.body.to_string(), 
                format_position_short(pos),
                pos.house,
                aspect_str)
        };
        output.push(line);
    }
    
    output.join("\n")
}

/// Format synastry charts
pub fn format_synastry_chart(chart1: &Chart, label1: &str, chart2: &Chart, label2: &str) -> String {
    let mut output = Vec::new();
    
    output.push(format!("=== {}'S CHART ===", label1.to_uppercase()));
    output.push(String::new());
    output.push(format_single_chart(chart1, &format!("H{}", label2), Some(label1)));
    output.push(String::new());
    
    output.push(format!("=== {}'S CHART ===", label2.to_uppercase()));
    output.push(String::new());
    output.push(format_single_chart(chart2, &format!("H{}", label1), Some(label2)));
    output.push(String::new());
    
    // Calculate inter-aspects
    output.push("=== SYNASTRY ASPECTS ===".to_string());
    output.push(String::new());
    
    let mut combined_chart = chart1.clone();
    for pos in &chart2.positions {
        combined_chart.positions.push(pos.clone());
    }
    for angle in &chart2.angles {
        combined_chart.angles.push(angle.clone());
    }
    
    let all_aspects = calculate_aspects(&combined_chart);
    
    // Find inter-aspects (aspects between the two charts)
    for (entity1, aspects) in all_aspects {
        if !aspects.is_empty() {
            // Check if this entity is from chart1 by checking if position exists in chart1
            let is_chart1_entity = match &entity1 {
                AspectEntity::Body(body) => {
                    chart1.positions.iter().any(|p| p.body == *body)
                }
                AspectEntity::Angle(angle) => {
                    chart1.angles.iter().any(|a| a.angle == *angle)
                }
            };
            
            if is_chart1_entity {
                // Filter for aspects to chart2 entities
                let inter_aspects: Vec<_> = aspects.iter()
                    .filter(|a| {
                        match &a.entity2 {
                            AspectEntity::Body(body) => {
                                chart2.positions.iter().any(|p| p.body == *body)
                            }
                            AspectEntity::Angle(angle) => {
                                chart2.angles.iter().any(|a| a.angle == *angle)
                            }
                        }
                    })
                    .collect();
                
                if !inter_aspects.is_empty() {
                    let aspect_str = inter_aspects.iter()
                        .map(|a| format!("{} {}'s {}", 
                            a.aspect_type.symbol(), 
                            label2,
                            a.entity2.to_string()))
                        .collect::<Vec<_>>()
                        .join(", ");
                    
                    output.push(format!("{}'s {}: {}.", label1, entity1.to_string(), aspect_str));
                }
            }
        }
    }
    
    output.join("\n")
}

/// Format a single chart (internal helper)
fn format_single_chart(chart: &Chart, house_prefix: &str, _label: Option<&str>) -> String {
    let mut output = Vec::new();

    // Calculate all aspects
    let all_aspects = calculate_aspects(chart);

    // Helper to find aspects for an entity
    let find_aspects = |entity: &AspectEntity| -> Vec<crate::aspects::Aspect> {
        all_aspects
            .iter()
            .find(|(e, _)| e == entity)
            .map(|(_, aspects)| aspects.clone())
            .unwrap_or_default()
    };

    // Format each body
    let bodies_to_show = [
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
    ];

    for body in &bodies_to_show {
        if let Some(pos) = chart.get_position(*body) {
            let entity = AspectEntity::Body(*body);
            let aspects = find_aspects(&entity);
            let aspect_str = format_aspects(&aspects);
            
            let line = if aspect_str.is_empty() {
                format!("{} {}.", body.to_string(), format_position(pos, house_prefix))
            } else {
                format!("{} {}; {}.", body.to_string(), format_position(pos, house_prefix), aspect_str)
            };
            output.push(line);
        }
    }

    // Format angles
    for angle_type in [AnglePoint::Ascendant, AnglePoint::Midheaven] {
        if let Some(angle_pos) = chart.get_angle(angle_type) {
            let entity = AspectEntity::Angle(angle_type);
            let aspects = find_aspects(&entity);
            let aspect_str = format_aspects(&aspects);
            
            let degree = angle_pos.degree_in_sign().floor() as u32;
            let sign = angle_pos.sign();
            let sign_str = sign.to_string();
            
            let line = if aspect_str.is_empty() {
                format!("{} {} {}.", angle_type.to_string(), degree, sign_str)
            } else {
                format!("{} {} {}; {}.", angle_type.to_string(), degree, sign_str, aspect_str)
            };
            output.push(line);
        }
    }

    // Add house cusps section
    output.push(String::new());
    output.push("House #s > Degree > Signs Respective to House #s:".to_string());
    output.push(String::new());

    // Format house cusps in pairs (1/7, 2/8, 3/9, 4/10, 5/11, 6/12)
    for i in 0..6 {
        let house1 = i + 1;
        let house7 = i + 7;
        
        let cusp1 = chart.houses.cusps[i];
        let cusp7 = chart.houses.cusps[i + 6];
        
        let sign1 = ZodiacSign::from_longitude(cusp1);
        let sign7 = ZodiacSign::from_longitude(cusp7);
        
        let deg1 = (cusp1 % 30.0).floor() as u32;
        let deg7 = (cusp7 % 30.0).floor() as u32;
        
        output.push(format!(
            "House {}/{} {} {}/{} {}",
            house1, house7, deg1, sign1.to_string(), deg7, sign7.to_string()
        ));
    }

    output.join("\n")
}

/// Format position without house (for transits)
fn format_position_short(pos: &Position) -> String {
    let degree = pos.degree_in_sign().floor() as u32;
    let retro = if pos.retrograde { "r" } else { "" };
    let descending = if matches!(pos.body, CelestialBody::TrueNode) && pos.retrograde {
        "d"
    } else {
        ""
    };
    
    format!(
        "{}{}{} {}",
        degree,
        retro,
        descending,
        pos.sign().to_string()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chart::HouseCusps;

    #[test]
    fn test_format_position() {
        let pos = Position {
            body: CelestialBody::Sun,
            longitude: 71.5, // 11.5 Gemini
            retrograde: false,
            house: 12,
        };
        
        let formatted = format_position(&pos, "H");
        assert!(formatted.contains("11"));
        assert!(formatted.contains("Gemini"));
        assert!(formatted.contains("H12"));
    }
}
