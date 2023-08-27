use std::io;

fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    println!("Bir sayı girin:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Okuma hatası");

    let number: u64 = input.trim().parse().expect("Geçersiz sayı");

    let result = factorial(number);
    println!("{} sayısının faktöriyeli: {}", number, result);
}
