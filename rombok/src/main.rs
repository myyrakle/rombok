use rombok::Getter;

mod foo {
    pub struct Bar {
        pub a: u8,
        pub b: u8,
    }
}

#[Getter]
struct Person {
    name: String,
    age: u8,
    money: Option<f64>,
    point: (u8, u8),
    bar: foo::Bar,
}

fn main() {
    let p = Person {
        name: "John".to_string(),
        age: 30,
        money: Some(2500.50),
        point: (10, 20),
        bar: foo::Bar { a: 1, b: 2 },
    };

    let foo = p.get_name();

    println!("Hello, world!: {foo}");
}
