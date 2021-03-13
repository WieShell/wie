use std::io;
pub fn run() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error! Couldn't read correctly!");
        println!("{}", input.replace("\n", ""))
    }
}