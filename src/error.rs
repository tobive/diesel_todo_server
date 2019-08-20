use diesel::result::Error;
use rocket::http::Status;
use rocket::response::{status, Responder};
use rocket::{response, Request};

pub type ApiResponse<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    AlreadyExists,
    FailedSaving,
    Diesel(Error),
}

/// Impl to implicitly convert io errors to our error
impl From<Error> for ApiError {
    fn from(err: Error) -> Self {
        ApiError::Diesel(err)
    }
}

impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        match self {
            ApiError::NotFound => status::NotFound(Some("Entry not found.")).respond_to(req),
            ApiError::AlreadyExists => {
                status::BadRequest(Some("The item with the same title already existed."))
                    .respond_to(req)
            }
            ApiError::Diesel(err) => {
                status::Custom(Status::InternalServerError, err.to_string()).respond_to(req)
            }
            _ => {
                status::Custom(Status::InternalServerError, "Internal server error").respond_to(req)
            }
        }
    }
}
