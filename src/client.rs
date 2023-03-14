use http_body::Body;

use hyper::{body::Incoming, Request, Response};
use hyper_util::{
    client::{
        connect::HttpConnector,
        legacy::{Client as HyperClient, Error as HyperError},
    },
    rt::TokioExecutor,
};

use crate::Service;

pub struct Client<B> {
    inner: HyperClient<HttpConnector, B>,
}

impl<B> Client<B>
where
    B: Body + Send,
    B::Data: Send,
{
    pub fn new() -> Self {
        let inner = HyperClient::builder(TokioExecutor::new()).build_http();
        Self { inner }
    }
}

impl<B> Service<Request<B>> for Client<B>
where
    B: Body + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    type Res = Response<Incoming>;
    type Error = HyperError;

    async fn call(&self, req: Request<B>) -> Result<Self::Res, Self::Error> {
        self.inner.request(req).await
    }
}
