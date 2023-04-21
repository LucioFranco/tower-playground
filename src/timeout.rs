use std::time::Duration;

pub use tokio::time::error::Elapsed;

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
    type Error = Elapsed;

    async fn call(&self, req: Req) -> Result<Self::Res, Self::Error> {
        // tokio::time::timeout(self.duration, self.inner.call(req))
        //     .await
        todo!()
    }
}

crate::layer! {
    struct TimeoutLayer for Timeout<S> {
        duration: Duration,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeout() {
        let (mock, mut handle) = crate::mock::pair::<(), ()>();
       let layer = TimeoutLayer { duration: Duration::from_secs(1), }; 
    }
}
