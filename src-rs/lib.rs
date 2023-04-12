mod data;
use serde_json::json;

static AZURE_KEY:&'static str = env!("AZURE_KEY");
static AZURE_VISION_URL:&'static str = env!("AZURE_VISION_URL");

pub fn analyse_image(image_url: String) -> Result<String, Box<dyn std::error::Error>> {
    let body = json!({
        "url": image_url
    });
    let response = minreq::post(AZURE_VISION_URL)
    .with_header("Content-Type", "application/json")
    .with_header("Ocp-Apim-Subscription-Key", AZURE_KEY)
    .with_body(body.to_string()).send()?;
    let response: data::Data = serde_json::from_str(response.as_str()?)?;
    Ok(serde_json::to_string(&response)?)
}
