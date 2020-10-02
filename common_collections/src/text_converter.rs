use crate::human_resources::Employee;

pub fn convert_to_pig_latin(text: &str) -> String{

    fn is_vowel(letter: &str) -> bool {
        let vowels = vec!["a","e","i","o","u","y"];
        let result = vowels.iter().find(|&&character| character == letter);
        match result{
            Some(_) => true,
            None => false
        }
    }

    if text.len() == 0 {
        String::from(text)
    }
    else{
        let first_letter = &text[..1];
        let is_vowel = is_vowel(first_letter);

        if is_vowel { String::from(text) +"-hay" } else { String::from(&text[1..]) + "-" + first_letter + "ay" }
    }
}

pub fn process_employee_line(command: String) -> Employee {
    let command = command.trim();
    let command = command.to_lowercase();
    let words:Vec<_> = command.trim().split(" ").collect();
    let filtered_words: Vec<_> = words.iter().filter(|&&word| word != "add" && word!= "to").collect();

    let maybe_name = filtered_words.first();
    let maybe_department = filtered_words.last();

    Employee {
        name: maybe_name.expect("no name").to_string(),
        department: maybe_department.expect("no department").to_string()
    }
}