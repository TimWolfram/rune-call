// FROM: https://stackoverflow.com/a/69342225
// needed for CORS, without this the browser will block the request

use rocket::http::{Header, Method, Status};
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }
    
    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let res = response.status();
        println!("Host: {:?}, Origin: {:?}, Path: {:?}\nResponse: {:?}",
            request.headers().get_one("Host"),
            request.headers().get_one("Origin"),
            request.uri().path(),
            res.to_string()
        );
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:3000"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PUT, PATCH, OPTIONS, DELETE"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type, Authorization, Cache-Control"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        if request.method() == Method::Options {
            response.set_status(Status::Ok);
        }
    }
}