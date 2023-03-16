#![allow(warnings)]
#![feature(async_fn_in_trait)]
#![feature(associated_return_type_bounds)]

pub mod box_service;
pub mod buffer;
pub mod client;
// pub mod compat;
pub mod limit;
pub mod mock;
pub mod retry;
pub mod server;
pub mod timeout;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub trait Service<Req> {
    type Res;
    type Error;

    async fn call(&self, req: Req) -> Result<Self::Res, Self::Error>;
}

// impl<T, Req> Service<Req> for &T
// where
//     T: Service<Req>,
// {
//     type Res = T::Res;
//     type Error = T::Error;

//     async fn call(&self, req: Req) -> Result<T::Res, T::Error> {
//         <T as Service<Req>>::call(self, req).await
//     }
// }

pub trait Layer<S> {
    type Service;

    fn layer(&self, inner: S) -> Self::Service;
}

pub struct StackBuilder<L> {
    layer: L,
}

impl StackBuilder<Identity> {
    pub fn new() -> Self {
        Self { layer: Identity {} }
    }
}

impl<L> StackBuilder<L> {
    pub fn push<T>(self, layer: T) -> StackBuilder<Stack<L, T>> {
        StackBuilder {
            layer: Stack {
                left: self.layer,
                right: layer,
            },
        }
    }

    pub fn service<S>(&self, svc: S) -> L::Service
    where
        L: Layer<S>,
    {
        self.layer.layer(svc)
    }
}

pub struct Identity {}

impl<S> Layer<S> for Identity {
    type Service = S;

    fn layer(&self, inner: S) -> Self::Service {
        inner
    }
}

pub struct Stack<L, R> {
    left: L,
    right: R,
}

impl<L, R, S> Layer<S> for Stack<L, R>
where
    L: Layer<S>,
    R: Layer<L::Service>,
{
    type Service = R::Service;

    fn layer(&self, inner: S) -> Self::Service {
        let l = self.left.layer(inner);
        self.right.layer(l)
    }
}

#[macro_export]
macro_rules! layer {
    (struct $name:ident for $service:ident<$service_generic:ident> {
        $(
            $field:ident: $field_ty:ty
        ),+ $(,)?
    }) => {
        struct $name {
            $($field: $field_ty),+
        }

        impl<S> crate::Layer<S> for $name {
            type Service = $service<$service_generic>;

            fn layer(&self, inner: S) -> Self::Service {
                $service {
                    inner,
                    $($field: self.$field),+
                }
            }
        }
    };
}
