#![allow(dead_code, unused_imports)]

use rombok::Value;

mod foo {
    pub struct Bar {
        pub a: u8,
        pub b: u8,
    }
}

use std::hash::{DefaultHasher, Hash, Hasher};

#[Value]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person::with_all_args("John".to_string(), 30);

    println!("age: {}", person.get_age());

    let person2 = Person::with_all_args("Jane".to_string(), 30);
    assert_ne!(person, person2);

    println!("person: {}", person.to_string());
}
