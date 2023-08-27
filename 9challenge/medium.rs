fn main() {
    let numbers = vec![5, 10, 3, 8, 15];
    let even_sum: i32 = numbers.iter().filter(|&x| x % 2 == 0).sum();
    println!("Dizideki çift sayıların toplamı: {}", even_sum);
}
