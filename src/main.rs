mod translate_text;
mod detect_language;
mod file_operations;

use translate_text::translate_text;
use detect_language::detect_language;
use file_operations::divide_into_blocks;
use std::process::{Command, Stdio};

#[tokio::main]
async fn main() {
    let api_key = "YOUR API KEY"; // Replace with your actual API key

    let output = Command::new("xclip")
    .args(&["-o", "-selection", "primary"])
    .output()
    .expect("failed to execute xclip");

    let input_text = String::from_utf8_lossy(&output.stdout);


    if input_text.trim().is_empty() {
        println!("No text found in clipboard.");
        return;
    }

    let lang = detect_language(&input_text).unwrap_or_else(|_| "Unknown".to_string());

    let pair: String = format!("{}|pl", lang);

    let blocks = divide_into_blocks(&input_text);
    let mut whole_string = String::new();

    for i in blocks {
        match translate_text(&i, api_key, &pair).await {
            Ok(translated) => {
                whole_string.push_str(&translated);
            }
            Err(e) => {
                let _ = Command::new("zenity")
                    .args(&["--error", "--title=Błąd", "--text", &format!("Błąd tłumaczenia: {}", e)])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn();
            }
        }
    }

    let _ = Command::new("zenity")
        .args(&["--info", "--title=Tłumaczenie", "--text", &whole_string])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
}