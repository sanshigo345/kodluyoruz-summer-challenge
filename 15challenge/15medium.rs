use std::collections::HashMap;
use std::io;

fn main() {
    println!("Bir metin girin:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Giriş okunamadı");

    let text = input.trim();

    let max_repeated_char = find_max_repeated_char(text);

    match max_repeated_char {
        Some((char, count)) => println!("En çok tekrar eden harf: {}, Toplam tekrar sayısı: {}", char, count),
        None => println!("Metinde hiç harf yok veya tüm harfler eşit tekrar ediyor."),
    }
}

fn find_max_repeated_char(text: &str) -> Option<(char, usize)> {
    let mut char_count_map: HashMap<char, usize> = HashMap::new();

    for c in text.chars() {
        if c.is_alphabetic() {
            let count = char_count_map.entry(c).or_insert(0);
            *count += 1;
        }
    }

    let mut max_count = 0;
    let mut max_char = '\0';

    for (char, count) in char_count_map {
        if count > max_count {
            max_count = count;
            max_char = char;
        }
    }

    if max_count > 0 {
        Some((max_char, max_count))
    } else {
        None
    }
}
