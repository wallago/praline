//! Bearer-token auth for the protected routes.

use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{HeaderValue, StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};

/// Middleware: lets the request through only if it carries
/// `Authorization: Bearer <token>`, else answers 401.
pub(super) async fn require(
    State(token): State<Arc<str>>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if authorized(request.headers().get(AUTHORIZATION), &token) {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

/// Whether `header` is `Bearer <token>`.
fn authorized(header: Option<&HeaderValue>, token: &str) -> bool {
    header
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .is_some_and(|given| same(given.as_bytes(), token.as_bytes()))
}

/// Compares in time independent of where the bytes differ, so response
/// timing doesn't leak how much of a guess was right.
fn same(left: &[u8], right: &[u8]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .fold(0, |diff, (l, r)| diff | (l ^ r))
            == 0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::right_token(Some("Bearer s3cret"), true)]
    #[case::wrong_token(Some("Bearer s3creT"), false)]
    #[case::prefix_of_token(Some("Bearer s3cr"), false)]
    #[case::no_scheme(Some("s3cret"), false)]
    #[case::other_scheme(Some("Basic s3cret"), false)]
    #[case::no_header(None, false)]
    fn checks_the_header(#[case] header: Option<&'static str>, #[case] expected: bool) {
        let header = header.map(HeaderValue::from_static);
        assert_eq!(authorized(header.as_ref(), "s3cret"), expected);
    }
}
