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
    ///parse http Request
    ///TODO::  don't  use hattparse lib,and do it by self
    pub(crate) fn parse(buffer: &[u8]) -> Result<Request, Box<dyn Error>> {
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);
        let res = match req.parse(buffer)? {
            httparse::Status::Complete(amt) => amt,
            httparse::Status::Partial => {
                return Err("Request is incompete".into());
            }
        };
        let method = match req.method.ok_or("Method not found")? {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            _ => HttpMethod::OTHER(req.method.unwrap().to_string()),
        };
        let uri = req.path.ok_or("URI not found")?.to_string();
        let version = req.version.ok_or("Version not found")?.to_string();

        let mut headers = HashMap::new();

        req.headers
            .iter()
            .try_for_each(|header| -> Result<(), Box<dyn Error>> {
                headers.insert(
                    header.name.to_string(),
                    String::from_utf8(header.value.to_vec())?,
                );
                Ok(())
            })?;

        let body = if res < buffer.len() {
            Some(String::from_utf8(buffer[res..].to_vec())?)
        } else {
            None
        };
        Ok(Self {
            method,
            uri,
            version,
            headers,
            body,
        })
    }
}
