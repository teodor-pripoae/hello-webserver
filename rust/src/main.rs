use dropshot::endpoint;
use dropshot::ApiDescription;
use dropshot::ConfigDropshot;
use dropshot::ConfigLogging;
use dropshot::ConfigLoggingLevel;
use dropshot::HttpError;
use dropshot::HttpServerStarter;
use dropshot::Path;
use dropshot::RequestContext;
use hyper::http;
use hyper::Body;
use hyper::Response;
use hyper::StatusCode;
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct ReqCtx {}

#[derive(Deserialize, JsonSchema)]
pub struct PathArgs {
    path: String,
}

#[endpoint {
    method = GET,
    path = "/{path}",
}]
async fn wildcard_route(
    _rqctx: Arc<RequestContext<ReqCtx>>,
    path_params: Path<PathArgs>,
) -> Result<Response<Body>, HttpError> {
    let path = path_params.into_inner().path;

    let msg = format!("hello, you've hit /{}", path);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "text/plain")
        .body(msg.into())?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // Set up a logger.
    let log = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Info,
    }
    .to_logger("hello-webserver")
    .map_err(|e| e.to_string())?;

    let req_ctx = ReqCtx {};

    // Describe the API.
    let mut api = ApiDescription::new();
    api.register(wildcard_route).map_err(|e| e.to_string())?;
    // Register API functions -- see detailed example or ApiDescription docs.

    // Start the server.
    let server = HttpServerStarter::new(
        &ConfigDropshot {
            bind_address: addr.as_str().parse().unwrap(),
            request_body_max_bytes: 1024,
            tls: None,
        },
        api,
        req_ctx,
        &log,
    )
    .map_err(|error| format!("failed to start server: {}", error))?
    .start();

    server.await
}
