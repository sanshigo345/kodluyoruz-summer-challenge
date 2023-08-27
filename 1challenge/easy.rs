fn main() {
    let total_balls = 5 + 4 + 3;
    let probability = (5.0 / total_balls as f64) * ((4.0 - 1.0) / (total_balls - 1) as f64) +
                     (4.0 / total_balls as f64) * ((3.0 - 1.0) / (total_balls - 1) as f64) +
                     (3.0 / total_balls as f64) * ((2.0 - 1.0) / (total_balls - 1) as f64);
    
    println!("İki topun aynı renk olma olasılığı: {:.2}", probability);
}
