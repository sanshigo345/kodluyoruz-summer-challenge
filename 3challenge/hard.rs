fn main() {
    fn factorial(n: u64) -> u64 {
        if n == 0 || n == 1 {
            1
        } else {
            n * factorial(n - 1)
        }
    }

    let total_students = 30;
    let selected_students = 4;

    let ways_to_select = factorial(total_students) / (factorial(selected_students) * factorial(total_students - selected_students));
    
    println!("Öğrencilerin seçim sayısı: {}", ways_to_select);
}
