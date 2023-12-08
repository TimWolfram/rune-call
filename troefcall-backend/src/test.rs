// use rocket::{Build, Rocket};
// use rocket::local::blocking::Client;
// use rocket::http::{Status, ContentType};

// use crate::repository::UserRepository;

// pub fn test_rocket(build: Rocket<Build>) {
//     // write to console
//     println!("Testing rocket instance");
//     let client = Client::tracked(build).expect("valid rocket instance");
//     let form_data = "{
//         \"username\": \"Admin\", 
//         \"password\": \"Adminpw!\"
//     }";
//     let mut response = client
//         .get("/login")
//         .header(ContentType::JSON)
//         .body(&form_data)
//         .dispatch();
//     assert_eq!(response.status(), Status::Ok);
//     assert_eq!(response.into_string().unwrap(), "Hello, world!");
// }