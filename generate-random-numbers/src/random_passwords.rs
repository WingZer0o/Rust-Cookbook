use rand::{thread_rng, Rng};
use rand::{distributions::Alphanumeric};

pub fn generate_random_passwords() {
    let rand_string: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .map(char::from)
    .collect();

    println!("{}", rand_string);
}