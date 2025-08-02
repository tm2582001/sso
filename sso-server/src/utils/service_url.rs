use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    #[serde(rename = "serviceURL")]
    pub service_url: Option<String>,
}