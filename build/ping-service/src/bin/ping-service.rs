
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate hyper;

use std::error;
use std::env;

use hyper::{Body, Request, Response, Server};
use hyper::rt::{self, Future};
use hyper::service::service_fn_ok;

fn main() -> Result<(), Box<dyn error::Error>> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let app_id = env::var("APP_ID")?;

    let create_service = move || {
        let app_id = app_id.clone();
        service_fn_ok(move |_req: Request<Body>| {
            let body = Body::from(format!("app_id={}", app_id));
            Response::new(body)
        })
    };
    
    let server =
        Server::bind(&([0,0,0,0], 80).into())
            .serve(create_service)
            .map_err(|reason| error!("server error: {:?}", reason));

    rt::run(server);

    Ok(())
}

