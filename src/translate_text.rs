use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
pub struct HttpResponse {
    responseData: Option<TranslateResponse>,
    responseStatus: Option<u16>
}

#[derive(Deserialize, Debug)]
struct TranslateResponse {
    translatedText: String,
}

pub async fn translate_text(text: &str, api_key: &str, pair: &str) -> Result<String, String> {
    let client = Client::new();

    let response = client.get("https://api.mymemory.translated.net/get")
    .query(&[("q", text), ("langpair", pair), ("key", api_key)])
        .timeout(Duration::new(30, 0))
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status() == StatusCode::OK {
                let full_response: HttpResponse = resp.json().await.unwrap();
                if let Some(translated_response) = full_response.responseData {
                    Ok(translated_response.translatedText)
                } else {
                    Err("No translation found".to_string())
                }
            } else {
                Err(format!("Error: {}", resp.status()))

            }
        }
        Err(_) => {
            Err("Failed to send request".to_string())
        }
    }
}