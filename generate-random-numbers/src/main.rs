use rand::Rng;

use crate::random_values_custom_type::Point;
mod distribution_numbers;
mod random_values_custom_type;
mod random_passwords;

fn main() {
    generate_random_numbers();
    generate_random_numbers_in_range();
    distribution_numbers::generate_numbers_in_distribution();
    generate_random_values();
    random_passwords::generate_random_password();
    random_passwords::generate_user_defined_password();
}


fn generate_random_numbers() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

fn generate_random_numbers_in_range() {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));
}

fn generate_random_values() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}