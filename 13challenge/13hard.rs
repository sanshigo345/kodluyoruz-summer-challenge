use std::io;

fn main() {
    let cost: u32 = 100;
    let price: u32 = 150;

    if cost >= price {
        println!("Hata: Birim maliyet, birim satış fiyatından düşük olmalıdır.");
    } else {
        let profit_threshold: u32 = cost / (price - cost);
        println!("Cost: {}", cost);
        println!("Price: {}", price);
        println!("Kaç ürün satılırsa kâr edilir? {}", profit_threshold);
    }
}
