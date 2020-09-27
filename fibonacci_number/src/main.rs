use std::io;

fn main() {

    //Index:  0 1 2 3 4 5 6  7
    //Number: 1 1 2 3 5 8 13 21
    println!("Please choose the index of Fibonacci number");

    let mut index_text = String::new();
    io::stdin().read_line(&mut index_text).expect("This should be a number.");
    let index: i32 = index_text.trim().parse().expect("Cannot parse this number.");

    fn count_fibonacci(index: i32) -> i32{
        if index <= 1{
            1
        }
        else{
            count_fibonacci(index - 1) + count_fibonacci(index - 2)
        }
    }

    if index < 0 {
        println!("The index is wrong. It should be >= 1.");
    }
    else{
        let current_value = count_fibonacci(index);
        println!("The value of index {} is {}", index, current_value)
    }
}
