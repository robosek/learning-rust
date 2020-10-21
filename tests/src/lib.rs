pub struct Guess {
    value: i32
}

impl Guess{
    pub fn new(value: i32) -> Guess {
        if value < 0 || value > 100 {
            panic!("Guess value must be between 0 and 100. You value is {}", value)
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic]
    fn should_panic_if_greater_then_100(){
        Guess::new(300);
    }

    #[test]
    fn should_initialize_new_value(){
        let expected_value = 43;
        let guess = Guess::new(43);

        assert!(guess.value == expected_value);
    }

    #[test]
    fn this_test_should_fail_with_defined_message(){
        let expected_vale = 43;
        let guess = Guess::new(23);

        assert!(guess.value == expected_vale,
                "This is not correct value. The result is {}",
                guess.value)

    }

    #[test]
    #[ignore]
    fn this_test_should_be_ignored(){
        assert!(true);
    }
}
