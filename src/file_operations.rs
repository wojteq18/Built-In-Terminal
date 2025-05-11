pub fn divide_into_blocks(text: &str) -> Vec<String> {
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


