use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::time::Duration;
use std::{fs, io};

#[derive(Deserialize, Debug)]
pub struct HttpResponse {
    responseData: Option<TranslateResponse>,
    responseStatus: Option<u16>
}

#[derive(Deserialize, Debug)]
struct TranslateResponse {
    translatedText: String,
}

async fn translate_text(text: &str, api_key: &str, pair: &str) -> Result<String, String> {
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

fn get_pair() -> String {
    let mut pair = String::new();
    println!("Enter the language pair (e.g., 'en|es' for English to Spanish): ");
    io::stdin().read_line(&mut pair).expect("Failed to read line");
    pair.trim().to_string()
}

fn get_text() -> String {
    let mut text = String::new();
    println!("Enter the text to translate: ");
    io::stdin().read_line(&mut text).expect("Failed to read line");
    text.trim().to_string()
}

fn get_file_path() -> String {
    let mut text = String::new();
    println!("Enter the file path to translate: ");
    io::stdin().read_line(&mut text).expect("Failed to read line");
    text.trim().to_string()
}

#[tokio::main]
async fn main() { 
    let pair = get_pair();
    let path = get_file_path();
    let api_key = "YOUR_API_KEY"; // Replace with your actual API key
    //let text = get_text();

    let text = std::fs::read_to_string(&path).expect("Unable to read file");
    fs::write(&path, "").unwrap();
    match translate_text(&text, api_key, &pair).await {
        Ok(translated_text) => {
            fs::write(&path, translated_text).expect("Unable to write file");
            println!("Wszystko top");
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}