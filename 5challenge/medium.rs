use std::io;

fn main() {
    println!("Bir metin girin:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Okuma hatası");

    let words = input.trim().split_whitespace().collect::<Vec<&str>>();
    let word_count = words.len();

    println!("Metindeki kelime sayısı: {}", word_count);
}
