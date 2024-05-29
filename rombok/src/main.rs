#![allow(dead_code, unused_imports)]

use rombok::{AllArgsConstructor, EqualsAndHashcode, NoArgsConstructor, ToString};

mod foo {
    pub struct Bar {
        pub a: u8,
        pub b: u8,
    }
}

use std::hash::{DefaultHasher, Hash, Hasher};

#[ToString]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: "John".to_string(),
        age: 30,
    };

    println!("to_string: {}", person.to_string());
}
