use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        // Define different routes based on HTTP method and path
        (&hyper::Method::GET, "/") => {
            let response = Response::builder()
                .status(200)
                .header("Content-Type", "text/plain")
                .body(Body::from("Hello, World!"))
                .unwrap();

            Ok(response)
        }
        (&hyper::Method::GET, "/api/greet") => {
            let response = Response::builder()
                .status(200)
                .header("Content-Type", "text/plain")
                .body(Body::from("Greetings from the API!"))
                .unwrap();

            Ok(response)
        }
        // Handle 404 for unknown routes
        _ => {
            let response = Response::builder()
                .status(404)
                .header("Content-Type", "text/plain")
                .body(Body::from("Not Found"))
                .unwrap();

            Ok(response)
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();
    let make_svc = make_service_fn(|_conn| {
        let handle_request = service_fn(handle_request);
        async { Ok::<_, Infallible>(handle_request) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Server listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
