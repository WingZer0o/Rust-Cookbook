mod vector_helpers;

fn main() {
    let vec = vec![1, 5, 10, 2, 15];
    let sorted_vec = vector_helpers::sort_vector(vec);
    assert_eq!(sorted_vec, vec![1, 2, 5, 10, 15]);
}