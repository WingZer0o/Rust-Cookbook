mod array;

use array::sort_arrayi32_choice;

fn main() {
    let newArray = Box::new([10, 25, 1, 5, 90, 55, 14]);
    sort_arrayi32_choice(newArray, false);
}