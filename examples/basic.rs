use http_body_util::Empty;
use hyper::{body::Bytes, Request};
use tower::{client::Client, Service, StackBuilder};

#[tokio::main]
async fn main() {
    let client = Client::<Empty<Bytes>>::new();
    let svc = StackBuilder::new().service(client);

    let request = Request::get("http://google.com")
        .body(Empty::default())
        .unwrap();
    svc.call(request).await.unwrap();
}
