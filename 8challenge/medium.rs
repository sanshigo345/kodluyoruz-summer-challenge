fn median(numbers: &mut [i32]) -> f32 {
    numbers.sort();
    let length = numbers.len();
    if length % 2 == 1 {
        numbers[length / 2] as f32
    } else {
        let middle_right = numbers[length / 2];
        let middle_left = numbers[length / 2 - 1];
        (middle_left + middle_right) as f32 / 2.0
    }
}

fn main() {
    let mut numbers = vec![5, 10, 3, 8, 15];
    let result = median(&mut numbers);
    println!("Dizinin medyanı: {}", result);
}
