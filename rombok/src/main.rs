#![allow(dead_code)]

use rombok::{AllArgsConstructor, NoArgsConstructor};

mod foo {
    pub struct Bar {
        pub a: u8,
        pub b: u8,
    }
}

#[NoArgsConstructor]
struct Person {
    name: String,
    age: u8,
    money: Option<f64>,
}

fn main() {
    let person = Person::with_no_args();

    let money = person.money;

    println!("Hello, world!: {money:?}");
}
