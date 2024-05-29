#![allow(dead_code, unused_imports)]

use rombok::{AllArgsConstructor, EqualsAndHashcode, NoArgsConstructor};

mod foo {
    pub struct Bar {
        pub a: u8,
        pub b: u8,
    }
}

use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug)]
#[EqualsAndHashcode]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: "John".to_string(),
        age: 30,
    };

    let person2 = Person {
        name: "Jane".to_string(),
        age: 30,
    };

    let mut hasher = DefaultHasher::new();
    person.hash(&mut hasher);
    let hashcode = hasher.finish();
    println!("hashcode: {:?}", hashcode);

    assert_ne!(person, person2);
}
