fn count_occurrences(arr: &[i32], target: i32) -> usize {
    let first_occurrence = arr.iter().position(|&x| x >= target).unwrap_or(arr.len());
    let last_occurrence = arr.iter().rposition(|&x| x <= target).unwrap_or(0);
    
    if first_occurrence > last_occurrence {
        0
    } else {
        last_occurrence - first_occurrence + 1
    }
}

fn main() {
    let sorted_array = vec![2, 3, 3, 5, 5, 5, 7, 8, 8, 8, 8, 10];
    let target_number = 5;
    
    let occurrences = count_occurrences(&sorted_array, target_number);
    println!("Hedef sayı {} dizide {} kez tekrar etti.", target_number, occurrences);
}
