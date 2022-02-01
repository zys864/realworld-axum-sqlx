use crate::error::ErrorKind;
use crate::jwt::KEYS;
use axum::{
    body::Body,
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};
use futures_util::future::BoxFuture;
use http::{header::AUTHORIZATION, Request, Response, StatusCode};
use jsonwebtoken::{decode, Validation};
use tower_http::auth::AsyncAuthorizeRequest;

use crate::jwt::Claims;
#[derive(Clone, Copy)]
struct MyAuth;

impl<B> AsyncAuthorizeRequest<B> for MyAuth
where
    B: Send + Sync + 'static,
{
    type RequestBody = B;
    type ResponseBody = axum::body::Body;
    type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, mut request: Request<B>) -> Self::Future {
        Box::pin(async {
            if let Ok(_claims) = check_auth(request).await {
                Ok(request)
            } else {
                let unauthorized_response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::empty())
                    .unwrap();

                Err(unauthorized_response)
            }
        })
    }
}
async fn check_auth<B>(request: Request<B>) -> Result<Claims, ErrorKind>
where
    B: Send,
{
    // Extract the token from the authorization header
    let mut req = RequestParts::new(request);
    let TypedHeader(Authorization(bearer)) =
        TypedHeader::<Authorization<Bearer>>::from_request(&mut req)
            .await
            .map_err(|_| ErrorKind::TokenError)?;

    // Decode the user data
    let token_data =
        decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| ErrorKind::TokenError)?;

    Ok(token_data.claims)
}
