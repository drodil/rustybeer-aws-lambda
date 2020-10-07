use rustybeer::calculators::abv;

use lambda::{handler_fn, Context};
use lambda_gateway::{LambdaRequest, LambdaResponse, LambdaResponseBuilder};
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[derive(Deserialize, Debug)]
struct Input {
    #[serde(rename = "originalGravity")]
    original_gravity: f32,
    #[serde(rename = "finalGravity")]
    final_gravity: f32,
}

#[derive(Serialize, Debug)]
struct Output {
    abv: f32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()?;
    lambda::run(handler_fn(calculate_abv)).await?;
    Ok(())
}

async fn calculate_abv(event: LambdaRequest<Input>, _c: Context) -> Result<LambdaResponse, Error> {
    let payload = event.body();
    let abv_calc = abv::Abv {};
    let data = Output {
        abv: abv_calc.calculate_abv(payload.original_gravity, payload.final_gravity),
    };
    let response = LambdaResponseBuilder::new().set_json_payload(data).build();

    Ok(response)
}
