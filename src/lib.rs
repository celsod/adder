pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1  { // || value > 100 removed for testing purposes
            panic!("Guess value must be greater than 1, got {}!", value);
        }
        else if value > 100 {
            panic!("Guess value must be less than 100, got {}", value);
        }

        Guess {value}
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //The test module is an inner module, bringing the test code in the outher module into the scope of the inner module
    //The * represents a blog to use the entire module.

    // Commented out due to not being used in later part of the chapter. 
    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    /* #[test]
    fn another() {
        panic!("Make this test fail!");
    } */

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
        /*The assert! macro is checking to see if the output is true in this instance.
        The larger rectangle can hold the smaller rectangle */
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
        /*The assert! macro is checking if the result is true, but in this case it is false,
        so the extra ! inside the expression is the logical "not" which turns the value to true.  The way the 
        tutorial describes it is that the extra ! means that the assert! macro wants a false for the test to be a pass */
    }

    /* #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    } */

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
                "Greeting did not contain name, value was {}", result);
    }

    #[test]
    #[should_panic(expected = "less than 100")] //the test should cause a panic in the function
    //the expected line is a substring of the output message that causes the panic
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[ignore] //when running cargo test, it will ignore this test
    fn expensive_test() {
        // code that takes an hour to run
    }
}

