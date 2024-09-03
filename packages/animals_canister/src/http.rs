use candid::CandidType;
use serde::Deserialize;

pub fn default_headers() -> Vec<HeaderField> {
    vec![("Access-Control-Allow-Origin".to_string(), "*".to_string())]
}

pub type HeaderField = (String, String);

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HeaderField>,
    pub body: Vec<u8>,
    pub certificate_version: Option<u16>,
}

#[derive(CandidType, Debug, Clone)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    pub body: Vec<u8>,
    pub upgrade: Option<bool>,
}

pub fn http_error(message: &str) -> HttpResponse {
    HttpResponse {
        status_code: 500,
        body: message.as_bytes().to_vec(),
        headers: vec![],
        upgrade: None,
    }
}
