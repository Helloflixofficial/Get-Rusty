use tracing::{info, instrument};
use tracing_subscriber;
fn main() {
    tracing_subscriber::fmt::init();
    hello("hello".to_string());
}

#[instrument]
fn hello(s: String) {
    info!("{}", s);
}
