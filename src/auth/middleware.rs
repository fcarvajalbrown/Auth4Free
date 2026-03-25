// src/auth/middleware.rs

use super::models::*;
use std::future::Future;
use std::pin::Pin;
use tokio::task;

pub struct AuthMiddleware;

impl<B> tower::Service<B> for AuthMiddleware
where
    B: tower::Service<Request>,
{
    type Response = B::Response;
    type Error = B::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        B::poll_ready(self, cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        // Simulate authentication middleware logic
        let future = async move {
            // For now, just pass the request through
            req.into_inner().await
        };
        Box::pin(future)
    }
}

