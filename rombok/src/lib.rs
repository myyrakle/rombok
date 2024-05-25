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

/// This is an attribute macro that adds a `with value` method to the structure.
///
/**
 * # Example
```rust
use rombok::With;

#[With]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: "".to_string(),
        age: 0,
    }.with_name("Jane".to_string()).with_age(31);

    let person = person.with_name("tom".to_string()).with_age(44);

    println!("Hello, world!: {}, {}", person.name, person.age);
}
```
*/
pub use rombok_macro::With;

/// This is an attribute macro that creates Builder classes and builder methods for the structure.
///
/// # Example
/**
 * # Example
```rust
use rombok::Builder;

#[Builder]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person::builder()
        .name("Jane".to_string())
        .age(31)
        .build();

    println!("Hello, world!: {}, {}", person.name, person.age);
}
```
*/
pub use rombok_macro::Builder;
