use serde_json::{Value};

pub fn print(result: Value) {
    print_as_json(result);
}

fn print_as_json(result: Value) {
    println!("{:}", result);
}