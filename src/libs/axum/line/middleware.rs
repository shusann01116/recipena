use crate::{libs::axum::server::AppState, prelude::*};
use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::{IntoResponse, Response},
};
use base64::{Engine, engine};
use hmac::{Hmac, Mac};
use http::StatusCode;
use sha2::Sha256;

use crate::libs::axum::util::inspect_body;

const LINE_SIGNATURE_HEADER: &str = "x-line-signature";

pub(crate) async fn verify_line_signature(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> std::result::Result<impl IntoResponse, Response> {
    let signature = get_line_signature(&headers).ok_or(StatusCode::UNAUTHORIZED.into_response())?;
    let request = inspect_body(request, |body| {
        verify_line_signature_inner(
            signature.as_bytes(),
            body,
            &state.config.line_channel_secret,
        )
        .map_err(|e| {
            tracing::trace!("failed to verify line signature: {:?}", e);
            (
                StatusCode::UNAUTHORIZED,
                format!("failed to verify line signature: {e}"),
            )
                .into_response()
        })
    })
    .await
    .map_err(|e| {
        tracing::trace!("{:?}", e);
        e
    })?;

    Ok(next.run(request).await)
}

fn get_line_signature(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(LINE_SIGNATURE_HEADER)
        .and_then(|v| v.to_str().ok())
}

fn verify_line_signature_inner(signature: &[u8], body: &[u8], channel_secret: &str) -> Result<()> {
    tracing::trace!(signature, body, "verify_line_signature_inner");

    let hash = {
        let mut mac = Hmac::<Sha256>::new_from_slice(channel_secret.as_bytes())?;
        mac.update(body);
        let hash = mac.finalize();
        let hash = hash.into_bytes();
        engine::general_purpose::STANDARD.encode(hash)
    };

    let result = hash.as_bytes();
    if result == signature {
        Ok(())
    } else {
        Err(Error::Generic("invalid line signature".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("QcVhNFsoYq8eKlh/3wPt/P3+TcL96Vu8a+JQHF4PbD4=", "test body" => matches Ok(()) ; "valid")]
    #[test_case("", "" => matches Err(_) ; "empty")]
    fn verify_line_signature_test(signature: &str, body: &str) -> crate::prelude::Result<()> {
        verify_line_signature_inner(signature.as_bytes(), body.as_bytes(), "channel_secret")
    }
}
