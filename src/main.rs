mod translate_text;
mod detect_language;
//mod file_operations;

use std::{fs, io};
use translate_text::{translate_text};
use detect_language::detect_language;
use pdf_extract::extract_text;

    

fn get_text() -> String {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Failed to read line");
    text.trim().to_string()
}

fn get_language_pair(text: &str) -> Option<String> {
    println!("Do you want to try to detect language? (y/n)");
    match get_text().as_str() {
        "y" => {
            println!("To which language you want to translate?");
            let lang = get_text();
            Some(format!("{}|{}", detect_language(text).ok()?, lang))
        }
        "n" => {
            println!("Type pair of language - ex: pl|en");
            Some(get_text())
        }
        _ => {
            println!("Invalid input.");
            None
        }
    }
}

fn divide_into_blocks(text: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut current_string = String::new();
    for i in text.chars() {
        if current_string.len() > 490 {
            current_string.push(' ');
            blocks.push(current_string.clone());
            current_string.clear();    
        }
        current_string.push(i);
    }
    return blocks;
}            


#[derive(Debug)]
enum FileType {
    Pdf,
    Txt,
    Other(String),
}

pub fn get_file_type(path: &str) -> Result<String, String> {

    let file_type = match path {
        path if path.ends_with(".pdf") => Ok(FileType::Pdf),
        path if path.ends_with(".txt") => Ok(FileType::Txt),
        _ => Err(format!("Unsupported file type: {}", path)),
    };

    file_type.map(|ft| format!("{:?}", ft))

}

#[tokio::main]
async fn main() {
    let api_key = "YOUR_API_KEY"; // Replace with your actual API key

    println!("Choose action:\n1 - Translate text\n2 - Translate text file");
    let action = get_text();

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
    }
}