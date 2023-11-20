use crate::{responses::OpenAIResponse, requests};
use std::fs;
use base64;

pub fn encode_image(image_path: &str) -> String {
    let image_data = fs::read(image_path).expect("Failed to read image file");
    base64::encode(image_data)
}

pub fn get_gpt4_response(data: requests::OpenAIRequest, api_key: String) -> String {
    let json = serde_json::to_string(&data).unwrap();

    let client = reqwest::blocking::Client::new();
    let resp = client.post("https://api.openai.com/v1/chat/completions")
    .bearer_auth(api_key)
    .header("Content-Type", "application/json")
    .body(json.clone())
    .send();

    let test: &str = &resp.unwrap().text().unwrap();
    let result: OpenAIResponse = serde_json::from_str(test).unwrap();
    return result.choices[0].message.content.clone();
    
}