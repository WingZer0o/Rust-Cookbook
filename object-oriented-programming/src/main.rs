mod trait_file;
use trait_file::Show;

fn main() {
    big_implications();
}

fn big_implications() {
    let answer = 42;
    let maybe_pi = 3.14;
    let v: Vec<&dyn Show> = vec![&answer,&maybe_pi];
    for d in v.iter() {
        println!("show {}", d.show());
    }
}