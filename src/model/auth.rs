use actix_web::{
    dev::Payload, error::ErrorUnauthorized, http::header, web, Error as ActixWebError, FromRequest,
    HttpRequest,
};
use jsonwebtoken::{
    decode, errors::Error as JwtError, Algorithm, DecodingKey, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

#[derive(Serialize, Deserialize)]
pub struct AuthenticationToken {
    pub id: usize,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    id: usize,
    exp: usize,
}

impl FromRequest for AuthenticationToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // get auth token from authorization header
        let auth_header: Option<&header::HeaderValue> = req.headers().get(header::AUTHORIZATION);
        let auth_token: &str = auth_header.unwrap().to_str().unwrap_or("");
        if auth_token.is_empty() {
            return ready(Err(ErrorUnauthorized("Invalid auth token!")));
        }

        let secret: String = req.app_data::<web::Data<String>>().unwrap().to_string();
        // decode the token with secret
        let decode: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
            &auth_token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        );
        // return authentication token
        match decode {
            Ok(token) => ready(Ok(AuthenticationToken {
                id: token.claims.id,
            })),
            Err(_) => ready(Err(ErrorUnauthorized("Unauthorized"))),
        }
    }
}
