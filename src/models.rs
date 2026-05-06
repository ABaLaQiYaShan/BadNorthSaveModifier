use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    #[serde(default)]
    pub unity: Option<Value>,
    #[serde(default)]
    pub records: Value,
}

impl SaveData {
    pub fn recruited_count(&self) -> usize {
        if let Some(records_obj) = self.records.as_object() {
            records_obj
                .values()
                .filter(|record| {
                    let class_name = record["class_type_name"]
                        .as_str()
                        .unwrap_or("");
                    if !class_name.contains("HeroDefinition") {
                        return false;
                    }

                    if let Some(members) = record["members"].as_array() {
                        for member in members {
                            if member["name"].as_str() == Some("recruited") {
                                return member["value"].as_bool().unwrap_or(false);
                            }
                        }
                    }
                    false
                })
                .count()
        } else {
            0
        }
    }
}

