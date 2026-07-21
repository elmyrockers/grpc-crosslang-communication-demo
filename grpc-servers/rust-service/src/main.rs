#[path = "fbs/user_generated.rs"]
#[allow(warnings)]
mod user_generated;
#[path = "pb/user.rs"]
#[allow(warnings)]
mod user_pb;
mod codec;
mod router;
mod grpc_service;

use std::sync::Arc;
use grpc_service::UserService;
use router::UserRouter;
use user_pb::user_service_client::UserServiceClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Connect to the Go gRPC UserService
        let client = UserServiceClient::connect("http://[::1]:50053").await?;




    let addr = "[::1]:50052".parse()?;
    tracing::info!("UserService listening on {}", addr);

    tonic::transport::Server::builder()
        .add_service(UserRouter(Arc::new(UserService::new(client))))
        .serve(addr)
        .await?;

    Ok(())
}