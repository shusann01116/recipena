use axum::{
    body::Body,
    extract::Request,
    response::{IntoResponse, Response},
};
use http::StatusCode;
use http_body_util::BodyExt;

pub async fn inspect_body(
    request: Request,
    f: impl FnOnce(&[u8]) -> Result<(), Response>,
) -> Result<Request, Response> {
    let (parts, body) = request.into_parts();

    let bytes = body
        .collect()
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to collect body: {err}"),
            )
                .into_response()
        })?
        .to_bytes();
    f(&bytes)?;

    Ok(Request::from_parts(parts, Body::from(bytes)))
}
