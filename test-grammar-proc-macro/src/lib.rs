extern crate proc_macro;
extern crate quote;
extern crate syn;

use std::path::Path;

use proc_macro2::Span;
use syn::{ext, parse_macro_input, Ident, LitStr};
#[proc_macro]
pub fn test_grammar_files_in_dir(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let constant_directory = parse_macro_input!(input as LitStr).value();
    let constant_directory = Path::new(&constant_directory);
    let file_name = "PowerManagement.fidl";
    let file_name = Path::new(file_name)
        .file_stem()
        .expect("Expected file to exist")
        .to_str()
        .expect("Should be able to convert to string");
    let file_name = file_name.to_lowercase();
    // constant_directory is a filepath for now
    let file_name = format!("test_grammar_file_{}", file_name);
    let test_ident = syn::Ident::new(&file_name, Span::call_site());

    quote::quote!(
            #[test]
            fn #test_ident(){
                let file_path = "../grammar_test_files/PowerManagement.fidl";
                let src = fs::read_to_string(file_path);
                let src = match src {
                    Err(err) => {
                        panic!("Error: {:?}", err)
                    }
                    Ok(src) => src,
                };
                let result = shared(&src, grammar::<BasicContext>, Rules::Grammar);
                assert_eq!(result, (true, src.len() as u32));
    })
    .into()
}

// let traits: Vec<&str> = vec!["std::default::Default", "std::fmt::Debug"];
// syn::parse_str::<syn::Type>(&traits.join(" + ")).unwrap();
