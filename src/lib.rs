#![allow(incomplete_features)]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![feature(return_position_impl_trait_in_trait)]

pub mod mock;
pub mod retry;
pub mod timeout;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub trait Service<Req> {
    type Res;
    type Err;

    async fn call(&self, req: Req) -> Result<Self::Res, Self::Err>;
}
