use std::io;

fn has_duplicate_words(sentence: &str) -> bool {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    
    for i in 0..words.len() - 1 {
        if words[i] == words[i + 1] {
            return true;
        }
    }
    
    false
}

fn main() {
    println!("Bir cümle girin:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Okuma hatası");

    let sentence = input.trim();

    if has_duplicate_words(&sentence) {
        println!("Cümlede ikileme bulunuyor.");
    } else {
        println!("Cümlede ikileme bulunmuyor.");
    }
}
