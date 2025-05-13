use serde::{Deserialize, Serialize};

/// Represents a specific product variety with its PLU codes and category.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluItem {
    /// The specific name of the item, often including size or type.
    /// e.g., "Akane, small", "Mickey Lee", "Alfalfa Sprouts"
    pub name: String,

    /// List of PLU codes associated with this specific item.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub plu_codes: Vec<u32>,

    /// An ordered list representing the category hierarchy.
    /// e.g., ["Apple", "Akane"], ["Melon", "Watermelon", "Mickey Lee"], ["Alfalfa Sprouts"]
    pub category_path: Vec<String>,

    /// Optional alternative name(s).
    /// e.g., "Southern Rose, small", "Sugarbaby"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_name: Option<String>,

    /// Optional list of descriptive characteristics.
    /// e.g., ["seedless", "3-7 pounds"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristics: Vec<String>,

    /// Optional size description if explicitly mentioned (e.g., "small", "large")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

/// Holds the collection of all parsed PLU items.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PluCollection {
    pub items: Vec<PluItem>,
}

// Optional helper for creating items more easily during parsing
impl PluItem {
    pub fn new(
        name: String,
        plu_codes: Vec<u32>,
        category_path: Vec<String>,
        alternative_name: Option<String>,
        characteristics: Vec<String>,
        size: Option<String>,
    ) -> Self {
        PluItem {
            name,
            plu_codes,
            category_path,
            alternative_name,
            characteristics,
            size,
        }
    }
}
