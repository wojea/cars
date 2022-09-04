mod services;
use std::env;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let address = if args.len() > 1 {args[1].as_str()} else {"127.0.0.1"};
    let port = if args.len() > 2 {args[2].clone().parse::<u16>().unwrap()} else {8080};
    println!("{:?}", (address, port));
    HttpServer::new(|| {
        App::new()
            .service(services::calculateDisselUsageForDistance::calculateDisselUsageForDistance)
            .service(services::probabilityOfUnitInjectorFail::probabilityOfUnitInjectorFail)
    })
    .bind((address, port))?
    .run()
    .await
}
