enum Food {
    Apple,
    Carrot,
    Chocolate,
    // Add another value and see what happens
}

fn is_healthy(food: Food) -> String {
    match food {
        Food::Apple => "Yes".to_string(),
        Food::Carrot | Food::Chocolate => "No".to_string(),
    }
}

fn main() {
    println!("Apples are healthy? {}", is_healthy(Food::Apple));
    println!("Carrots are healthy? {}", is_healthy(Food::Carrot));
    println!("Chocolate is healthy? {}", is_healthy(Food::Chocolate));
}
