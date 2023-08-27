fn main() {
    let numbers = vec![5, 10, 3, 8, 15];
    let max_number = numbers.iter().max().cloned().unwrap_or(0);
    let min_number = numbers.iter().min().cloned().unwrap_or(0);
    println!("En büyük sayı: {}, En küçük sayı: {}", max_number, min_number);
}
