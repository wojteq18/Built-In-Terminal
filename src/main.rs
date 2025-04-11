use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct MyMemoryResponse {
   // responseData: Option<TranslateResponse>, // Pole zawierające tłumaczenie
    //responseStatus: Option<u16>, // Zmiana typu na u16, bo API zwraca liczby
    //responseDetails: Option<String>,
}

#[derive(Deserialize, Debug)]
struct TranslateResponse {
    translatedText: String,
}

async fn translate_text(text: &str, api_key: &str) -> Result<String, String> {
    println!("etap 1 - wysyłanie zapytania");

    // Utwórz klienta HTTP
    let client = Client::new();

    // Wysyłamy zapytanie GET do API MyMemory z parametrami w query string
    let response = client
        .get("https://api.mymemory.translated.net/get")
        .query(&[("q", text), ("langpair", "pl|es"), ("key", api_key)])
        .timeout(Duration::new(30, 0))
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status() == StatusCode::OK {
                println!("etap 2 - odpowiedź otrzymana");

                // Deserializujemy pełną odpowiedź JSON
                let full_response: MyMemoryResponse = res.json().await.map_err(|e| e.to_string())?;
                println!("Otrzymana odpowiedź: {:?}", full_response);

                if let Some(data) = full_response.responseData {
                    println!("etap 3 - tłumaczenie: {}", data.translatedText);
                    Ok(data.translatedText)
                } else {
                    Err("Brak danych tłumaczenia w odpowiedzi".to_string())
                }
            } else {
                let error_message = format!(
                    "Błąd odpowiedzi API: {} - szczegóły: {}",
                    res.status(),
                    res.text().await.unwrap_or_else(|_| "Brak szczegółów".to_string())
                );
                Err(error_message)
            }
        }
        Err(e) => {
            eprintln!("Błąd zapytania: {}", e);
            Err("Błąd zapytania".to_string())
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Zaczynamy tłumaczenie...");

    // Podaj swój klucz API MyMemory
    let api_key = "Nie tak prędko"; // Zamień "YOUR_API_KEY" na swój rzeczywisty klucz API

    match translate_text("Mam na imię Wojtek, jestem Polakiem i jestem super!", api_key).await {
        Ok(result) => println!("Tłumaczenie: {}", result),
        Err(e) => eprintln!("Błąd: {}", e),
    }
}
