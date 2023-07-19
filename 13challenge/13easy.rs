use std::io;

fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..(num as f64).sqrt() as u64 + 1 {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    println!("Bir sayı girin: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Okuma hatası.");
    let sayi: u64 = input.trim().parse().expect("Geçersiz sayı.");

    if is_prime(sayi) {
        println!("{} bir asal sayıdır.", sayi);
    } else {
        println!("{} bir asal sayı değildir.", sayi);
    }
}
