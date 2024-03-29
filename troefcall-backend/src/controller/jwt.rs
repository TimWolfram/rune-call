use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::Validation;

use jsonwebtoken::get_current_timestamp;
use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::http::SameSite;
use rocket::http::Status;
use crate::model::login::LoginToken;

const SECRET: &[u8] = b"pl@y3r53cr3t_is_v3ry_53cr3t_1236549871!";
const COOKIE_NAME: &str = "login_user_token";

type UserReturn<'a> = Result<usize, (Status, &'a str)>;
type EmptyReturn<'a> = Result<(), (Status, &'a str)>;

impl LoginToken {
    /// Create a new JWT and store it in the cookies
    pub fn create(user_id: usize, cookies: &CookieJar<'_>) -> UserReturn<'static> {
        let claims = LoginToken {
            user_id,
            exp: get_current_timestamp() + 
                60      // 1 minute
                * 60    // 1 hour
                * 24    // 1 day
                * 7,    // 1 week
        };
        let encoding_key: &EncodingKey = &EncodingKey::from_secret(SECRET);
        return match jsonwebtoken::encode(&Header::default(), &claims, encoding_key) {
            Ok(token) => {
                LoginToken::remove_cookie(cookies);
                let cookie = Cookie::build(COOKIE_NAME, token.to_string())
                    .same_site(SameSite::None)
                    // .secure(true)
                    .finish();
                cookies.add_private(cookie);
                Ok(user_id)
            },
            Err(_) => Err((Status::InternalServerError, "Error encoding token!")),
        };
    }

    /// Gets the user ID from the JWT stored in the cookies, and refreshes its duration (1 week)
    pub fn refresh_jwt(cookies: &CookieJar<'_>) -> UserReturn<'static> {
        let user_id = LoginToken::from_cookies(cookies)?;
        LoginToken::create(user_id, cookies)?;
        return Ok(user_id);
    }

    /// Get the user ID from the JWT stored in the cookies
    pub fn from_cookies(cookies: &CookieJar<'_>) -> UserReturn<'static> {
        let Some(cookie) = cookies.get_private(COOKIE_NAME) else {
            return Err((Status::Unauthorized, "User is not logged in; no JWT token found!"));
        };
        let jwt: &str = &cookie.value();
        let decoding_key: &DecodingKey = &DecodingKey::from_secret(SECRET);
        let validation: &Validation = &Validation::default();
        return match jsonwebtoken::decode::<LoginToken>(jwt, decoding_key, validation) {
            Ok(token_data) => Ok(token_data.claims.user_id),
            Err(err) => 
                match err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature 
                        => Err((Status::Unauthorized, "User not logged in: JWT token expired!")),
                    _ 
                        => Err((Status::Unauthorized, "User not logged in: invalid JWT token!")),
                },
        };
    }
    
    /// Remove the JWT from the cookies, effectively logging the user out
    pub fn remove_cookie(cookies: &CookieJar<'_>) {
        cookies.add_private(Cookie::build(COOKIE_NAME, "")
            .same_site(SameSite::None)
            // .secure(true) //HTTPS only
            .finish());
        cookies.remove_private(Cookie::named(COOKIE_NAME));
    }
}
