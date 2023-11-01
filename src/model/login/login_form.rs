use serde::Deserialize;

#[derive(FromForm, Deserialize)]
pub struct LoginForm<'a> {
    pub username: &'a str,
    pub password: &'a str,
}