use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    pub content: String,
}
