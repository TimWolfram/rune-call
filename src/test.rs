use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::request::{self, Request, FromRequest};

/// A global atomic counter for generating IDs.
static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// A type that represents a request's ID.
pub struct RequestId(usize);

/// Returns the current request's ID, assigning one only as necessary.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r RequestId {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // The closure passed to `local_cache` will be executed at most once per
        // request: the first time the `RequestId` guard is used. If it is
        // requested again, `local_cache` will return the same value.
        request::Outcome::Success(request.local_cache(|| {
            RequestId(ID_COUNTER.fetch_add(1, Ordering::Relaxed))
        }))
    }
}

#[get("/")]
pub fn id(id: &RequestId) -> String {
    format!("This is request #{}.", id.0)
}