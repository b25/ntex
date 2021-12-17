use std::{io, marker::PhantomData, task::Context, task::Poll};

use crate::http::h1::Codec;
use crate::http::request::Request;
use crate::io::Io;
use crate::{util::Ready, Service, ServiceFactory};

pub struct UpgradeHandler<F>(PhantomData<F>);

impl<F> ServiceFactory for UpgradeHandler<F> {
    type Config = ();
    type Request = (Request, Io<F>, Codec);
    type Response = ();
    type Error = io::Error;
    type Service = UpgradeHandler<F>;
    type InitError = io::Error;
    type Future = Ready<Self::Service, Self::InitError>;

    #[inline]
    fn new_service(&self, _: ()) -> Self::Future {
        unimplemented!()
    }
}

impl<F> Service for UpgradeHandler<F> {
    type Request = (Request, Io<F>, Codec);
    type Response = ();
    type Error = io::Error;
    type Future = Ready<Self::Response, Self::Error>;

    #[inline]
    fn poll_ready(&self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    fn call(&self, _: Self::Request) -> Self::Future {
        unimplemented!()
    }
}
