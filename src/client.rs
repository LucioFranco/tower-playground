use crate::Service;
use hyper::{
    body::{Body, Incoming},
    client::conn::http1::SendRequest,
    Request, Response,
};

pub struct Client<B> {
    sr: SendRequest<B>,
}

impl<B> Service<Request<B>> for Client<B>
where
    B: Body + 'static,
{
    type Res = Response<Incoming>;
    type Err = hyper::Error;

    async fn call(&self, req: Request<B>) -> Result<Self::Res, Self::Err> {
        // self.sr.ready().await?;

        // self.sr.send_request(req).await
        todo!()
    }
}
