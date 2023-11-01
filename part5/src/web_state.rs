use std::{
    fmt::{Display, Formatter, Result},
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
pub struct WebState {
    pub fib_log: Arc<Mutex<Vec<u64>>>,
    pub previous_fib: Arc<Mutex<u64>>,
}

impl WebState {
    // Constructor for our web state, sets default values for our properties
    pub fn new() -> Self {
        WebState {
            fib_log: Arc::new(Mutex::new(Vec::<u64>::new())),
            previous_fib: Arc::new(Mutex::new(0)),
        }
    }

    // Implement this!
    pub fn get_fib_at_nth(&self, nth: u64) -> u64 {
        // 2.1 Get the fibonacci sequence up to the nth number provided
        // 2.2 Get the nth/last number of the sequence
        // 2.3 Update the web state with help of private function update_web_state
        // 2.4 Return the fib at the nth position
        unimplemented!()
    }

    pub fn print_fib_log(&self) {
        let log = self.fib_log.lock().unwrap();
        println!("{:?}", log);
    }

    pub fn get_fib_log(&self) -> String {
        let log = self.fib_log.lock().unwrap();
        log.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(",<br>")
    }

    // Private function that overwrites the state. Don't touch! :-)
    fn update_web_state(&self, fib_value: u64) {
        let mut log = self.fib_log.lock().unwrap();
        let mut previous_fib = self.previous_fib.lock().unwrap();
        log.push(fib_value);
        *previous_fib = fib_value;
    }
}

// Get the fibonacci sequence of provided size. If you didn't solve this in a previous part, you
// can find the code for this in solution/web_state.rs
fn fibonacci(size: u64) -> Vec<u64> {
    unimplemented!()
}

// Bonus: Implement custom display trait for WebState
// Goal: When println! is called on WebState, it should print out:
// `The last fibonacci number requested is: {}. Since the server was spawned, you've requested a
// total of {} number of fibonacci numbers`
