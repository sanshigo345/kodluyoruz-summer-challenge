use std::io;

fn main() {
    println!("Bir kelime girin:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Okuma hatası");

    let word = input.trim();
    let length = word.len();

    println!("Girilen kelimenin uzunluğu: {}", length);
}
