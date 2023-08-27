use std::io;

fn main() {
    println!("Bir sayı girin:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Okuma hatası");

    let number: f64 = input.trim().parse().expect("Geçersiz sayı");

    if number >= 0.0 {
        let square_root = number.sqrt();
        if (square_root - square_root.round()).abs() < f64::EPSILON {
            println!("Cevap: {}", square_root as i64);
        } else {
            println!("Cevap: Karekökten tam olarak çıkmıyor.");
        }
    } else {
        println!("Negatif sayının karekökü alınamaz.");
    }
}
