use std::{collections::HashMap, error::Error};

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

impl Request {
    //TODO:: request parse
    pub(crate) fn parse(buffer: &[u8]) -> Result<Request, Box<dyn Error>> {
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);
        match req.parse(buffer)? {
            httparse::Status::Complete(amt) => amt,
            httparse::Status::Partial => {
                return Err("Request is incompete".into());
            }
        };
        todo!()
    }
}
