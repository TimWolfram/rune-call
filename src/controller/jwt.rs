use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::Validation;

use jsonwebtoken::get_current_timestamp;
use rocket::http::Cookie;
use rocket::http::CookieJar;
use crate::model::login::LoginToken;

const SECRET: &[u8] = b"pl@y3r53cr3t_is_v3ry_53cr3t_1236549871!";
const COOKIE_NAME: &str = "login_user_token";

impl LoginToken {
    /// Create a new JWT and store it in the cookies
    pub fn create(user_id: usize, cookies: &CookieJar<'_>) -> Result<(), &'static str> {
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
                cookies.add_private(Cookie::new(COOKIE_NAME, token.to_string()));
                Ok(())
            },
            Err(_) => Err("Error encoding token!"),
        };
    }

    /// Gets the user ID from the JWT stored in the cookies, and refreshes it
    pub fn try_refresh(cookies: &CookieJar<'_>) -> Result<usize, &'static str> {
        let user_id = LoginToken::from_cookies(cookies)?;
        LoginToken::create(user_id, cookies)?;
        return Ok(user_id);
    }

    /// Get the user_id from the JWT stored in the cookies
    pub fn from_cookies(cookies: &CookieJar<'_>) -> Result<usize, &'static str> {
        let jwt = cookies.get_private(COOKIE_NAME);
        if let Some(cookie) = jwt {
            let jwt: &str = &cookie.value();
            let decoding_key: &DecodingKey = &DecodingKey::from_secret(SECRET);
            let validation: &Validation = &Validation::default();
            let token_data = jsonwebtoken::decode::<LoginToken>(jwt, decoding_key, validation);
            
            return match token_data {
                Ok(token_data) => Ok(token_data.claims.user_id),
                Err(_) => Err("Player not logged in: invalid token!"),
            };
        } else {
            return Err("Player is not logged in!");
        }
    }
    
    pub fn remove_cookie(cookies: &CookieJar<'_>) {
        cookies.remove_private(Cookie::named(COOKIE_NAME));
    }
}
