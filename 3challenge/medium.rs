fn main() {
    let kitap_fiyat = 80;
    let defter_fiyat = 20;
    let kalem_fiyat = 5;
    
    let kitap_adet = 2;
    let defter_adet = 3;
    let kalem_adet = 4;
    
    let toplam_odeme = (kitap_fiyat * kitap_adet) + (defter_fiyat * defter_adet) + (kalem_fiyat * kalem_adet);
    
    println!("Müşteri toplam ödeme: {} TL", toplam_odeme);
}
