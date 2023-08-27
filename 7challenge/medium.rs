use std::io;

fn main() {
    println!("Bir sayı girin:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Okuma hatası");

    let number: u32 = input.trim().parse().expect("Geçersiz sayı");
    let mut num = number;
    let mut sum = 0;

    while num > 0 {
        sum += num % 10;
        num /= 10;
    }

    println!("Sayının basamaklarının toplamı: {}", sum);
}
