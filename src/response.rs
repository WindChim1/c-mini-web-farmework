use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Response {
    pub version: String,
    pub status_code: u8,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}
