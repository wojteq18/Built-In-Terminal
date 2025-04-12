mod translate_text;

use whatlang::detect;
use std::{fs, io};
use translate_text::{translate_text};
    

fn get_text() -> String {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Failed to read line");
    text.trim().to_string()
}


#[tokio::main]
async fn main() { 
    let api_key = "YOUR_API_KEY"; // Replace with your actual API key
    println!("Type action to perform: 1 - translate text; 2 - translate text file");
    let mut action = String::new();
    io::stdin().read_line(&mut action).expect("Failed to read line");

    let action_num: i32 = action.trim().parse().expect("Please type a number!");
    

    match action_num {
        1 => {
            println!("Type pair of language - first language to translate from, second language to translate to: ex (pl|en)");
            let pair = get_text();
            println!("Type text to translate:");
            let text = get_text();

            match translate_text(&text, &api_key, &pair).await {
                Ok(translated_text) => {
                    println!("Translated text: {}", translated_text);
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        2 => {
            println!("Type pair of language - first language to translate from, second language to translate to: ex (pl|en)");
            let pair = get_text();
            println!("Type path to file:");
            let path = get_text();
            let text = std::fs::read_to_string(&path).expect("Unable to read file");

            match translate_text(&text, api_key, &pair).await {
                Ok(translated_text) => {
                    fs::write(&path, translated_text).expect("Unable to write file");
                    println!("File translated");
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        _ => {
            println!("Invalid action");
            return;
        }
    }
}