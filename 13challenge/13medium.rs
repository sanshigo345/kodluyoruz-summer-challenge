use std::io;

fn main() {
    println!("Bir kelime girin: ");
    let mut kelime = String::new();
    io::stdin().read_line(&mut kelime).expect("Okuma hatası.");

    let buyuk_harfler = kelime.to_uppercase();
    println!("Kelimenin büyük harfli hali: {}", buyuk_harfler);
}
