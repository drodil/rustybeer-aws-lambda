// TODO: This could be replaced with lambda-http
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde::de::{DeserializeOwned, Deserializer};
use serde::de::Error as SerdeError;

pub fn deserialize<'a, T: DeserializeOwned, D: Deserializer<'a>>(
    deserializer: D,
) -> Result<T, D::Error> {
    serde_json::from_str(&String::deserialize(deserializer)?).map_err(SerdeError::custom)
}

#[derive(Deserialize, Debug)]
pub struct LambdaRequest<Data: DeserializeOwned> {
    #[serde(deserialize_with = "deserialize")]
    body: Data,
    headers: HashMap<String, String>,
    path: String,
}

impl<Data: DeserializeOwned> LambdaRequest<Data> {
    pub fn body(&self) -> &Data {
        &self.body
    }
}

#[derive(Serialize, Debug)]
pub struct LambdaResponse {
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
    #[serde(rename = "statusCode")]
    status_code: u16,
    headers: HashMap<String, String>,
    body: String,
}

pub struct LambdaResponseBuilder {
    status_code: u16,
    headers: HashMap<String, String>,
    body: String,
    is_base64_encoded: bool
}

impl LambdaResponseBuilder {
    pub fn new() -> Self {
        LambdaResponseBuilder {
            status_code: 200,
            headers: HashMap::new(),
            body: "".to_owned(),
            is_base64_encoded: false,
        }
    }

    pub fn set_status_code(mut self, status_code: u16) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn add_header<S: Into<String>>(mut self, name: S, value: S) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    pub fn clear_headers(mut self) -> Self {
        self.headers.clear();
        self
    }

    pub fn set_json_payload<D: Serialize>(mut self, data: D) -> Self {
        self.headers.entry("Content-Type".to_owned())
            .or_insert_with(|| "application/json".to_owned());
        self.body = serde_json::to_string(&data).unwrap();
        self
    }

    pub fn set_string_payload<S: Into<String>>(mut self, data: S) -> Self {
        self.body = data.into();
        self
    }

    pub fn build(self) -> LambdaResponse {
        LambdaResponse {
            is_base64_encoded: self.is_base64_encoded,
            status_code: self.status_code,
            headers: self.headers,
            body: self.body,
        }
    }
}
