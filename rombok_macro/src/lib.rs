use proc_macro::{Ident, TokenStream, TokenTree};

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Getter(_: TokenStream, mut item: TokenStream) -> TokenStream {
    let struct_info = extract_struct_info(item.clone());

    if !struct_info.is_struct {
        panic!("The #[Getter] attribute can only be used on structs");
    }

    return item;
}

#[derive(Debug)]
struct StructInfo {
    struct_name: String,
    is_struct: bool,
    fields: Vec<(String, String)>,
}

fn extract_struct_info(item: TokenStream) -> StructInfo {
    let mut result = StructInfo {
        struct_name: String::new(),
        is_struct: false,
        fields: Vec::new(),
    };

    let mut iter = item.into_iter();

    while let Some(token) = iter.next() {
        println!("{:?}", token);
        if let TokenTree::Ident(ident) = token {
            if ident.to_string() == "struct" {
                result.is_struct = true;

                if let Some(TokenTree::Ident(name)) = iter.next() {
                    result.struct_name = name.to_string();
                }

                break;
            }
        }
    }

    if !result.is_struct {
        return result;
    }

    while let Some(token) = iter.next() {
        if let TokenTree::Group(group) = token {
            let mut group_iter = group.stream().into_iter().peekable();

            while let Some(token) = group_iter.next() {
                if let TokenTree::Ident(name) = token {
                    let field_name = name.to_string();
                    let mut type_name = String::new();

                    if let Some(TokenTree::Punct(punct)) = group_iter.next() {
                        if punct.as_char() == ':' {
                            loop {
                                let peeked = group_iter.peek();

                                match peeked {
                                    Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => {
                                        result.fields.push((field_name, type_name));
                                        break;
                                    }
                                    None => {
                                        result.fields.push((field_name, type_name));
                                        break;
                                    }
                                    Some(anything) => {
                                        type_name += &anything.to_string();
                                        group_iter.next();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    result
}
