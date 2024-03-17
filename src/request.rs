use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::util::HttpMethod;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub method: HttpMethod,
    pub uri: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}
