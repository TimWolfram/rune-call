use rocket::State;
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use crate::model::login::{LoginForm, User, LoginToken, Role};
use crate::repository::UserRepository;

#[get("/login", data="<form>")]
pub async fn login<'a>(user_repo: &State<UserRepository>,
    form: Json<LoginForm<'a>>,
    cookies: &'a CookieJar<'a>)
-> Result<Json<User>, &'static str> {
    let user = user_repo.get_by_username(form.username).await;
    if let None = user {
        return Err("User not found!");
    }
    let user = user.unwrap();
    if !crate::controller::password::verify_password(form.password, user.password_hash.as_str()) {
        return Err("Incorrect password!");
    }
    LoginToken::create(user.id, cookies)?;
    Ok(Json(user))
}
#[delete("/login")]
pub async fn logout(cookies: &CookieJar<'_>) -> Result<(), ()> {
    LoginToken::remove_cookie(cookies);
    Ok(())
}
#[post("/login", data="<form>")]
pub async fn register<'a>(user_repo: &'a State<UserRepository>,
    form: Json<LoginForm<'a>>,
    cookies: &CookieJar<'a>)
-> Result<Json<User>, &'a str> {
    if LoginToken::from_cookies(cookies).is_ok() {
        return Err("Cannot create account when user is already logged in!");
    }
    let username = form.username;
    if username.len() < 3 {
        return Err("Username must be at least 3 characters long!");
    }
    let password = form.password;
    if password.len() < 6 {
        return Err("Password must be at least 6 characters long!");
    }
    let user = user_repo.create_user(username, password, Role::Player).await?;
    LoginToken::create(user.id, cookies)?;
    Ok(Json(user))
}
#[delete("/login/<user_id>")]
pub async fn delete_user<'a>(user_id: usize,
    user_repo: &'a State<UserRepository>,
    cookies: &CookieJar<'a>)
-> Result<(), &'a str> {
    let login_user_id = LoginToken::from_cookies(cookies)?;
    let user = user_repo.get(user_id).await?;
    if user.role != Role::Admin
        && user.id != login_user_id {
            return Err("User can only delete their own account!");
    }
    user_repo.remove_user(user_id).await;
    LoginToken::remove_cookie(cookies);
    Ok(())
}