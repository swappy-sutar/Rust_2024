use std::io;

fn main() {

    let mut input = String::new();
    println!("Enter your name: ");
    io::stdin().read_line(&mut input).expect("input failed");
    println!("Hello, {}!", input);

}
