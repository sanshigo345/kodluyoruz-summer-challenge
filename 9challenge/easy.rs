use std::io;

fn main() {
    println!("Bir sayı girin:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Okuma hatası");

    let number: i32 = input.trim().parse().expect("Geçersiz sayı");

    if number % 2 == 0 {
        println!("Girilen sayı çifttir.");
    } else {
        println!("Girilen sayı tektir.");
    }
}
