use serde::{Deserialize, Serialize};
use vercel_runtime::{
    http::bad_request, process_request, process_response, run_service, service_fn, Body, Error,
    Request, RequestExt, Response, ServiceBuilder, StatusCode,
};
use simple_runtime_demo::analyse_image;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub url: String,
}

#[derive(Serialize)]
pub struct APIError {
    pub message: &'static str,
    pub code: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        // disable printing the name of the module in every log line.
        .with_target(false)
        .init();

    // This allows to extend the tower service with more layers
    let handler = ServiceBuilder::new()
        .map_request(process_request)
        .map_response(process_response)
        .service(service_fn(handler));

    run_service(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    tracing::info!("Generating image description!");
    let payload = req.payload::<Payload>();

    match payload {
        Err(..) => bad_request(APIError {
            message: "Invalid payload",
            code: "invalid_payload",
        }),
        Ok(None) => bad_request(APIError {
            message: "No payload",
            code: "no_payload",
        }),
        Ok(Some(payload)) => {
            if let Ok(data) = analyse_image(payload.url) {
                Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(data.into(),
                )?)
            }
            else {
                Ok(Response::builder()
            .status(StatusCode::OK)
            .body("Invalid URL!".into())?)
            }
        
        }
    }
}
