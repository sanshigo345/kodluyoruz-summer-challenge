use std::io;

fn main() {
    println!("İki sayı girin:");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Okuma hatası");
    let number1: i32 = input1.trim().parse().expect("Geçersiz sayı");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Okuma hatası");
    let number2: i32 = input2.trim().parse().expect("Geçersiz sayı");

    let sum = number1 + number2;
    println!("Toplam: {}", sum);
}
