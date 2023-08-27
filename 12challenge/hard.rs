fn main() {
    let filling_rates = [(10, 1), (15, 1), (-30, 1)]; // (hız, adet)
    let mut total_rate = 0;
    
    for (rate, count) in &filling_rates {
        total_rate += rate * count;
    }
    
    let time_to_fill = 1 / total_rate.abs() as f64;
    
    println!("Havuz {} saatte dolar.", time_to_fill);
}
