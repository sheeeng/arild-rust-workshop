# Part 5 - Rust on Web exercises

The objective of the exercises for this part is to flesh out the web server
skeleton in this folder.

## 1. Get the server up and running

The main.rs file is riddled with typos. See if you can fix it and get it up and
running with `cargo run`.

There's also some missing dependencies in the Cargo.toml file. See if you can
find them and install them to the project using `cargo add X Y Z`!

> Hint: https://docs.rs/axum/latest/axum/#required-dependencies

The goal is to be able to access `localhost:3000` and receive a "Hello future
Rust developers!!" in a header tag.

## 2. Implement function `get_fib_at_nth` for struct WebState

This is the function that will use the code for the Fibonacci sequence you
implemented earlier. We will use this function in #3.

## 3. Implement GET /fib/:number page

Now that we've got a web page up and running, the next goal is to implement the
fib_handler function. See comments for instructions.

Notice the `:` in `/fib/:number`. This is what tells the Axum parser what part
of the url to extract as a path variable.

Don't forget to add the route to the app!

## 4. Implement GET /fib/log

In order to see how many times you have requested a fibonacci number, and what
numbers you've received so far, implement the handler for fib_log.

Then add the route to the app router in the main function.

## BONUS. Implement the trait "Display" to the WebState struct

If you've raced through the previous tasks, you can try at implementing the
Display trait for the WebState struct. Use this page in "Rust by Example" as
reference:
[Display](https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html)
