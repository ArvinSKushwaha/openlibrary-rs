use crate::OpenLibrarySession;
use isahc::{http::StatusCode, AsyncBody, Request, Response};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    pub access: String,
    pub secret: String,
}

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("Isahc HTTP Error")]
    IsahcHttpError(#[from] isahc::http::Error),
    #[error("Isahc Error")]
    IsahcError(#[from] isahc::Error),
    #[error("Failed to Login: {0:?}")]
    FailedLogin(Response<AsyncBody>),
}

impl OpenLibrarySession {
    pub async fn login(&mut self, auth: &AuthRequest) -> Result<(), LoginError> {
        let request = Request::builder()
            .uri("https://openlibrary.org/account/login")
            .method("POST")
            .header("User-Agent", &*self.user_agent)
            .header("Content-Type", "application/json")
            .body(serde_json::to_vec(auth).unwrap())?;

        let response = self.client.send_async(request).await?;

        match response.status() {
            StatusCode::OK => Ok(()),
            _ => Err(LoginError::FailedLogin(response)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::OpenLibrarySession;

    use super::AuthRequest;

    #[test]
    pub fn test_login() {
        let mut sess =
            OpenLibrarySession::new("OpenLibraryApiRs/0.1 me@arvinsk.org".to_string()).unwrap();
        smol::block_on(sess.login(&AuthRequest {
            access: std::env::var("OPENLIBRARY_ACCESS").unwrap(),
            secret: std::env::var("OPENLIBRARY_SECRET").unwrap(),
        }))
        .unwrap();

        println!("{sess:?}");
        println!("{:?}", sess.client.cookie_jar());
    }
}
