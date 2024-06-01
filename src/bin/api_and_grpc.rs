/*
 * api_and_grpc.rs
 *
 * This Rust program initializes and starts both an Actix-web server for API REST requests and a
 * gRPC server using the tonic framework. It's meant as an example on how to modify the existing
 * Mighty Inference Server to serve up both interfaces if desired. Of course, the `BinaryClient`
 * implementation located at `/src/services/client/binary.rs` is not complete as this would be
 * completed only if the wish is to serve up both interfaces through a single binary.
 *
 * It only supports running in binary mode, controlled by the `--features binary` flag.
 *
 *
 * The program performs the following steps:
 * 1. Initializes logging based on environment settings.
 * 2. Loads application settings from a configuration file.
 * 3. Creates a binary client for communication based on the enabled `binary` feature flag.
 * 4. Configures and starts a gRPC server on the specified address and port set in `config.toml`.
 *
 * Usage:
 * To run the server:
 *   cargo run --bin api_and_grpc --features binary
 */

#![allow(unused_imports, unused)] // turned to silence clippy warnings due to using feature flags
use std::env;

use actix_web::{App, HttpServer, middleware, Responder, web};
use cfg_if::cfg_if;
use env_logger::Builder;
use futures::TryFutureExt;
use log::{error, info};
use tokio::signal;
use tonic::transport::Server;

use mighty_grpc::config::AppSettings;
#[cfg(feature = "binary")]
use mighty_grpc::services::clients::binary::BinaryClient;
use mighty_grpc::services::server_proxy::create_mighty_inference_server;

fn init_logging() {
    let mut builder = Builder::from_default_env();
    builder.filter(Some("h2"), log::LevelFilter::Warn);
    builder.init();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cfg_if! {
        if #[cfg(feature = "binary")] {
            let settings = AppSettings::new()?;
            env::set_var("RUST_LOG", &settings.logging.level);
            init_logging();

            let binary_client = BinaryClient::new();

            // gRPC server setup
            let grpc_addr = format!(
                "{}:{}",
                settings.grpc_server.address, settings.grpc_server.port
            )
            .parse()?;
            info!("gRPC Server listening on {}", grpc_addr);
            let grpc_service = create_mighty_inference_server(Box::new(binary_client));
            let grpc_future = Server::builder()
                .add_service(grpc_service)
                .serve(grpc_addr)
                .map_err(|e| anyhow::anyhow!(e));

            // REST API server setup
            let api_server = settings
                .api_server
                .as_ref()
                .expect("API Server configuration is missing");
            let http_addr = format!(
                "{}:{}",
                api_server.address, api_server.port
            );
            info!("API Server listening on {}", http_addr);
            let actix_future = HttpServer::new(move || {
                App::new()
                    .wrap(middleware::Logger::default())
                    .service(web::resource("/healthcheck").route(web::get().to(mock_health_check)))
                    .service(web::resource("/embeddings").route(web::get().to(mock_embeddings)))
            })
            .bind(http_addr)?
            .run()
            .map_err(|e| anyhow::anyhow!(e));

            // Handle shutdown signal
            let shutdown_signal = async {
                signal::ctrl_c()
                    .await
                    .expect("Failed to listen for shutdown signal");
                info!("Received shutdown signal");
            };

            // Run both servers concurrently and handle shutdown
            tokio::select! {
                res = grpc_future => {
                    if let Err(e) = res {
                        error!("gRPC server error: {:?}", e);
                    }
                },
                res = actix_future => {
                    if let Err(e) = res {
                        error!("Actix web server error: {:?}", e);
                    }
                },
                _ = shutdown_signal => {
                    info!("Shutting down API and gRPC servers");
                }
            }
        } else {
            panic!("Requires `--features binary` to run")
        }
    }

    Ok(())
}

async fn mock_health_check() -> impl Responder {
    web::Json("OK")
}

async fn mock_embeddings() -> impl Responder {
    web::Json(vec![1.0, 2.0, 3.0])
}
