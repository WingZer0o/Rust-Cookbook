fn main() {
    println!("Hello, world!");
}


fn match_numbers() {
    let matches = App::new("My Test Program")
        .version("0.1.0")
        .author("Hackerman Jones <testemail@gmail.com>")
        .about("Teaches argument parsing")
        .arg(Arg::with_name("file"))
            .short("f")
            .long("file")
            .takes_value(true)
            .help("A cool file")
        .arg(Arg::with_name("num"))
            .short("n")
            .long("number")
            .takes_value(true)
            .help("Five less than your favorite number")
}