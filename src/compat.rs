use std::{sync::Arc, task::Poll};

type BoxFuture<'a, T> = std::pin::Pin<Box<dyn 'a + std::future::Future<Output = T>>>;

pub struct Compat<S> {
    inner: S,
}

impl<S, Request> tower_service::Service<Request> for Compat<S>
where
    S: crate::Service<Request> + Clone + 'static,
    Request: 'static
{
    type Response = S::Res;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let inner = self.inner.clone();

        Box::pin(async move { inner.call(req).await })
    }
}


