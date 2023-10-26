use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::Validation;
use rocket::http::CookieJar;
use crate::model::PlayerToken;

const SECRET: &[u8; 12] = b"pl4y3r53cr3t";
const E_KEY: &EncodingKey = &EncodingKey::from_secret(SECRET);
const D_KEY: &DecodingKey = &DecodingKey::from_secret(SECRET);
const VALIDATION: &Validation = &Validation::default();
const COOKIE_NAME: &str = "token";

impl PlayerToken {
    pub fn encode(player_id: usize) -> Result<&'static str, &'static str> {
        let claims = PlayerToken {
            exp: 10000,
            player_id: player_id,
        };
        let token = jsonwebtoken::encode(&Header::default(), &claims, E_KEY);
        return match token {
            Ok(token) => Ok(token.as_str()),
            Err(_) => Err("Error encoding token!"),
        };
    }

    pub fn from_cookies<'a>(cookies: &CookieJar<'_>) -> Result<PlayerToken, &'a str> {
        let jwt = cookies.get_private(COOKIE_NAME);
        if let Some(cookie) = jwt {
            let jwt: &str = &cookie.value();
            let token_data = jsonwebtoken::decode::<PlayerToken>(jwt, D_KEY, VALIDATION);
            return match token_data {
                Ok(token_data) => Ok(token_data.claims),
                Err(_) => Err("Invalid token!"),
            };
        } else {
            return Err("No cookie provided");
        }
    }
}