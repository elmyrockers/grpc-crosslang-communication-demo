use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use bytes::Bytes;
use http_body_util::{BodyExt, combinators::UnsyncBoxBody};
use tonic::codegen::http::{Request, Response};
use tonic::Status;

use crate::codec::{FlatCodec, FlatMessage};
use crate::grpc_service::UserService;

type BoxBody = UnsyncBoxBody<Bytes, Status>;

fn empty_body() -> BoxBody {
    UnsyncBoxBody::new(
        http_body_util::Empty::<Bytes>::new()
            .map_err(|_| Status::internal("empty body error"))
    )
}

#[derive(Clone)]
pub struct UserRouter(pub Arc<UserService>);

impl tonic::server::NamedService for UserRouter {
    const NAME: &'static str = "user_fb.UserService";
}

impl tonic::codegen::Service<Request<tonic::body::Body>> for UserRouter {
    type Response = Response<BoxBody>;
    type Error    = std::convert::Infallible;
    type Future   = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<tonic::body::Body>) -> Self::Future {
        let svc = self.0.clone();

        Box::pin(async move {
            let handler: fn(&UserService, &[u8]) -> Result<Bytes, Status> = match req.uri().path() {
                "/user_fb.UserService/All"    => |s, b| s.all(b),
                "/user_fb.UserService/Add"    => |s, b| s.add(b),
                "/user_fb.UserService/Edit"   => |s, b| s.edit(b),
                "/user_fb.UserService/Delete" => |s, b| s.delete(b),
                _ => return Ok(Response::builder()
                        .status(404)
                        .body(empty_body())
                        .unwrap()),
            };

            let mut grpc = tonic::server::Grpc::new(FlatCodec::default());
            let res = grpc.unary(UnaryHandler { svc, handler }, req).await;
            let res = res.map(|b| b.map_err(|e| Status::internal(e.to_string()))
                .boxed_unsync());
            Ok(res)
        })
    }
}

pub struct UnaryHandler {
    pub svc:     Arc<UserService>,
    pub handler: fn(&UserService, &[u8]) -> Result<Bytes, Status>,
}

impl tonic::server::UnaryService<FlatMessage> for UnaryHandler {
    type Response = FlatMessage;
    type Future   = Pin<Box<dyn std::future::Future<Output = Result<tonic::Response<FlatMessage>, Status>> + Send>>;

    fn call(&mut self, req: tonic::Request<FlatMessage>) -> Self::Future {
        let svc     = self.svc.clone();
        let handler = self.handler;
        Box::pin(async move {
            let bytes = handler(&svc, &req.into_inner().0)?;
            Ok(tonic::Response::new(FlatMessage(bytes)))
        })
    }
}