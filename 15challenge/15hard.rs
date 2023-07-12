use std::io;

fn main() {
    println!("Tam sayı dizisi oluşturun (sayıları boşlukla ayırın):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Giriş okunamadı");

    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|num| num.parse().expect("Geçerli bir tam sayı değil"))
        .collect();

    println!("Hedef sayıyı girin:");

    let mut target = String::new();
    io::stdin()
        .read_line(&mut target)
        .expect("Giriş okunamadı");

    let target: i32 = target.trim().parse().expect("Geçerli bir tam sayı değil");

    let result = find_target_sum(&numbers, target);

    match result {
        Some(indices) => {
            println!("Hedef sayıya ulaşmak için seçilen sayılar:");
            for index in indices {
                print!("{} ", numbers[index]);
            }
            println!();
        }
        None => println!("Hedef sayıya ulaşmak mümkün değil."),
    }
}

fn find_target_sum(numbers: &[i32], target: i32) -> Option<Vec<usize>> {
    let mut selected_indices = Vec::new();
    let result = backtrack(numbers, target, 0, &mut selected_indices);
    if result {
        Some(selected_indices)
    } else {
        None
    }
}

fn backtrack(numbers: &[i32], target: i32, index: usize, selected_indices: &mut Vec<usize>) -> bool {
    if target == 0 {
        return true;
    }

    if target < 0 || index >= numbers.len() {
        return false;
    }

    selected_indices.push(index);

    let current_number = numbers[index];

    if backtrack(numbers, target - current_number, index + 1, selected_indices) {
        return true;
    }

    selected_indices.pop();

    backtrack(numbers, target, index + 1, selected_indices)
}
