fn main() {
    let numbers = vec![5, 10, 15, 20, 25];
    let sum: i32 = numbers.iter().sum();
    let average = sum as f32 / numbers.len() as f32;
    println!("Dizi ortalaması: {}", average);
}
