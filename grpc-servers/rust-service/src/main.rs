#[path = "fbs/user_generated.rs"]
#[allow(warnings)]
mod user_generated;

mod codec;
mod router;
mod grpc_service;

use std::sync::Arc;
use grpc_service::UserService;
use router::UserRouter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr = "[::1]:50052".parse()?;
    tracing::info!("UserService listening on {}", addr);

    tonic::transport::Server::builder()
        .add_service(UserRouter(Arc::new(UserService::new())))
        .serve(addr)
        .await?;

    Ok(())
}