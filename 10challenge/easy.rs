fn main() {
    let numbers = vec![5, 10, 3, 8, 15];
    let max_number = numbers.iter().max().cloned().unwrap_or(0);
    println!("Dizideki en büyük sayı: {}", max_number);
}
