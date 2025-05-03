use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigData {
    pub osm: String,
    pub applist: HashMap<String, String>,
}
