use actix_web::{get, web, Responder, Result};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct FailProbability {
    failProbability: u8,
}

#[derive(Deserialize)]
struct FormData {
    VIN: String,
}

#[get("/probabilityOfUnitInjectorFails")]
async fn probabilityOfUnitInjectorFail(form: web::Query<FormData>) -> Result<impl Responder> {
    let mut rng = rand::thread_rng();
    let response = FailProbability {
        failProbability: rng.gen_range(0..101),
    };
    Ok(web::Json(response))
}
