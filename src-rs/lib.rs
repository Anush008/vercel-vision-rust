use serde_json::json;
use serde::{Deserialize, Serialize};

static AZURE_KEY:&'static str = env!("AZURE_KEY");
static AZURE_VISION_URL:&'static str = env!("AZURE_VISION_URL");

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

pub fn analyse_image(image_url: String) -> Result<String, Box<dyn std::error::Error>> {
    let body = json!({
        "url": image_url
    });
    let response = minreq::post(AZURE_VISION_URL)
    .with_header("Content-Type", "application/json")
    .with_header("Ocp-Apim-Subscription-Key", AZURE_KEY)
    .with_body(body.to_string()).send()?;
    let response: Data = serde_json::from_str(response.as_str()?)?;
    Ok(serde_json::to_string(&response)?)
}
