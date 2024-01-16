use rocket::State;
use rocket::http::{CookieJar, Status};
use rocket::serde::json::Json;
use crate::controller::password;
use crate::model::login::{LoginForm, User, LoginToken, Role};
use crate::repository::UserRepository;

type Error<'a> = (Status, &'a str);
type ObjectReturn<'a, T> = Result<Json<T>, Error<'a>>;
type EmptyReturn<'a> = Result<(), Error<'a>>;

#[get("/", data="<form>", format="json")]
pub async fn login<'a>(user_repo: &State<UserRepository>,
    form: Option<Json<LoginForm<'a>>>,
    cookies: &'a CookieJar<'a>)
-> ObjectReturn<'a, User> {
    if form.is_none() {
        println!("Login: no form; trying to get user id from cookies!");
        //if no form, check if user is logged in
        let user_id = LoginToken::from_cookies(cookies)?;
        let user = user_repo.get(user_id).await;
        match user {
            Ok(user) => return Ok(Json(user)),
            Err(_) => return Err((Status::InternalServerError, "User not found!")),
        }
    }
    let form = form.unwrap();
    let user: Result<User, (Status, &str)> = user_repo.get_by_username(form.username).await;
    let user = match user {
        Ok(user) => user,
        Err(_) => return Err((Status::NotFound, "User not found!")),
    };
    match password::verify_password(form.password, user.password_hash.as_str()){
        Ok(_) => {},
        Err(_) => return Err((Status::Unauthorized, "Wrong password!")),
    }
    LoginToken::create(user.id, cookies)?;
    Ok(Json(user))
}

#[delete("/")]
pub async fn logout(cookies: &CookieJar<'_>) -> EmptyReturn<'static> {
    // might want to be removing cookie/state on client side manually instead of using this
    LoginToken::from_cookies(cookies)?; // return error if not logged in
    LoginToken::remove_cookie(cookies); 
    Ok(())
}

#[post("/", data="<form>", format="json")]
pub async fn register<'a>(user_repo: &'a State<UserRepository>,
    form: Json<LoginForm<'a>>,
    cookies: &CookieJar<'a>)
-> ObjectReturn<'a, User> {
    let logged_in_user = LoginToken::from_cookies(cookies);
    let username = form.username;
    let password = form.password;
    println!("Registering user :\n\t{}\nwith password:\n\t{}", username, password);
    if let Ok(_user) = logged_in_user {
        let logged_in_user = user_repo.get(logged_in_user.unwrap()).await?;
        // check if user is admin to create another admin
        if let Role::Admin = logged_in_user.role {
            // admin has slightly stricter requirements to uname/pw
            if username.len() < 5 {
                return Err((Status::BadRequest, "Admin username must be at least 5 characters long!"));
            }
            if password.len() < 8 {
                return Err((Status::BadRequest, "Admin password must be at least 8 characters long!"));
            }
            let user = user_repo.create_user(username, password, Role::Admin).await?;
            return Ok(Json(user));
        }
    }
        
    if username.len() < 3 {
        return Err((Status::BadRequest, "Username must be at least 3 characters long!"));
    }
    if password.len() < 6 {
        return Err((Status::BadRequest, "Password must be at least 6 characters long!"));
    }
    let user = user_repo.create_user(username, password, Role::Player).await?;
    LoginToken::create(user.id, cookies)?;
    Ok(Json(user))
}

#[delete("/<user_id>", format="json")]
pub async fn delete_user<'a>(user_id: usize,
    user_repo: &'a State<UserRepository>,
    cookies: &CookieJar<'a>)
-> EmptyReturn<'a> {
    let login_user_id = LoginToken::from_cookies(cookies)?;
    let user = user_repo.get(user_id).await?;
    if user.role != Role::Admin && user.id != login_user_id {
        return Err((Status::Unauthorized, "User can only delete their own account!"));
    }
    user_repo.remove_user(user_id).await;
    LoginToken::remove_cookie(cookies);
    Ok(())
}