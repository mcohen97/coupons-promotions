
use futures::{future, Stream, IntoFuture};
use crate::server::{ApiError, ApiResult};
use actix_web::dev::{Payload};
use lazy_static::lazy_static;
use jsonwebtoken::{Validation, decode};
use actix_web::{FromRequest, HttpResponse, HttpRequest};
use actix_web::web::Bytes;
use actix_web::error::PayloadError;

use std::collections::HashSet;
use std::iter::FromIterator;

const ADMIN_PERM: &str = "ADMIN";
const HEADER: &str = "Authorization";
lazy_static! {
    static ref SECRET: String = std::env::var("SECRET").expect("Missing SECRET");
}

pub struct Authorization {
    permissions: HashSet<String>,
    pub organization_id: String
}

impl FromRequest for Authorization {
    type Error = HttpResponse;
    type Future = future::FutureResult<Self, Self::Error>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload<Box<dyn Stream<Item=Bytes, Error=PayloadError>>>) -> Self::Future {
        val(req).into_future()
    }
}

fn val(req: &HttpRequest) -> Result<Authorization, HttpResponse> {
    let header = req.headers()
        .get(HEADER)
        .ok_or(HttpResponse::Unauthorized().finish())?
        .to_str()
        .unwrap_or("");
    if !header.starts_with("Bearer") {
        return Err(HttpResponse::Unauthorized().finish());
    }
    let split: Vec<&str> = header.split_ascii_whitespace().collect();
    let token = decode::<Claims>(split[1], SECRET.as_bytes(), &Validation { validate_exp: false, ..Validation::default() })
        .map_err(|e| {
            info!("Authentication failed: {}", e);
            HttpResponse::Unauthorized().finish()
        })?;
    let organization_id = token.claims.organization_id.clone();
    let permissions = HashSet::from_iter(token.claims.permissions.into_iter());

    Ok(Authorization { permissions, organization_id })
}

impl Authorization {
    pub fn validate(auth: &Option<Authorization>, permissions: &Vec<&str>) -> ApiResult<String> {
        match auth {
            Some(auth) => {
                if permissions.into_iter().any(|&p| auth.permissions.contains(p)) {
                    Ok(auth.organization_id.clone())
                }
                else {
                    Err(ApiError::Unauthorized)
                }
            }
            None => Err(ApiError::Unauthorized)
        }

    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub permissions: Vec<String>,
    pub organization_id: String
}