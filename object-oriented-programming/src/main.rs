mod trait_file;
use trait_file::Show;

fn main() {
    big_implications();
    big_box_implications();
}

fn big_implications() {
    let answer = 42;
    let maybe_pi = 3.14;
    let v: Vec<&dyn Show> = vec![&answer,&maybe_pi];
    for d in v.iter() {
        println!("show {}", d.show());
    }
}

// 
fn big_box_implications() {
    let answer = Box::new(42);
    let maybe_pi = Box::new(3.14);
    let show_list: Vec<Box<dyn Show>> = vec![answer, maybe_pi];
    for d in &show_list {
        println!("show {}", d.show());
    }
}