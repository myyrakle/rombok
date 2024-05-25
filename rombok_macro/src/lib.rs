use std::str::FromStr;

use proc_macro::{TokenStream, TokenTree};

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Getter(_: TokenStream, mut item: TokenStream) -> TokenStream {
    let struct_info = extract_struct_info(item.clone());

    if !struct_info.is_struct {
        panic!("The #[Getter] attribute can only be used on structs");
    }

    let mut new_code = String::new();

    new_code += &format!("impl {} {{\n", struct_info.struct_name);

    for (field_name, type_name) in struct_info.fields {
        new_code += &format!("pub fn get_{}(&self) -> &{} {{\n", field_name, type_name);
        new_code += &format!("&self.{}\n", field_name);
        new_code += &format!("}}\n");

        new_code += &format!(
            "pub fn get_{}_mut(&mut self) -> &mut {} {{\n",
            field_name, type_name
        );
        new_code += &format!("&mut self.{}\n", field_name);
        new_code += &format!("}}\n");
    }

    new_code += &format!("}}\n");

    item.extend(TokenStream::from_str(&new_code).unwrap());

    return item;
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Setter(_: TokenStream, mut item: TokenStream) -> TokenStream {
    let struct_info = extract_struct_info(item.clone());

    if !struct_info.is_struct {
        panic!("The #[Setter] attribute can only be used on structs");
    }

    let mut new_code = String::new();

    new_code += &format!("impl {} {{\n", struct_info.struct_name);

    for (field_name, type_name) in struct_info.fields {
        new_code += &format!("pub fn set_{field_name}(&mut self, value: {type_name}) {{\n",);
        new_code += &format!("self.{field_name} = value;\n",);
        new_code += &format!("}}\n");
    }

    new_code += &format!("}}\n");

    item.extend(TokenStream::from_str(&new_code).unwrap());

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
