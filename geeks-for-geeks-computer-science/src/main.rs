mod array;
mod vector;

// use array::sort_arrayi32_choice;
use vector::create_10_digit_vector;
fn main() {
    let vec = create_10_digit_vector(false);
    for x in &vec {
        println!("{}", x);
    }
    // let newArray = Box::new([10, 25, 1, 5, 90, 55, 14]);
    // sort_arrayi32_choice(newArray, false);


}