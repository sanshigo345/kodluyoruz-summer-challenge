use std::io;

fn count_vowels(text: &str) -> usize {
    let vowels = "aeiouAEIOU";
    text.chars().filter(|c| vowels.contains(*c)).count()
}

fn main() {
    println!("Bir metin girin:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Okuma hatası");

    let text = input.trim();
    let vowel_count = count_vowels(text);

    println!("Metindeki sesli harf sayısı: {}", vowel_count);
}
