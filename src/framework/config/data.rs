use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ConfigData {
    applist: HashMap<String, String>,
}
