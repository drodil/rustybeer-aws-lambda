use rustybeer::calculators::diluting;

use simple_logger::SimpleLogger;
use lambda::{handler_fn, Context};
use serde::{Serialize, Deserialize};
use lambda_gateway::{LambdaRequest, LambdaResponse, LambdaResponseBuilder};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[derive(Deserialize, Debug)]
struct Input {
    #[serde(rename = "currentGravity")]
    current_gravity: f32,
    #[serde(rename = "currentVolume")]
    current_volume: f32,
    #[serde(rename = "targetVolume")]
    target_volume: f32
}

#[derive(Serialize, Debug)]
struct Output {
    gravity: f32
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(log::LevelFilter::Debug).init()?;
    lambda::run(handler_fn(calculate_dilution)).await?;
    Ok(())
}

async fn calculate_dilution(event: LambdaRequest<Input>, _c: Context) -> Result<LambdaResponse, Error> {
    let payload = event.body();
    let dilution_calc = diluting::Diluting{};
    let data = Output{
        gravity: dilution_calc.calculate_dilution(payload.current_gravity, payload.current_volume, payload.target_volume),
    };
    let response = LambdaResponseBuilder::new()
        .set_json_payload(data)
        .build();

    Ok(response)
}
