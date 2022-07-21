//! Manages authentication into and out of Finch.

use std::time::Duration;

use aliri::{jwa, jwt};
use aliri_oauth2::Authority;
use aliri_tower::Oauth2Authorizer;

use crate::error::Result;

use self::{claims::CustomClaims, error::ErrorHandler};

pub const ISSUER: &str = "https://dev-sjbqm1dc.us.auth0.com/";
pub const AUDIENCE: &str = "https://finch.4343.ca/";

pub async fn construct_authorizer() -> Result<Oauth2Authorizer<CustomClaims, ErrorHandler>> {
    let authorizer = Oauth2Authorizer::new()
        .with_claims::<CustomClaims>()
        .with_error_handler(ErrorHandler);

    Ok(authorizer)
}

pub async fn construct_authority() -> Result<Authority> {
    let validator = jwt::CoreValidator::default()
        .add_approved_algorithm(jwa::Algorithm::RS256)
        .add_allowed_audience(jwt::Audience::from(AUDIENCE))
        .require_issuer(jwt::Issuer::from(ISSUER));

    let authority =
        Authority::new_from_url(format!("{}.well-known/jwks.json", ISSUER), validator).await?;

    authority.spawn_refresh(Duration::from_secs(600));

    Ok(authority)
}

pub mod scope {
    use aliri_axum::scope_guards;

    scope_guards! {
        type Claims = super::claims::CustomClaims;

        pub scope Member = "member";
        pub scope Mentor = "mentor";
        pub scope Superuser = "super";

        pub scope Anyone = *;
        pub scope Noone = [];
    }
}

pub mod claims {
    use aliri::jwt;
    use aliri_clock::UnixTime;
    use aliri_oauth2::oauth2;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct CustomClaims {
        iss: jwt::Issuer,
        aud: jwt::Audiences,
        sub: jwt::Subject,
        exp: UnixTime,
        iat: UnixTime,
        scope: oauth2::Scope,
        permissions: oauth2::Scope,
    }

    impl jwt::CoreClaims for CustomClaims {
        fn nbf(&self) -> Option<UnixTime> {
            None
        }
        fn exp(&self) -> Option<UnixTime> {
            Some(self.exp)
        }
        fn aud(&self) -> &jwt::Audiences {
            &self.aud
        }
        fn iss(&self) -> Option<&jwt::IssuerRef> {
            Some(&self.iss)
        }
        fn sub(&self) -> Option<&jwt::SubjectRef> {
            Some(&self.sub)
        }
    }

    impl oauth2::HasScope for CustomClaims {
        fn scope(&self) -> &oauth2::Scope {
            &self.scope
        }
    }
}

mod error {
    use aliri::error::JwtVerifyError;
    use axum::{http::Response, response::IntoResponse};

    #[derive(Clone, Copy)]
    pub struct ErrorHandler;

    impl aliri_tower::OnJwtError for ErrorHandler {
        type Body = axum::body::BoxBody;

        fn on_missing_or_malformed(&self) -> Response<Self::Body> {
            let (parts, ()) =
                aliri_tower::util::unauthorized("authorization token is missing or malformed")
                    .into_parts();

            (
                parts.status,
                parts.headers,
                "authorization token is missing or malformed\n",
            )
                .into_response()
        }

        fn on_no_matching_jwk(&self) -> Response<Self::Body> {
            let (parts, ()) =
                aliri_tower::util::unauthorized("token signing key (kid) is not trusted")
                    .into_parts();

            (
                parts.status,
                parts.headers,
                "token signing key (kid) is not trusted\n",
            )
                .into_response()
        }

        fn on_jwt_invalid(&self, error: JwtVerifyError) -> Response<Self::Body> {
            use std::fmt::Write;

            let mut header_description = String::new();
            let mut err: &dyn std::error::Error = &error;
            write!(&mut header_description, "{err}").unwrap();
            while let Some(next) = err.source() {
                write!(&mut header_description, ": {next}").unwrap();
                err = next;
            }

            let (parts, ()) = aliri_tower::util::unauthorized(&header_description).into_parts();

            let mut message = String::new();
            let mut err: &dyn std::error::Error = &error;
            write!(&mut message, "{err}\nDetails:\n").unwrap();
            while let Some(next) = err.source() {
                writeln!(&mut message, "\t{next}").unwrap();
                err = next;
            }

            (parts.status, parts.headers, message).into_response()
        }
    }
}
