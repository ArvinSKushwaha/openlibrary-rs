use isahc::HttpClient;

mod auth;
// mod search;

#[derive(Debug)]
pub struct OpenLibrarySession {
    client: HttpClient,
    user_agent: String,
}

impl OpenLibrarySession {
    pub fn new(user_agent: String) -> Result<Self, isahc::Error> {
        Ok(Self {
            client: HttpClient::builder().cookies().build()?,
            user_agent,
        })
    }
}
