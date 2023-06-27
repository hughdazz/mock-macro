use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident};

#[proc_macro]
pub fn generate_code(input: TokenStream) -> TokenStream {
    // 解析输入的标识符
    let ident = parse_macro_input!(input as Ident);

    // 获取标识符的字符串值
    let ident_str = ident.to_string();

    // 获取标识符的一部分（假设我们要获取前两个字符）
    let partial_ident_str = &ident_str[..2];

    // 生成新的标识符
    let new_ident = format_ident!("{}_new", partial_ident_str);

    // 生成新的代码
    let generated_code = quote! {
        fn #new_ident() {
            println!("Generated code based on identifier: {}", #ident);
        }
    };

    // 将生成的代码转换为TokenStream并返回
    generated_code.into()
}
#[proc_macro]
pub fn get_type(input: TokenStream) -> TokenStream {
    // 解析输入的标识符
    let ident = parse_macro_input!(input as Ident);

    // 获取标识符的字符串值
    let ident_str = ident.to_string();

    // 分割标志符
    // _id_u64 -> u64
    let type_str = ident_str.split("splitsplitsplit").collect::<Vec<&str>>()[1].to_owned();

    // 获得类型
    let new_ident = format_ident!("{}", type_str);

    // 生成新的代码
    let generated_code = quote! {
        #new_ident
    };
    generated_code.into()
}
