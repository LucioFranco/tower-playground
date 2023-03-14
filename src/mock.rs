use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

use crate::Service;

pub struct Mock<Req, Res> {
    tx: mpsc::Sender<Request<Req, Res>>,
}

pub struct Handle<Req, Res> {
    rx: mpsc::Receiver<Request<Req, Res>>,
}

pub struct Request<Req, Res> {
    req: Req,
    tx: oneshot::Sender<Result<Res, MockError>>,
}

pub fn pair<Req, Res>() -> (Mock<Req, Res>, Handle<Req, Res>) {
    let (tx, rx) = mpsc::channel(64);

    let mock = Mock { tx };
    let handle = Handle { rx };

    (mock, handle)
}

impl<Req, Res> Service<Req> for Mock<Req, Res> {
    type Res = Res;
    type Error = MockError;

    async fn call(&self, req: Req) -> Result<Self::Res, Self::Error> {
        let (tx, rx) = oneshot::channel();

        self.tx
            .try_send(Request { req, tx })
            .map_err(|_| MockError::Full)?;

        rx.await.map_err(|_| MockError::Dropped)?
    }
}

impl<Req, Res> Handle<Req, Res> {
    pub async fn recv(&mut self) -> Option<Request<Req, Res>> {
        self.rx.recv().await
    }
}

impl<Req, Res> Request<Req, Res> {
    pub fn get_ref(&self) -> &Req {
        &self.req
    }

    pub fn send_res(self, res: Res) {
        let _ = self.tx.send(Ok(res));
    }

    pub fn send_err(self, err: impl Into<String>) {
        let _ = self.tx.send(Err(MockError::Custom(err.into())));
    }
}

#[derive(Error, Debug)]
pub enum MockError {
    #[error("Mock message queue is full")]
    Full,
    #[error("Mock request dropped")]
    Dropped,
    #[error("Custom error: {0}")]
    Custom(String),
}
