use std::io;

fn sum_of_divisors(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=n {
        if n % i == 0 {
            sum += i;
        }
    }
    sum
}

fn main() {
    println!("Bir sayı girin:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Okuma hatası");

    let number: u32 = input.trim().parse().expect("Geçersiz sayı");

    let result = sum_of_divisors(number);
    println!("{} sayısının tam bölenlerinin toplamı: {}", number, result);
}
