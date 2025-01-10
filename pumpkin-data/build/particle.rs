use heck::ToPascalCase;
use proc_macro2::TokenStream;
use quote::quote;

use crate::ident;

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=assets/particles.json");

    let particle: Vec<String> = serde_json::from_str(include_str!("../../assets/particles.json"))
        .expect("Failed to parse particles.json");
    let mut variants = TokenStream::new();

    for (id, particle) in particle.iter().enumerate() {
        let id = id as u8;
        let name = ident(particle.to_pascal_case());

        variants.extend([quote! {
            #name = #id,
        }]);
    }
    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum Particle {
            #variants
        }
    }
}
