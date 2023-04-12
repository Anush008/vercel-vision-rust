use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Description {
    pub tags: Vec<String>,
    pub captions: Vec<Caption>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Caption {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub description: Description
}
