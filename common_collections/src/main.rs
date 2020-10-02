mod statistic;
mod text_converter;
mod human_resources;

use statistic::*;
use text_converter::*;
use human_resources::*;
use std::collections::HashMap;
use std::io;
use std::io::Read;

fn main(){
    // let mut test_suite = vec![2,3,3,123,123,3,5453,46,46];
    // let apple = "apple";
    // let beer = "beer";
    // println!("Mean is {}", mean(&test_suite));
    // println!("Median is {}", median( &mut test_suite));
    // println!("Mode is {}", mode(test_suite));
    // println!("Text for apple is {}", convert_to_pig_latin(apple));
    // println!("Text for beer is {}", convert_to_pig_latin(beer));

    let mut database: HashMap<String,String> = HashMap::new();
    let mut should_add_new_employee = true;
    while should_add_new_employee{
        println!("Please add new employee by typing Add 'Name' to 'Department'");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Cannot read this line!");
        let employee = process_employee_line(command);
        add_employee(&mut database, &employee);

        println!("Employee added! Do you want to add new one? Y(Yes)/N(No)");

        let mut add_new = String::new();
        io::stdin()
            .read_line(&mut add_new)
            .expect("Cannot read this line!");

        if add_new.trim().to_lowercase() != "y"{
            should_add_new_employee = false
        }
    }

    list_all_employees(&database);

    println!("Please type department name to search employees");
    let mut department = String::new();
    io::stdin()
        .read_line(&mut department)
        .expect("Cannot read line!");

    search_by_department(&database, department);
}
