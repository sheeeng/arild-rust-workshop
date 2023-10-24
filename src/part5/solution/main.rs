use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use web_state::WebState;

mod web_state;

#[tokio::main]
async fn main() {
    // 1. Define Web State for app
    let shared_state = WebState::new();
    // 2. Build the app router
    let app = Router::new()
        .route("/", get(hello_handler))
        .route("/fib/:size", get(fib_handler))
        .route("/fib/log", get(fib_log))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[axum::debug_handler]
async fn fib_handler(Path(nth): Path<String>, State(state): State<WebState>) -> impl IntoResponse {
    let nth = nth
        .parse::<u64>()
        .expect("faulty input, should be a number from 0");
    let fib_at_nth = state.get_fib_at_nth(nth);
    println!("{}, {:?}", fib_at_nth, state);
    Html(format!("<h1>Fibonacci #{} is: {}</h1>", nth, fib_at_nth))
}

async fn fib_log(State(state): State<WebState>) -> impl IntoResponse {
    state.print_fib_log();
    let log_as_string = state.get_fib_log();
    Html(format!("<h1>Fibonacci Log: </h1><p>{}</p>", log_as_string))
}
