use rustybeer::calculators::diluting;

use lambda::{handler_fn, Context};
use lambda_gateway::{LambdaRequest, LambdaResponse, LambdaResponseBuilder};
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[derive(Deserialize, Debug)]
struct Input {
    #[serde(rename = "currentGravity")]
    current_gravity: f32,
    #[serde(rename = "currentVolume")]
    current_volume: f32,
    #[serde(rename = "targetVolume")]
    target_volume: Option<f32>,
    #[serde(rename = "targetGravity")]
    target_gravity: Option<f32>,
}

#[derive(Serialize, Debug)]
struct Output {
    gravity: Option<f32>,
    volume: Option<f32>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()?;
    lambda::run(handler_fn(calculate_dilution)).await?;
    Ok(())
}

async fn calculate_dilution(
    event: LambdaRequest<Input>,
    _c: Context,
) -> Result<LambdaResponse, Error> {
    let payload = event.body();
    let dilution_calc = diluting::Diluting {};

    let mut gravity = None;
    let mut volume = None;
    if let Some(tv) = payload.target_volume {
        gravity = Some(dilution_calc.calculate_new_gravity(
            payload.current_gravity,
            payload.current_volume,
            tv,
        ));
    }
    if let Some(gr) = payload.target_gravity {
        volume = Some(dilution_calc.calculate_new_volume(
            payload.current_volume,
            payload.current_gravity,
            gr,
        ));
    }

    let data = Output {
        gravity: gravity,
        volume: volume,
    };

    let response = LambdaResponseBuilder::new().set_json_payload(data).build();

    Ok(response)
}
