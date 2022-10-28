use std::time::Duration;

use crate::Service;

pub struct Timeout<S> {
    inner: S,
    duration: Duration,
}

impl<Req, S> Service<Req> for Timeout<S>
where
    S: Service<Req>,
{
    type Res = S::Res;
    type Err = TimeoutError<S::Err>;

    async fn call(&self, req: Req) -> Result<Self::Res, Self::Err> {
        tokio::select! {
            res = self.inner.call(req) => {
                res.map_err(TimeoutError::Svc)
            }

             _ = tokio::time::sleep(self.duration) => {
                 Err(TimeoutError::Elapsed)
            }
        }
    }
}

pub enum TimeoutError<E> {
    Svc(E),
    Elapsed,
}
