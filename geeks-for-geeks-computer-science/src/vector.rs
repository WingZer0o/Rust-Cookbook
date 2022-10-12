use rand::Rng;

pub fn create_10_digit_vector(order: bool) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    for x in 1..10 {
        vec.push(rng.gen());
    }
    if order {
        vec.sort();
    }
    return vec;
}