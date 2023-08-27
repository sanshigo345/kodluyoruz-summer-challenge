use std::io;

fn is_armstrong_number(number: u32) -> bool {
    let mut num = number;
    let mut sum = 0;

    let digits = (number as f64).log10() as u32 + 1;

    while num > 0 {
        let digit = num % 10;
        sum += u32::pow(digit, digits);
        num /= 10;
    }

    sum == number
}

fn main() {
    println!("Bir sayı girin:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Okuma hatası");

    let number: u32 = input.trim().parse().expect("Geçersiz sayı");

    if is_armstrong_number(number) {
        println!("{} bir Armstrong sayısıdır.", number);
    } else {
        println!("{} bir Armstrong sayısı değildir.", number);
    }
}
