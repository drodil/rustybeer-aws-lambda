use rustybeer::calculators::abv;

use std::collections::HashMap;
use lambda::{handler_fn, Context};
use serde::{Serialize, Deserialize};
use serde::de::{DeserializeOwned, Deserializer};
use serde::de::Error as SerdeError;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

// Move these to separate package
#[derive(Serialize, Debug)]
struct LambdaResponse {
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
    #[serde(rename = "statusCode")]
    status_code: u16,
    headers: HashMap<String, String>,
    body: String,
}

pub fn deserialize<'a, T: DeserializeOwned, D: Deserializer<'a>>(
    deserializer: D,
) -> Result<T, D::Error> {
    serde_json::from_str(&String::deserialize(deserializer)?).map_err(SerdeError::custom)
}

#[derive(Deserialize, Debug)]
pub struct LambdaRequest<Data: DeserializeOwned> {
    #[serde(deserialize_with = "deserialize")]
    body: Data,
}

#[derive(Deserialize, Debug)]
struct Input {
    #[serde(rename = "originalGravity")]
    original_gravity: f32,
    #[serde(rename = "finalGravity")]
    final_gravity: f32,
}

#[derive(Serialize, Debug)]
struct Output {
    abv: f32
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::init_with_level(log::Level::Debug)?;
    lambda::run(handler_fn(calculate_abv)).await?;
    Ok(())
}

async fn calculate_abv(event: LambdaRequest<Input>, _c: Context) -> Result<LambdaResponse, Error> {
    let payload = event.body;
    let abv_calc = abv::Abv{};
    let data = Output{
        abv: abv_calc.calculate_abv(payload.original_gravity, payload.final_gravity),
    };
    let response = LambdaResponse{
        is_base64_encoded: false,
        status_code: 200,
        headers: HashMap::new(),
        body: serde_json::to_string(&data).unwrap(),
    };

    Ok(response)
}

