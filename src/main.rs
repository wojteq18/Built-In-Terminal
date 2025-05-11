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

    let pair = format!("{}|pl", lang);

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


    /*match translate_text(&input_text, api_key, &pair).await {
        Ok(translated) => {
            let _ = Command::new("zenity")
                .args(&["--info", "--title=Tłumaczenie", "--text", &translated])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn();
        }
        Err(e) => {
            let _ = Command::new("zenity")
                .args(&["--error", "--title=Błąd", "--text", &format!("Błąd tłumaczenia: {}", e)])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn();
        }
    }*/
    

    

   /*  let action = get_text();

    match action.as_str() {
        "1" => {
            println!("Type text to translate:");
            let text = get_text();

            let pair = match get_language_pair(&text) {
                Some(p) => p,
                None => return,
            };

            match translate_text(&text, api_key, &pair).await {
                Ok(translated) => println!("Translated text:\n{}", translated),
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        "2" => {
            println!("Type path to file:");
            let path = get_text();
            let file_type = get_file_type(&path).unwrap();
        
            match file_type.as_str() {
                "Txt"  => {
                    let text = fs::read_to_string(&path).expect("Unable to read file");

                    let pair = match get_language_pair(&text) {
                        Some(p) => p,
                        None => return,
                    };
        
                    match translate_text(&text, api_key, &pair).await {
                        Ok(translated) => {
                            fs::write(&path, translated).expect("Unable to write file");
                            println!("File translated successfully.");
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }

                }
                "Pdf" => {
                    let text = extract_text(path).unwrap();
                    let blocks = divide_into_blocks(&text);
                    let mut whole_text = String::new();

                    let pair = match get_language_pair(&text) {
                        Some(p) => p,
                        None => return,
                    };

                     for i in blocks {
                        match translate_text(&i, api_key, &pair).await {
                            Ok(translated) => {
                                whole_text.push_str(&translated);
                            }
                            Err(e) => eprintln!("Error: {}", e),
                        }
                        println!("Translated text:\n{}", whole_text);
                    }

                }
                _ => {
                    println!("Unsupported file type.");
                    return;
                }
            };
        }

        _ => println!("Invalid action."),
    }*/
}