// use actix_web::{
//     error,
//     http::{header::ContentType, StatusCode},
// };
use actix_web::{get, web, Responder, Result};
use chrono::Datelike;
//use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct FuelUsage {
    fuelUsage: f64,
}

// #[derive(Serialize)]
// struct ErrorMessage {
//     message: String,
// }

#[derive(Deserialize)]
struct FormData {
    distance: u32,
    yearOfProduction: u16,
    fuelUsagePer100KM: u16,
}

#[get("/calculateDisselUsageForDistance")]
async fn calculateDisselUsageForDistance(form: web::Query<FormData>) -> Result<impl Responder> {
    let FormData {
        distance,
        yearOfProduction,
        fuelUsagePer100KM,
    } = form.into_inner();
    let current_year = chrono::Utc::now().year();
    //first car was produced in 1886
    // if yearOfProduction < 1886 || i32::from(yearOfProduction) > current_year {
    //     HttpResponse::build(StatusCode::BAD_REQUEST)
    //         .insert_header(ContentType::json())
    //         .body(web::Json(ErrorMessage {
    //             message: "Invalid year of production",
    //         }))
    // }
    let base: f64 = 1.1;
    let multiplier: f64 = f64::powf(base, f64::from(current_year) - f64::from(yearOfProduction));
    let response = FuelUsage {
        fuelUsage: (f64::from(distance) / f64::from(fuelUsagePer100KM * 100)) * multiplier,
    };
    Ok(web::Json(response))
}
