// Pyrmnont Brewery 2025

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeerSensory {
    pub beer_tastes: BeerTasteData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeerTasteData {
    pub categories: HashMap<String, Category>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub subcategories: HashMap<String, Subcategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subcategory {
    pub descriptors: Vec<String>,
    pub notes: String,
}

#[derive(Debug)]
pub struct BeerSensorySearch {
    sensory: BeerSensory,
}

impl BeerSensorySearch {
    pub fn new(sensory_json: &str) -> Result<Self, serde_json::Error> {
        let sensory: BeerSensory = serde_json::from_str(sensory_json)?;
        Ok(BeerSensorySearch { sensory })
    }

    // Find all descriptors matching a search term
    pub fn search_descriptors(&self, search_term: &str) -> Vec<BeerTasteMatch> {
        let mut matches = Vec::new();
        let search_lower = search_term.to_lowercase();

        for (cat_name, category) in &self.sensory.beer_tastes.categories {
            for (sub_name, subcategory) in &category.subcategories {
                for descriptor in &subcategory.descriptors {
                    if descriptor.to_lowercase().contains(&search_lower) {
                        matches.push(BeerTasteMatch {
                            category: cat_name.clone(),
                            subcategory: sub_name.clone(),
                            descriptor: descriptor.clone(),
                            notes: subcategory.notes.clone(),
                        });
                    }
                }
            }
        }
        matches
    }

    // Get all tastes in a specific category
    pub fn get_category_tastes(&self, category_name: &str) -> Option<&Category> {
        self.sensory.beer_tastes.categories.get(category_name)
    }

    // Find potential off-tastes (classes 5-8 typically indicate defects)
    pub fn identify_off_tastes(&self, descriptors: &[&str]) -> Vec<OffTasteMatch> {
        // This function now cannot use class_id, so we will match by category name
        let off_taste_categories = ["oxidized_stale", "phenolic", "sulfur", "fatty_acid"];
        let mut off_tastes = Vec::new();

        for descriptor in descriptors {
            let matches = self.search_descriptors(descriptor);
            for taste_match in matches {
                if off_taste_categories.contains(&taste_match.category.as_str()) {
                    off_tastes.push(OffTasteMatch {
                        descriptor: taste_match.descriptor.clone(),
                        category: taste_match.category.clone(),
                        severity: self.assess_severity(&taste_match.category),
                        possible_cause: self.get_possible_cause(&taste_match.category),
                    });
                }
            }
        }
        off_tastes
    }

    // Generate taste profile summary
    pub fn generate_taste_profile(&self, descriptors: &[&str]) -> TasteProfile {
        let mut profile = TasteProfile::default();

        for descriptor in descriptors {
            let matches = self.search_descriptors(descriptor);
            for taste_match in matches {
                match taste_match.category.as_str() {
                    "aromatic_fragrant_fruity_floral" => profile.aromatic_intensity += 1,
                    "cereal" | "caramelized_roasted" | "sweet" => profile.malty_intensity += 1,
                    "sour_acidic" => profile.sourness += 1,
                    "bitter" => profile.bitterness += 1,
                    "mouthfeel" => profile.mouthfeel_complexity += 1,
                    "oxidized_stale" | "phenolic" | "sulfur" | "fatty_acid" => profile.off_taste_count += 1,
                    _ => {},
                }
            }
        }
        profile
    }

    fn assess_severity(&self, category: &str) -> &'static str {
        match category {
            "oxidized_stale" => "Moderate",
            "phenolic" => "High",
            "sulfur" => "High",
            "fatty_acid" => "High",
            _ => "Low",
        }
    }

    fn get_possible_cause(&self, category: &str) -> &'static str {
        match category {
            "oxidized_stale" => "Oxygen exposure, staling, poor storage",
            "phenolic" => "Wild yeast, chlorinated water, cleaning residue",
            "sulfur" => "Bacterial contamination, poor yeast health",
            "fatty_acid" => "Bacterial infection, lipid oxidation",
            _ => "Various causes",
        }
    }
}

#[derive(Debug, Clone)]
pub struct BeerTasteMatch {
    pub category: String,
    pub subcategory: String,
    pub descriptor: String,
    pub notes: String,
}

#[derive(Debug)]
pub struct OffTasteMatch {
    pub descriptor: String,
    pub category: String,
    pub severity: &'static str,
    pub possible_cause: &'static str,
}

#[derive(Debug, Default)]
pub struct TasteProfile {
    pub aromatic_intensity: u32,
    pub malty_intensity: u32,
    pub sourness: u32,
    pub sweetness: u32,
    pub bitterness: u32,
    pub mouthfeel_complexity: u32,
    pub off_taste_count: u32,
}

// Example usage and testing
pub fn main() {
    let sensory_json = include_str!("beersensory.json"); // Your JSON file
    let sensory_search = BeerSensorySearch::new(sensory_json).expect("Failed to parse JSON");

    // Search for specific tastes
    let apple_tastes = sensory_search.search_descriptors("apple");
    println!("Apple-related tastes: {:#?}", apple_tastes);

    // Check for off-tastes
    let sample_descriptors = ["cardboard", "vinegar", "metallic", "banana"];
    let off_tastes = sensory_search.identify_off_tastes(&sample_descriptors);
    println!("Potential off-tastes: {:#?}", off_tastes);

    // Generate taste profile
    let profile = sensory_search.generate_taste_profile(&sample_descriptors);
    println!("Taste profile: {:#?}", profile);

    // Get specific category
    if let Some(aromatic) = sensory_search.get_category_tastes("aromatic_fragrant_fruity_floral") {
        println!("Aromatic category: {:#?}", aromatic);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_functionality() {
        let sensory_json = r#"{"beer_tastes":{"categories":{"aromatic_fragrant_fruity_floral":{"subcategories":{"fruity":{"descriptors":["apple","banana"],"notes":"Test"}}}}}}"#;

        let sensory_search = BeerSensorySearch::new(sensory_json).unwrap();
        let results = sensory_search.search_descriptors("apple");

        assert!(!results.is_empty());
        assert_eq!(results[0].descriptor, "apple");
    }

    #[test]
    fn test_off_taste_detection() {
        let sensory_json = r#"{"beer_tastes":{"categories":{"oxidized_stale":{"subcategories":{"papery":{"descriptors":["cardboard"],"notes":"Oxidation"}}}}}}"#;

        let sensory_search = BeerSensorySearch::new(sensory_json).unwrap();
        let off_tastes = sensory_search.identify_off_tastes(&["cardboard"]);

        assert!(!off_tastes.is_empty());
        assert_eq!(off_tastes[0].severity, "Moderate");
    }
}
