use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

fn main() {
println!("Guess the number!");

    let mut rng = thread_rng();
    let secret_number = rng.gen_range(1,101);

    println!("This is secret number: {}", secret_number);

    loop{
        println!("Please input the value");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Cannot read input");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("To small"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("Yes, you're right!");
                break;
            },
        };
    };
}
