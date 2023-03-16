use std::{io::Error, net::SocketAddr};

use hyper::server::conn::http1;
use tokio::net::TcpListener;

pub struct Server<S> {
    tcp: TcpListener,
    inner: S,
}

impl<S> Server<S> {
    pub async fn bind(addr: SocketAddr, inner: S) -> Result<Self, Error> {
        let tcp = TcpListener::bind(addr).await?;
        Ok(Server { inner, tcp })
    }

    pub fn addr(&self) -> Result<SocketAddr, Error> {
        self.tcp.local_addr()
    }

    pub async fn serve(self) -> Result<(), Error> {
        let builder = http1::Builder::new();

        todo!()
    }
}
