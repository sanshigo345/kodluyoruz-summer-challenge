use std::io;

fn main() {
    println!("Doğum tarihinizi girin (GG.AA.YYYY):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Giriş okunamadı");

    let birthday: Vec<&str> = input.trim().split('.').collect();

    if birthday.len() != 3 {
        println!("Geçersiz tarih formatı");
        return;
    }

    let day: u32 = birthday[0].parse().expect("Gün geçerli bir sayı değil");
    let month: u32 = birthday[1].parse().expect("Ay geçerli bir sayı değil");
    let year: u32 = birthday[2].parse().expect("Yıl geçerli bir sayı değil");

    let age = calculate_age(day, month, year);

    println!("Yaşınız: {}", age);
}

fn calculate_age(day: u32, month: u32, year: u32) -> u32 {
    let now = chrono::offset::Local::now().date();
    let birthday = chrono::NaiveDate::from_ymd(year as i32, month, day);
    let age = now.year() - birthday.year();

    if now.month() < birthday.month() || (now.month() == birthday.month() && now.day() < birthday.day()) {
        age - 1
    } else {
        age
    }
}
