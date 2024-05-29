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

/// This is an attribute macro that creates a constructor that initializes all fields of the structure.
///
/// # Example
/**
```rust
use rombok::AllArgsConstructor;

#[AllArgsConstructor]
struct Person {
    name: String,
    age: u8,
    money: Option<f64>,
    point: (u8, u8),
}

fn main() {
    let person = Person::with_all_args(
        "John".to_string(),
        30,
        Some(100.0),
        (10, 20),
    );

    let name = person.name;

    println!("Hello, world!: {name}");
}
```
 */
pub use rombok_macro::AllArgsConstructor;

/// This is an attribute macro that creates a constructor method that does not receive arguments and initializes all fields to default.
///
/// # Example
/**
```rust
use rombok::NoArgsConstructor;

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
```
 */
pub use rombok_macro::NoArgsConstructor;

/// This is an attribute macro that generates the `equals` and `hashcode` methods for the structure. (Eq, Hash trait)
///
/// # Example
/**
```rust
use rombok::EqualsAndHashcode;

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
```
 */
pub use rombok_macro::EqualsAndHashcode;

/// This is an attribute macro that generates the `to_string` method for the structure. (Display, ToString, Debug trait)
///
/// # Example
/**
```rust
use rombok::ToString;

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
```
 */
pub use rombok_macro::ToString;

/// This macro is a boilerplate combination of the following macros:: ToString + EqualsAndHashcode + Getter + AllArgsConstructor
///
/// # Example
/**
```rust
use rombok::Value;

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
```
 */
pub use rombok_macro::Value;
