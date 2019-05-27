extern crate hyper;
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::{service_fn, service_fn_ok};

extern crate futures;

use futures::future;
use hyper::{Method, StatusCode};
const PHRASE: &str = "Hello, World!";

fn hello_world(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from(PHRASE))
} 
type BoxFut = Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>;
fn echo(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => { 
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },
        (&Method::POST, "/echo") => {
            // we'll be back
        },
        _ => { 
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Box::new(future::ok(response))
}
fn main() {
    //地址
    let addr = ([127, 0, 0, 1], 3000).into();
 
    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    //处理方法
    let new_svc = || {
        // service_fn_ok converts our function into a `Service`
        service_fn_ok(hello_world)
    };
    //创建服务器 绑定地址 设置处理方法 和错误处理
    let server = Server::bind(&addr)
        .serve(|| service_fn(echo))
        .map_err(|e| eprintln!("server error: {}", e));

    // Run this server for... forever!
    //启动
    hyper::rt::run(server);
}
