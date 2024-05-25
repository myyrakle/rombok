use rombok::Getter;

mod foo {
    pub struct Bar {
        a: u8,
        b: u8,
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
    println!("Hello, world!");
}

// TokenStream [
//     Ident { ident: "struct", span: #0 bytes(101..107) },
//     Ident { ident: "Person", span: #0 bytes(108..114) },
//     Group {
//         delimiter: Brace,
//         stream: TokenStream [
//             Ident { ident: "name", span: #0 bytes(121..125) },
//             Punct { ch: ':', spacing: Alone, span: #0 bytes(125..126) },
//             Ident { ident: "String", span: #0 bytes(127..133) },
//             Punct { ch: ',', spacing: Alone, span: #0 bytes(133..134) },
//             Ident { ident: "age", span: #0 bytes(139..142) },
//             Punct { ch: ':', spacing: Alone, span: #0 bytes(142..143) },
//             Ident { ident: "u8", span: #0 bytes(144..146) },
//             Punct { ch: ',', spacing: Alone, span: #0 bytes(146..147) },
//             Ident { ident: "money", span: #0 bytes(152..157) },
//             Punct { ch: ':', spacing: Alone, span: #0 bytes(157..158) },
//             Ident { ident: "Option", span: #0 bytes(159..165) },
//             Punct { ch: '<', spacing: Alone, span: #0 bytes(165..166) },
//             Ident { ident: "f64", span: #0 bytes(166..169) },
//             Punct { ch: '>', spacing: Joint, span: #0 bytes(169..170) },
//             Punct { ch: ',', spacing: Alone, span: #0 bytes(170..171) },
//             Ident { ident: "point", span: #0 bytes(176..181) },
//             Punct { ch: ':', spacing: Alone, span: #0 bytes(181..182) },
//             Group {
//                 delimiter: Parenthesis,
//                 stream: TokenStream [
//                     Ident {
//                         ident: "u8", span: #0 bytes(184..186)
//                     },
//                     Punct { ch: ',', spacing: Alone, span: #0 bytes(186..187) },
//                     Ident { ident: "u8", span: #0 bytes(188..190) }
//                 ],
//                 span: #0 bytes(183..191)
//             },
//             Punct { ch: ',', spacing: Alone, span: #0 bytes(191..192) },
//             Ident { ident: "bar", span: #0 bytes(197..200) },
//             Punct { ch: ':', spacing: Alone, span: #0 bytes(200..201) },
//             Ident { ident: "foo", span: #0 bytes(202..205) },
//             Punct { ch: ':', spacing: Joint, span: #0 bytes(205..206) },
//             Punct { ch: ':', spacing: Alone, span: #0 bytes(206..207) },
//             Ident { ident: "Bar", span: #0 bytes(207..210) },
//             Punct { ch: ',', spacing: Alone, span: #0 bytes(210..211) }
//         ],
//         span: #0 bytes(115..213)
//     }
// ]
