use crate::Service;

pub struct Retry<S, C> {
    inner: S,
    classify: C,
}

pub trait Classify<Req, Res, E> {
    fn is_retryable(&self, req: &Req, res: &mut Result<Res, E>) -> bool;
}

impl<S, C, Req> Service<Req> for Retry<S, C>
where
    S: Service<Req>,
    Req: Clone,
    C: Classify<Req, S::Res, S::Error>,
{
    type Res = S::Res;
    type Error = S::Error;

    async fn call(&self, req: Req) -> Result<Self::Res, Self::Error> {
        loop {
            let mut res = self.inner.call(req.clone()).await;

            if !self.classify.is_retryable(&req, &mut res) {
                return res;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{mock::MockError, Service};

    #[tokio::test]
    async fn smoke() {
        let (mock, mut handle) = crate::mock::pair::<(), ()>();

        let jh = tokio::spawn(async move {
            let req = handle.recv().await.unwrap();
            req.send_err("failed");

            let req = handle.recv().await.unwrap();
            req.send_res(());
        });

        let svc = Retry {
            inner: mock,
            classify: MockClassify,
        };

        svc.call(()).await.unwrap();

        jh.await.unwrap();
    }

    struct MockClassify;

    impl Classify<(), (), MockError> for MockClassify {
        fn is_retryable(&self, _req: &(), res: &mut Result<(), MockError>) -> bool {
            res.is_err()
        }
    }
}
