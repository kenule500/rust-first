use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFailed,
    TicketDeleteFailIdNotFound { id: u64 },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        print!("->> {:<12} Error::into_response - {self:?}\n", "HANDLER");
        
        // 👇 I deleted the dead code here 👇
        
        match self {
            Error::LoginFailed => {
                let body: &str = "Login failed";
                (StatusCode::UNAUTHORIZED, body).into_response()
            }
            
            Error::TicketDeleteFailIdNotFound { id } => {
                let body = format!("Ticket with id {} not found", id);
                (StatusCode::NOT_FOUND, body).into_response()
            }
        } // 👈 No semicolon here! This match block returns the Response.
    }
}