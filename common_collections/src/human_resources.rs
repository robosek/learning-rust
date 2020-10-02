use std::collections::HashMap;

pub struct Employee{
    pub name: String,
    pub department: String
}


pub fn add_employee(db: &mut HashMap<String,String>, employee: &Employee) {
    db.entry(String::from(&employee.name)).or_insert(String::from(&employee.department));
}


pub fn list_all_employees(db: &HashMap<String, String>){
    let mut items: Vec<_> = db.iter().collect();
    items.
        sort_by(|a, b| a.0.cmp(b.0));

    items.iter()
        .for_each(|e| println!("Name: {}, Department: {}", e.0, e.1))
}

pub fn search_by_department(db: &HashMap<String, String>, department: String){
    let department = department.trim().to_lowercase();
    let items: Vec<_> = db.iter().collect();
    let employees: Vec<_> = items.iter().filter(|&e| e.1 == &department).collect();


    if employees.len() > 0 {
        employees.iter().
            for_each(|e| println!("Name: {}, Department: {}", e.0, e.1))
    }
    else{
        println!("There are no employees in {}", department);
    }
}