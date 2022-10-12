pub fn sort_arrayi32_choice(arrayToSort: Box<[i32]>, reverse: bool) {
    let sorted;
    if !reverse {
        sorted = sort_array(arrayToSort)
    } else {
        sorted = sort_array_reverse(arrayToSort)
    }
    for x in sorted.iter() {
        println!("{}", x);
    }
}

fn sort_array(mut arrayToSort: Box<[i32]>) -> Box<[i32]> {
    arrayToSort.sort();
    return arrayToSort;
}

fn sort_array_reverse(mut arrayToSort: Box<[i32]>) -> Box<[i32]> {
    arrayToSort.sort_by(|a, b| b.cmp(a));
    return arrayToSort;
}