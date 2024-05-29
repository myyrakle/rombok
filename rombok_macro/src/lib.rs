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

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn With(_: TokenStream, mut item: TokenStream) -> TokenStream {
    let struct_info = extract_struct_info(item.clone());

    if !struct_info.is_struct {
        panic!("The #[With] attribute can only be used on structs");
    }

    let mut new_code = String::new();

    new_code += &format!("impl {} {{\n", struct_info.struct_name);

    for (field_name, type_name) in struct_info.fields {
        new_code +=
            &format!("pub fn with_{field_name}(mut self, value: {type_name}) -> Self {{\n",);
        new_code += &format!("self.{field_name} = value;\n",);
        new_code += &format!("self\n");
        new_code += &format!("}}\n");
    }

    new_code += &format!("}}\n");

    item.extend(TokenStream::from_str(&new_code).unwrap());

    return item;
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Builder(_: TokenStream, mut item: TokenStream) -> TokenStream {
    let struct_info = extract_struct_info(item.clone());

    if !struct_info.is_struct {
        panic!("The #[Builder] attribute can only be used on structs");
    }

    let mut new_code = String::new();

    new_code += &format!("impl {} {{\n", struct_info.struct_name);

    new_code += &format!(
        "pub fn builder() -> {}Builder {{\n",
        struct_info.struct_name
    );
    new_code += &format!("{}Builder::default()\n", struct_info.struct_name);
    new_code += &format!("}}\n");

    new_code += &format!("}}\n");

    new_code += &format!(
        "#[derive(Debug, Default)] 
pub struct {}Builder {{\n",
        struct_info.struct_name
    );

    for (field_name, type_name) in &struct_info.fields {
        new_code += &format!("{}: Option<{}>,\n", field_name, type_name);
    }

    new_code += &format!("}}\n");

    new_code += &format!("impl {}Builder {{\n", struct_info.struct_name);

    for (field_name, type_name) in &struct_info.fields {
        new_code += &format!("pub fn {field_name}(mut self, value: {type_name}) -> Self {{\n",);
        new_code += &format!("self.{field_name} = Some(value);\n",);
        new_code += &format!("self\n");
        new_code += &format!("}}\n");
    }

    new_code += &format!("pub fn build(self) -> {} {{\n", struct_info.struct_name);
    new_code += &format!("{} {{\n", struct_info.struct_name);

    for (field_name, _) in &struct_info.fields {
        new_code += &format!("{field_name}: self.{field_name}.unwrap_or_default(),\n",);
    }

    new_code += &format!("}}\n");
    new_code += &format!("}}\n");

    new_code += &format!("}}\n");

    item.extend(TokenStream::from_str(&new_code).unwrap());

    return item;
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn AllArgsConstructor(_: TokenStream, mut item: TokenStream) -> TokenStream {
    let struct_info = extract_struct_info(item.clone());

    if !struct_info.is_struct {
        panic!("The #[AllArgsConstructor] attribute can only be used on structs");
    }

    let mut new_code = String::new();

    new_code += &format!("impl {} {{\n", struct_info.struct_name);

    let mut args = String::new();

    for (field_name, type_name) in &struct_info.fields {
        args += &format!("{field_name}: {type_name}, ",);
    }

    args = args.trim_end_matches(", ").to_string();

    new_code += &format!("  pub fn with_all_args({}) -> Self {{\n", args);
    new_code += &format!("      Self {{\n");

    for (field_name, _) in &struct_info.fields {
        new_code += &format!("      {field_name},\n",);
    }

    new_code += &format!("      }}\n");
    new_code += &format!("  }}\n");

    new_code += &format!("}}\n");

    item.extend(TokenStream::from_str(&new_code).unwrap());

    return item;
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn NoArgsConstructor(_: TokenStream, mut item: TokenStream) -> TokenStream {
    let struct_info = extract_struct_info(item.clone());

    if !struct_info.is_struct {
        panic!("The #[NoArgsConstructor] attribute can only be used on structs");
    }

    let mut new_code = String::new();

    new_code += &format!("impl {} {{\n", struct_info.struct_name);

    new_code += &format!("  pub fn with_no_args() -> Self {{\n");
    new_code += &format!("      Self {{\n");

    for (field_name, _) in &struct_info.fields {
        new_code += &format!("      {field_name}: Default::default(),\n",);
    }

    new_code += &format!("      }}\n");
    new_code += &format!("  }}\n");

    new_code += &format!("}}\n");

    item.extend(TokenStream::from_str(&new_code).unwrap());

    return item;
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn EqualsAndHashcode(_: TokenStream, item: TokenStream) -> TokenStream {
    let struct_info = extract_struct_info(item.clone());

    if !struct_info.is_struct {
        panic!("The #[EqualsAndHashcode] attribute can only be used on structs");
    }

    let before_code = format!("#[derive(PartialEq, Hash)]\n");

    let after_code = format!("impl Eq for {} {{}}\n", struct_info.struct_name);

    let mut result = TokenStream::from_str(&before_code).unwrap();

    result.extend(item);

    result.extend(TokenStream::from_str(&after_code).unwrap());

    result
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn ToString(_: TokenStream, mut item: TokenStream) -> TokenStream {
    let struct_info = extract_struct_info(item.clone());

    if !struct_info.is_struct {
        panic!("The #[ToString] attribute can only be used on structs");
    }

    let mut new_code = String::new();

    new_code += &format!(
        "impl std::fmt::Display for {} {{\n",
        struct_info.struct_name
    );

    new_code += &format!("fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{\n");

    new_code += &format!("write!(f, \"{} {{", struct_info.struct_name);

    for (field_name, _) in &struct_info.fields {
        new_code += &format!(" {}: {{}},", field_name);
    }

    new_code += &format!("}}\"");

    for (field_name, _) in &struct_info.fields {
        new_code += &format!(", self.{}", field_name);
    }

    new_code += &format!(")\n");

    new_code += &format!("}}\n");

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
