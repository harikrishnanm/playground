use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::web::Data;
use actix_web::Error;
use futures::future::{ok, Either, Ready};
use log::{debug, error, trace};
use std::collections::HashMap;

use crate::AppState;

pub struct Authenticate;

impl<S, B> Transform<S> for Authenticate
where
  S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
{
  type Request = ServiceRequest;
  type Response = ServiceResponse<B>;
  type Error = Error;
  type InitError = ();
  type Transform = CheckTokenMiddleware<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(CheckTokenMiddleware { service })
  }
}
pub struct CheckTokenMiddleware<S> {
  service: S,
}

impl<S, B> Service for CheckTokenMiddleware<S>
where
  S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
{
  type Request = ServiceRequest;
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

  fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
    self.service.poll_ready(cx)
  }

  fn call(&mut self, req: ServiceRequest) -> Self::Future {
    trace!("Request {:?}", req);

    match req.app_data::<Data<AppState>>() {
      Some(c) => match c.counter.lock() {
        Ok(mut v) => {
          debug!("Got lock");
          debug!("map {:?}", v);
        }
        Err(e) => error!("Couldnt lock"),
      },
      None => error!("No app state"),
    };
    debug!("{:?}", req.app_data::<Data<AppState>>());
    //let mut b = a.counter.lock().unwrap();
    //*b += 1;
    //Check if path and method combination needs to be authenticated
    let path = &req.path().to_string();
    debug!("Path requested {}", path);

    let method = &req.method().to_string();
    debug!("Method requested {}", method);
    Either::Left(self.service.call(req))
  }
}
