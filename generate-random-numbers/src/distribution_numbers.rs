use rand_distr::{Distribution, Normal, NormalError};
use rand::thread_rng;

pub fn generate_numbers_in_distribution() -> Result<(), NormalError> {
    let mut rng = thread_rng();

    let normal = Normal::new(2.0, 3.0)?;
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v);
    Ok(())
}