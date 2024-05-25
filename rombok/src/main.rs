use rombok::Getter;

#[Getter]
struct Person {
    name: String,
    age: u8,
    money: Option<f64>,
}

fn main() {
    println!("Hello, world!");
}

// TokenStream [
//     Ident { ident: "struct", span: #0 bytes(31..37) },
//     Ident { ident: "Person", span: #0 bytes(38..44) },
//     Group {
//         delimiter: Brace,
//         stream: TokenStream [
//             Ident { ident: "name", span: #0 bytes(51..55) },
//             Punct { ch: ':', spacing: Alone, span: #0 bytes(55..56) },
//             Ident { ident: "String", span: #0 bytes(57..63) },
//             Punct { ch: ',', spacing: Alone, span: #0 bytes(63..64) },
//             Ident { ident: "age", span: #0 bytes(69..72) },
//             Punct { ch: ':', spacing: Alone, span: #0 bytes(72..73) },
//             Ident { ident: "u8", span: #0 bytes(74..76) },
//             Punct { ch: ',', spacing: Alone, span: #0 bytes(76..77) },
//             Ident { ident: "money", span: #0 bytes(82..87) },
//             Punct { ch: ':', spacing: Alone, span: #0 bytes(87..88) },
//             Ident { ident: "Option", span: #0 bytes(89..95) },
//             Punct { ch: '<', spacing: Alone, span: #0 bytes(95..96) },
//             Ident { ident: "f64", span: #0 bytes(96..99) },
//             Punct { ch: '>', spacing: Joint, span: #0 bytes(99..100) },
//             Punct { ch: ',', spacing: Alone, span: #0 bytes(100..101) }
//         ],
//         span: #0 bytes(45..103) }
//     ]
