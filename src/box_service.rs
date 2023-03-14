use crate::Service;

type BoxFuture<'a, T> = std::pin::Pin<Box<dyn 'a + std::future::Future<Output = T>>>;

pub struct BoxService<'a, Req, Res, Err> {
    b: Box<dyn 'a + DynService<Req, Res = Res, Error = Err>>,
}

impl<'a, Req, Res, Err> BoxService<'a, Req, Res, Err> {
    pub fn new<T>(service: T) -> Self
    where
        T: 'a + Service<Req, Res = Res, Error = Err>,
    {
        Self {
            b: Box::new(service),
        }
    }
}

impl<'a, Req, Res, Err> Service<Req> for BoxService<'a, Req, Res, Err> {
    type Res = Res;
    type Error = Err;

    async fn call(&self, req: Req) -> Result<Res, Err> {
        self.b.call(req).await
    }
}

trait DynService<Req> {
    type Res;
    type Error;

    fn call<'a>(&'a self, req: Req) -> BoxFuture<'a, Result<Self::Res, Self::Error>>
    where
        Req: 'a;
}

impl<T, Req> DynService<Req> for T
where
    T: Service<Req>,
{
    type Res = <T as Service<Req>>::Res;
    type Error = <T as Service<Req>>::Error;

    fn call<'a>(&'a self, req: Req) -> BoxFuture<'a, Result<Self::Res, Self::Error>>
    where
        Req: 'a,
    {
        Box::pin(self.call(req))
    }
}

#[cfg(test)]
mod tests {
    use crate::{mock, Service};

    use super::BoxService;

    #[tokio::test]
    async fn smoke() {
        let (mock, mut handle) = mock::pair::<(), ()>();

        let jh = tokio::spawn(async move {
            let res = handle.recv().await.unwrap();
            res.send_res(());
        });

        let svc = BoxService::new(mock);

        svc.call(()).await.unwrap();

        jh.await.unwrap();
    }
}
