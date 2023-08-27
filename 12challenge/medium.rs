fn main() {
    let total_animals = 36;
    let total_legs = 100;
    
    for chickens in 0..=total_animals {
        let sheep = total_animals - chickens;
        let legs = (chickens * 2) + (sheep * 4);
        
        if legs == total_legs {
            println!("Tavuk: {}, Koyun: {}", chickens, sheep);
            break;
        }
    }
}
