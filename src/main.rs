use std::env;

mod app;

fn main() {
    let args: Vec<String> = env::args().collect();
    app::run_app(args);
}
