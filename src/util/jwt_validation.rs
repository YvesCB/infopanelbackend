use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Header, TokenData, Validation};
use reqwest::get;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum ValidationError {
    NoKey,
    MissingEnvVar(String),
    JwtError(jsonwebtoken::errors::Error),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::NoKey => write!(f, "Cannot fetch jwk from url."),
            ValidationError::MissingEnvVar(v) => write!(f, "Missing .env var: {v}"),
            ValidationError::JwtError(e) => write!(f, "{e}"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct JWK {
    kty: String,
    #[serde(rename = "use")]
    use_field: String,
    kid: String,
    x5t: String,
    n: String,
    e: String,
    x5c: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct JWKS {
    keys: Vec<JWK>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // Add required claims like 'sub', 'exp', etc.
    sub: String,
    exp: usize,
    aud: String,
}

async fn fetch_public_key(jwks_url: &str, kid: &str) -> Option<JWK> {
    let response = get(jwks_url).await.ok()?;
    let jwks: JWKS = response.json().await.ok()?;
    jwks.keys.into_iter().find(|key| key.kid == kid)
}

fn decode_token_header(token: &str) -> Result<Header, jsonwebtoken::errors::Error> {
    let header = decode_header(token)?;
    Ok(header)
}

fn validate_token(
    token: &str,
    public_key: &str,
    audience: &str,
) -> Result<TokenData<Claims>, ValidationError> {
    let decoding_key = DecodingKey::from_rsa_pem(public_key.as_ref()).expect("Invalid public key");
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[audience]);
    match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(token_data) => Ok(token_data),
        Err(e) => Err(ValidationError::JwtError(e)),
    }
}

pub async fn validate(token: &str) -> Result<TokenData<Claims>, ValidationError> {
    let jwks_url = &std::env::var("JWKS_URL")
        .map_err(|_| ValidationError::MissingEnvVar(String::from("JWKS_URL")))?;

    let audience = &std::env::var("JWT_AUD")
        .map_err(|_| ValidationError::MissingEnvVar(String::from("JWT_AUD")))?;

    // Decode the token header to get the `kid`
    let header = match decode_token_header(token) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Failed to decode token header: {}", e);
            return Err(ValidationError::JwtError(e));
        }
    };

    let kid = header.kid.unwrap();

    // Fetch the public key from JWKS endpoint
    let jwk = match fetch_public_key(jwks_url, &kid).await {
        Some(jwk) => jwk,
        None => {
            eprintln!("Could not get matching public key");
            return Err(ValidationError::NoKey);
        }
    };

    let mut key = String::new();
    key.push_str("-----BEGIN CERTIFICATE-----\n");
    key.push_str(jwk.x5c.first().unwrap());
    key.push_str("\n-----END CERTIFICATE-----\n");

    validate_token(token, &key, audience)
}
