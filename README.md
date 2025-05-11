# PDF Text Translator

This is a simple Linux desktop tool written in Rust that allows you to translate selected text (e.g. from a PDF) into another language using the MyMemory translation API. It is designed to be triggered via a custom keyboard shortcut and displays the translation in a graphical dialog.

## Getting Started

Before using the program, you need to obtain a **free MyMemory API key**:

1. Go to [https://mymemory.translated.net/doc/keygen.php](https://mymemory.translated.net/doc/keygen.php)  
2. Generate your free API key.  
3. Open `src/main.rs` and replace the placeholder in line 12 with your actual API key:

```rust
let api_key = "YOUR API KEY"; // Replace with your actual API key.
```

## Requirements

To use this program, the following software must be installed on your Linux system:

- `zenity` – for displaying graphical dialog windows  
- `xclip` – for accessing the clipboard content  
- Rust compiler and Cargo (you can install both via `rustup`)  

## Default Behavior

The application reads the currently selected clipboard text, automatically detects its language, and translates it into **Polish**.

If you want to translate into a different language, edit the following line in `src/main.rs` (line 29):

```rust
let pair: String = format!("{}|pl", lang);
```

Replace `"pl"` with your desired target language code according to the file in `src/detect_language.rs`.

## Usage

1. Build the application in release mode:

   ```bash
   cargo build --release
   ```

2. Set a global keyboard shortcut in your system settings to point to the compiled binary  
   (typically located at `target/release/pdf-translator` or similar path).

3. To use the translator:
   - Select a portion of text (e.g. in a PDF viewer)
   - Press your configured keyboard shortcut
   - A dialog window will appear showing the translated text

**Note:** The selected text should be longer than just a few words to ensure reliable language detection by the API.

## Translation Quality

The quality of the translation may vary and is far from perfect. However, it is usually sufficient to understand the general meaning of the selected text — even if it's written in a language you don't speak.

## Example Usage

Below are example screenshots showing how the program works:

![Selecting text and triggering translation](png/1.png)
![Translated text shown in a dialog](png/2.png)
