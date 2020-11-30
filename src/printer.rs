use serde_json;
use serde::{Serialize};

pub fn print<T: Serialize>(result: T) {
    print_as_json(result);
}

fn print_as_json<T: Serialize>(result: T) {
    println!("{:}", serde_json::to_value(result).unwrap());
}