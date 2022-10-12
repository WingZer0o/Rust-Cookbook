use array::sort_array;

mod array;

fn main() {
    sort_i64_boxed_array();
}

fn sort_i64_boxed_array() {
    let newArray = Box::new([10, 25, 1, 5, 90, 55, 14]);
    let sorted = sort_array(newArray);
    for x in sorted.iter() {
        println!("{}", x);
    }
}