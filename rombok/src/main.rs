#![allow(dead_code)]

use rombok::AllArgsConstructor;

mod foo {
    pub struct Bar {
        pub a: u8,
        pub b: u8,
    }
}

#[AllArgsConstructor]
struct Person {
    name: String,
    age: u8,
    money: Option<f64>,
    point: (u8, u8),
    bar: foo::Bar,
}

fn main() {
    let person = Person::with_all_args(
        "John".to_string(),
        30,
        Some(100.0),
        (10, 20),
        foo::Bar { a: 1, b: 2 },
    );

    let name = person.name;

    println!("Hello, world!: {name}");
}
