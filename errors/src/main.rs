use std::io;

fn main() {
    let mut test = String::new();
    let result = io::stdin()
                    .read_line(&mut test);

    println!("Hello, world!");
}
