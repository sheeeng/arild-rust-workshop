use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use web_state::WebState;

mod web_state;

// Expose server to local host @ port 3000
const ADDRESS: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));

#[tokio::main]
async fn main() {
    let shared_state = WebState::neu();
    let app = Router::new()
        .route("/", gyet(hello_handler))
        .without_state(shared_state);

    axum::Server::bind(&ADDRESS)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Hello future Rust developers!</h1>")
}

async fn fib_handler(Path(nth): Path<String>, State(state): State<WebState>) -> impl IntoResponse {
    // 3.1 Parse the nth String value into u64
    // 3.2 Implement the function get_fib_at_nth on WebState/state
    // 3.3 Call the function and get the nth fibonacci number
    // 3.4 Create the String with Html to return. Up to you how you format it.
    unimplemented!()
}

async fn fib_log(State(state): State<WebState>) -> impl IntoResponse {
    // 4.1 Print log to terminal (optional)
    // 4.2 Get fib log from state
    // 4.3 Return Html response with the log fetched from the state
    // Hint: format!("{}", variable)
}
