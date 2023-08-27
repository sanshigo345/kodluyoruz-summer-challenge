fn main() {
    let first_runner_speed = 15; // km/h
    let second_runner_speed = 20; // km/h
    
    let relative_speed = second_runner_speed - first_runner_speed;
    let time_to_catch_up = 1.0 * 15.0 / relative_speed as f64;
    
    println!("İkinci yarışmacı ilk yarışmacıyı {} saat sonra yakalar.", time_to_catch_up);
}
