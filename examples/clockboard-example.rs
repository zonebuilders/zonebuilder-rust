use std::env;

fn main() {
    let name = env::args().skip(1).next();
    println!("Hello, {}!", name.unwrap_or("world".into()));
}
