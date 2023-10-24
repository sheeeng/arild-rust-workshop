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
        // 1. Get the fibonacci sequence up to the nth number provided
        let fib_sequence = fibonacci(nth);
        // 2. Get the nth/last number of the sequence
        let fib_at_nth = *fib_sequence.last().unwrap();
        // 3. Update the web state with help of private function update_web_state
        self.update_web_state(fib_at_nth);

        // 3. Return the fib at the nth position
        fib_at_nth
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

impl Display for WebState {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let prev_fib = self.previous_fib.lock().unwrap();
        let fib_log = self.fib_log.lock().unwrap();
        write!(f, "The last fibonacci number requested was: {}.
                   Since the server spawned, you've requested a total of {:?} number of fibonacci numbers.",
               prev_fib, fib_log)
    }
}

// Get the fibonacci sequence of provided size
fn fibonacci(size: u64) -> Vec<u64> {
    // Implement / copy fibonacci code from previous part

    // 1. Create a new vec that contains the type u64 to hold the sequence
    let mut sequence = Vec::<u64>::new();

    // 2. Loop through all the numbers starting from 0 to the desired fibonacci number
    for i in 0..size {
        // 3. First two fibonacci numbers equal the index, afterwards its the sum of the previous
        //    two numbers
        if i == 0 || i == 1 {
            sequence.push(i);
        } else {
            // 4. Shadow index to be type usize, in order to be able to get the value
            let i = i as usize;
            // 5. Calculate the next value based on the sum of the two previous numbers
            let next_value = sequence[i - 1] + sequence[i - 2];
            // 6. Append the next value to the sequence
            sequence.push(next_value);
        }
    }
    // 7. Return fibonacci sequence
    sequence
}
