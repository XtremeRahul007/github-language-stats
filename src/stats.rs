use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct LanguageStats {
    #[serde(rename = "nFiles")]
    pub n_files: u32,
    pub blank: u32,
    pub comment: u32,
    pub code: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Header {
    pub n_files: u32,
    pub n_lines: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClocStats {
    pub header: Header,
    #[serde(flatten)]
    pub languages: HashMap<String, LanguageStats>,
}

#[derive(Debug, Clone)]
pub struct LanguageEntry {
    pub name: String,
    pub stats: LanguageStats,
    pub percentage: f64,
}

pub fn parse_cloc_json(json: &str) -> Result<ClocStats, serde_json::Error> {
    let value: Value = serde_json::from_str(json)?;

    let header: Header = serde_json::from_value(value["header"].clone())?;

    let mut languages = HashMap::new();

    if let Value::Object(entries) = &value {
        for (name, stats) in entries {
            if name == "header" || name == "SUM" {
                continue;
            }

            let stats: LanguageStats = serde_json::from_value(stats.clone())?;

            languages.insert(name.clone(), stats);
        }
    }

    Ok(ClocStats { header, languages })
}
