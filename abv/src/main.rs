use rustybeer::calculators::abv;

use std::collections::HashMap;
use lambda::{handler_fn, Context};
use serde::{Serialize, Deserialize};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[derive(Deserialize, Debug)]
struct Input {
    #[serde(rename = "originalGravity")]
    original_gravity: f32,
    #[serde(rename = "finalGravity")]
    final_gravity: f32,
}

#[derive(Serialize, Debug)]
struct LambdaResponse {
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
    #[serde(rename = "statusCode")]
    status_code: u16,
    headers: HashMap<String, String>,
    body: String,
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

async fn calculate_abv(event: Input, _c: Context) -> Result<LambdaResponse, Error> {
    let abv_calc = abv::Abv{};
    let data = Output{
        abv: abv_calc.calculate_abv(event.original_gravity, event.final_gravity),
    };
    let response = LambdaResponse{
        is_base64_encoded: false,
        status_code: 200,
        headers: HashMap::new(),
        body: serde_json::to_string(&data).unwrap(),
    };

    Ok(response)
}

