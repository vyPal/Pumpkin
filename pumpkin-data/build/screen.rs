use heck::ToPascalCase;
use proc_macro2::TokenStream;
use quote::quote;

use crate::ident;

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=assets/screens.json");

    let screens: Vec<String> = serde_json::from_str(include_str!("../../assets/screens.json"))
        .expect("Failed to parse screens.json");
    let mut variants = TokenStream::new();

    for (id, screen) in screens.iter().enumerate() {
        let id = id as u8;
        let name = ident(screen.to_pascal_case());

        variants.extend([quote! {
            #name = #id,
        }]);
    }
    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum WindowType {
            #variants
        }
    }
}
