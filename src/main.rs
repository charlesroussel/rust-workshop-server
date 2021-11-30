use actix_web::{App, HttpServer};
use color_eyre::eyre::Result;
use tracing::info;

use tracing_actix_web::TracingLogger;
use tracing_subscriber::filter::EnvFilter;

mod endpoints;

#[actix_web::main]
async fn main() -> Result<()> {
    setup()?;
    run().await?;
    Ok(())
}

fn setup() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    Ok(())
}

async fn run() -> Result<(), std::io::Error> {
    let server =
        HttpServer::new(||
            App::new()
                .wrap(TracingLogger::default())
                .service(endpoints::create_service())
        );
    let address = "0.0.0.0:8080";
    info!("Starting server on {}", address);
    server.bind(address)?
        .run()
        .await
}
