
use actix_web::{ dev::{ServiceRequest, ServiceResponse}, body::MessageBody, Error};

use actix_web_lab::middleware::{ Next};
/// Example Middleware 1: Appends server header to trace requests
pub async fn append_trace_header_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    tracing::debug!("Route Middleware 1: Processing tracing headers intercept");
    let res = next.call(req).await?;
    Ok(res)
}

/// Example Middleware 2: Basic security validation gate simulation
pub async fn dummy_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    tracing::debug!("Route Middleware 2: Validating client security permissions");

    if req.headers().contains_key("x-api-token") {
        let res = next.call(req).await?;
        Ok(res)
    } else {
        Err(actix_web::error::ErrorUnauthorized("Missing mandatory API security parameters"))
    }
}
