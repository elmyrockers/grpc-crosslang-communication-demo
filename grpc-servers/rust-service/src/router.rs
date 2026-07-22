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


type HandlerFuture = Pin<Box<dyn std::future::Future<Output = Result<Bytes, Status>> + Send>>;
type HandlerFn = fn(Arc<UserService>, Bytes) -> HandlerFuture;

fn all_handler(svc: Arc<UserService>, buf: Bytes) -> HandlerFuture {
    Box::pin(async move { svc.all(&buf).await })
}
fn add_handler(svc: Arc<UserService>, buf: Bytes) -> HandlerFuture {
    Box::pin(async move { svc.add(&buf).await })
}
fn edit_handler(svc: Arc<UserService>, buf: Bytes) -> HandlerFuture {
    Box::pin(async move { svc.edit(&buf).await })
}
fn delete_handler(svc: Arc<UserService>, buf: Bytes) -> HandlerFuture {
    Box::pin(async move { svc.delete(&buf) })
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
            let handler: HandlerFn = match req.uri().path() {
                "/user_fb.UserService/All"    => all_handler,
                "/user_fb.UserService/Add"    => add_handler,
                "/user_fb.UserService/Edit"   => edit_handler,
                "/user_fb.UserService/Delete" => delete_handler,
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
    pub handler: HandlerFn,
}

impl tonic::server::UnaryService<FlatMessage> for UnaryHandler {
    type Response = FlatMessage;
    type Future   = Pin<Box<dyn std::future::Future<Output = Result<tonic::Response<FlatMessage>, Status>> + Send>>;

    fn call(&mut self, req: tonic::Request<FlatMessage>) -> Self::Future {
        let svc     = self.svc.clone();
        let handler = self.handler;
        Box::pin(async move {
            let bytes = handler(svc, req.into_inner().0).await?;
            Ok(tonic::Response::new(FlatMessage(bytes)))
        })
    }
}