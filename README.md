# rombok

![](https://img.shields.io/badge/language-Rust-red) ![](https://img.shields.io/badge/version-0.3.0-brightgreen) [![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/myyrakle/rombok/blob/master/LICENSE)

boilerplate generation macros like lombok.

It automatically creates pattern implementations such as Getter, Setter, and Builder through attribute macros.


## Getter

The Getter macro generates `get_{fieldname}` and `get_{fieldname}_mut` methods for each field of the structure.
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

## Setter 

The Setter macro generates a `set_{fieldname}` method for each field in the structure.
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

## With 

With macro generates a `with_{fieldname}` method for each field in the structure.
Unlike Setter, it modifies the value through move and returns the changed object as the return value.
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

## Builder

The Builder macro generates a Builder pattern object for the structure, and adds a builder static method to the structure.
Each method in builder is implemented through the With pattern.
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

## AllArgsConstructor

This is an attribute macro that creates a constructor that initializes all fields of the structure.

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

## NoArgsConstructor

This is an attribute macro that creates a constructor method that does not receive arguments and initializes all fields to default.
The type of each field must implement the Default trait.

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

## EqualsAndHashcode

This is an attribute macro that generates the `equals` and `hashcode` methods for the structure. (Eq, Hash trait)

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

## ToString

This is an attribute macro that generates the `to_string` method for the structure. (Display, ToString, Debug trait)

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

## Value

This macro is a boilerplate combination of the following macros:: ToString + EqualsAndHashcode + Getter + AllArgsConstructor

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