use rombok_macro;

/// This is an attribute macro that adds a getter method to the structure.
///
/**
 * # Example
```rust
use rombok::Getter;

#[Getter]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: "John".to_string(),
        age: 30,
    };

    let name = person.get_name();
    let age = person.get_age();

    println!("Hello, world!: {name}, {age}");
}
```
*/
pub use rombok_macro::Getter;

/// This is an attribute macro that adds a setter method to the structure.
///
/**
 * # Example
```rust
use rombok::Setter;

#[Setter]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let mut person = Person {
        name: "John".to_string(),
        age: 30,
    };

    person.set_name("Jane".to_string());
    person.set_age(31);

    println!("Hello, world!: {}, {}", person.name, person.age);
}
```
*/
pub use rombok_macro::Setter;
