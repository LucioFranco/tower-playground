use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use tokio::sync::Semaphore;

use crate::Service;

const MAX_WAITERS: usize = 32;

pub struct Limit<S> {
    inner: S,
    semaphore: Arc<Semaphore>,
    waiters: AtomicUsize,
}

impl<S> Limit<S> {
    pub fn new(inner: S, semaphore: Arc<Semaphore>) -> Self {
        Self {
            inner,
            semaphore,
            waiters: AtomicUsize::new(0),
        }
    }
}

impl<Req, S> Service<Req> for Limit<S>
where
    S: Service<Req>,
{
    type Res = S::Res;
    type Error = LimitError<S::Error>;

    async fn call(&self, req: Req) -> Result<Self::Res, Self::Error> {
        // let permit = self.semaphore.try_acquire() else {
        //     if self.waiters.load(Ordering::Relaxed) < MAX_WAITERS {
        //         self.waiters.fetch_add(1, Ordering::Relaxed);
        //         self.semaphore.acquire().await
        //     } else {
        //         return Err(LimitError::Overloaded);
        //     }
        // };
        //
        todo!()
    }
}

pub enum LimitError<E> {
    Svc(E),
    Overloaded,
}
