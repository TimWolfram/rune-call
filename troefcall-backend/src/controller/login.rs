use rocket::State;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use crate::controller::password;
use crate::model::login::{LoginForm, User, LoginToken, Role};
use crate::repository::UserRepository;

type Error<'a> = &'a str;

#[get("/login", data="<form>")]
pub async fn login<'a>(user_repo: &State<UserRepository>,
    form: Option<Json<LoginForm<'a>>>,
    cookies: &'a CookieJar<'a>)
-> Result<Redirect, Error<'static>> {
    if form.is_none() {
        LoginToken::try_refresh(cookies)?;
        return Err("No form provided!");
    }
    let form = form.unwrap();
    let user = user_repo.get_by_username(form.username).await?;
    password::verify_password(form.password, user.password_hash.as_str())?;
    LoginToken::create(user.id, cookies)?;
    Ok(Redirect::to("/"))
}

#[delete("/login")]
pub async fn logout(cookies: &CookieJar<'_>) -> Result<Redirect, ()> {
    LoginToken::remove_cookie(cookies);
    Ok(Redirect::to("/"))
}

#[post("/login", data="<form>")]
pub async fn register<'a>(user_repo: &'a State<UserRepository>,
    form: Json<LoginForm<'a>>,
    cookies: &CookieJar<'a>)
-> Result<Redirect, Error<'a>> {
    let logged_in_user = LoginToken::from_cookies(cookies);
    let username = form.username;
    let password = form.password;
    let logged_in_user = user_repo.get(logged_in_user.unwrap()).await;
    if let Ok(user) = logged_in_user {
        // check if user is admin to create another admin
        if let Role::Admin = user.role {
            // admin has slightly stricter requirements to uname/pw
            if username.len() < 5 {
                return Err("Admin username must be at least 5 characters long!");
            }
            if password.len() < 8 {
                return Err("Admin password must be at least 8 characters long!");
            }
            user_repo.create_user(username, password, Role::Admin).await?;
            return Ok(Redirect::to("/"));
        }
    }
        
    if username.len() < 3 {
        return Err("Username must be at least 3 characters long!");
    }
    if password.len() < 6 {
        return Err("Password must be at least 6 characters long!");
    }
    let user = user_repo.create_user(username, password, Role::Player).await?;
    LoginToken::create(user.id, cookies)?;
    Ok(Redirect::to("/"))
}

#[delete("/login/<user_id>")]
pub async fn delete_user<'a>(user_id: usize,
    user_repo: &'a State<UserRepository>,
    cookies: &CookieJar<'a>)
-> Result<(), Error<'a>> {
    let login_user_id = LoginToken::from_cookies(cookies)?;
    let user = user_repo.get(user_id).await?;
    if user.role != Role::Admin && user.id != login_user_id {
        return Err("User can only delete their own account!");
    }
    user_repo.remove_user(user_id).await;
    LoginToken::remove_cookie(cookies);
    Ok(())
}