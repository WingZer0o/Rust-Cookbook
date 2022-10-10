use rand::{thread_rng, Rng};
use rand::{distributions::Alphanumeric};

pub fn generate_random_password() {
    let rand_string: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .map(char::from)
    .collect();

    println!("Non-user defined-{}", rand_string);
}

pub fn generate_user_defined_password() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";

    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

        println!("User Defined-{:?}", password);
}