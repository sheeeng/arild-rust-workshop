#![allow(unused_variables)]

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    unimplemented!()
}

fn get_value_or_zero(x: Option<i32>) -> i32 {
    unimplemented!()
}

fn subtract_one(x: Option<i32>) -> Option<i32> {
    unimplemented!()
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
    unimplemented!()
}

// EXTRA BONUS CREDIT

// Given a nested Option<Option<T>>, write a function to "flatten"
// it to a single Option<T>. If any of the Options is None, the
// result should also be None.

fn flatten_option<T>(value: Option<Option<T>>) -> Option<T> {
    unimplemented!()
}

// Write a function that tries to convert a &str to an u32.
// If successful, check if the number is positive.
// Return a Result indicating the outcomes.

fn validate_and_process(input: &str) -> Result<u32, &'static str> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_in_cents() {
        assert_eq!(value_in_cents(Coin::Penny), 1);
        assert_eq!(value_in_cents(Coin::Nickel), 5);
        assert_eq!(value_in_cents(Coin::Dime), 10);
        assert_eq!(value_in_cents(Coin::Quarter), 25);
    }

    #[test]
    fn test_get_value_or_zero() {
        assert_eq!(get_value_or_zero(Some(3)), 3);
        assert_eq!(get_value_or_zero(Some(0)), 0);
        assert_eq!(get_value_or_zero(None), 0);
    }

    #[test]
    fn test_subtract_one() {
        assert_eq!(subtract_one(Some(3)), Some(2));
        assert_eq!(subtract_one(Some(-1)), Some(-2));
        assert_eq!(subtract_one(None), None);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert_eq!(divide(10.0, 0.0), Err("Division by zero error"));
    }

    #[test]
    fn test_flatten_option() {
        assert_eq!(flatten_option(Some(Some(5))), Some(5));
        // Ignore the turbofish here.
        assert_eq!(flatten_option::<i32>(Some(None)), None);
        assert_eq!(flatten_option::<i32>(None), None);
    }

    #[test]
    fn test_validate_and_process() {
        assert_eq!(validate_and_process("42"), Ok(42));
        assert_eq!(
            validate_and_process("-5"),
            Err("Negative number not allowed")
        );
        assert_eq!(validate_and_process("hello"), Err("Not a valid number"));
    }
}
