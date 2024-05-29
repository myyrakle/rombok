#![allow(dead_code, unused_imports)]

use rombok::{AllArgsConstructor, EqualsAndHashcode, NoArgsConstructor};

mod foo {
    pub struct Bar {
        pub a: u8,
        pub b: u8,
    }
}

#[NoArgsConstructor]
#[EqualsAndHashcode]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person::with_no_args();

    let age = person.age;

    println!("Hello, world!: {age:?}");
}
