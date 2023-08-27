use std::io;

fn main() {
    println!("Bir sayı girin:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Okuma hatası");

    let number: i32 = input.trim().parse().expect("Geçersiz sayı");

    let square = number * number;
    println!("Girilen sayının karesi: {}", square);
}
