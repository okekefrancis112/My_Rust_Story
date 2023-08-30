use std::io;

fn main() {
    // println!("Hello, world!");
    println!("Enter your weight (Kg): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32  = input.trim().parse().unwrap();
    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {}Kg", mars_weight);

    calculate_weight_on_mars(100.0);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

//  Ownership in rust
// 1. each value in rust is owned by a variable

// 2. when the owner goes out of scope, the value will be deallocated

// 3. There can only be one owner at a time