#![allow(unused_variables)]

fn compute_sum_of_numbers(numbers: Vec<i32>) -> i32 {
    unimplemented!()
}

fn find_even_numbers(numbers: Vec<i32>) -> Vec<i32> {
    unimplemented!()
}

fn find_positive_numbers(numbers: Vec<i32>) -> Vec<i32> {
    unimplemented!()
}

fn find_last_number_or_zero(numbers: Vec<i32>) -> i32 {
    unimplemented!()
}

fn find_total_age(persons: Vec<Person>) -> i32 {
    unimplemented!()
}

fn find_sub_list_from_predicate(
    persons: Vec<Person>,
    predicate: fn(&Person) -> bool,
) -> Vec<Person> {
    unimplemented!()
}

// Duplicates elements in vector
// Example: [1, 5, 2] -> [1, 1, 5, 5, 2, 2]
fn duplicate(elements: Vec<i32>) -> Vec<i32> {
    unimplemented!()
}

// Get fibonacci number from sequence at n size
fn fibonacci(nth: u32) -> u32 {
    unimplemented!()
}

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_sum_of_numbers() {
        assert_eq!(compute_sum_of_numbers(vec![1, 2, 3]), 6);
        assert_eq!(compute_sum_of_numbers(vec![]), 0)
    }

    #[test]
    fn test_find_even_numbers() {
        assert_eq!(find_even_numbers(vec![1, 2, 3, 4]), vec![2, 4])
    }

    #[test]
    fn test_find_positive_numbers() {
        assert_eq!(
            find_positive_numbers(vec![1, -4, -8, 11, -200, -1, 8]),
            vec![1, 11, 8]
        );
        assert_eq!(find_positive_numbers(vec![0]), vec![])
    }

    #[test]
    fn test_find_last_item_or_zero() {
        assert_eq!(find_last_number_or_zero(vec![1, 2, 3]), 3);
        assert_eq!(find_last_number_or_zero(vec![]), 0);
    }

    #[test]
    fn test_find_total_age() {
        assert_eq!(
            find_total_age(vec![
                Person {
                    name: "John Smith".to_owned(),
                    age: 25
                },
                Person {
                    name: "Sandra White".to_owned(),
                    age: 19
                },
                Person {
                    name: "Paul Wright".to_owned(),
                    age: 64
                }
            ]),
            108
        );
    }

    #[test]
    fn test_find_sub_list_from_predicate() {
        let persons = vec![
            Person {
                name: "John Smith".to_owned(),
                age: 25,
            },
            Person {
                name: "Sandra White".to_owned(),
                age: 19,
            },
            Person {
                name: "Paul Wright".to_owned(),
                age: 64,
            },
        ];
        assert_eq!(
            find_sub_list_from_predicate(persons, over_age_30),
            vec![Person {
                name: "Paul Wright".to_owned(),
                age: 64
            }]
        );
    }

    #[test]
    fn test_duplicate() {
        assert_eq!(
            duplicate(vec![5, 1, 10, 11, 0]),
            vec![5, 5, 1, 1, 10, 10, 11, 11, 0, 0]
        );
        assert_eq!(duplicate(vec![]), vec![]);
    }

    fn over_age_30(person: &Person) -> bool {
        person.age > 30
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(1), 0);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(12), 89);
        assert_eq!(fibonacci(20), 4181);
    }
}
