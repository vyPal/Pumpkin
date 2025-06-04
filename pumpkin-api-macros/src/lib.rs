use proc_macro::TokenStream;
use quote::quote;
use std::sync::LazyLock;
use std::sync::Mutex;
use syn::{ImplItem, ItemFn, ItemImpl, ItemStruct, parse_macro_input, parse_quote};

// Store function names that should be included in the plugin impl
static PLUGIN_METHOD_NAMES: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[proc_macro_attribute]
pub fn plugin_method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_clone = item.clone();
    let input_fn = parse_macro_input!(item_clone as ItemFn);
    let fn_name = input_fn.sig.ident.to_string();

    PLUGIN_METHOD_NAMES.lock().unwrap().push(fn_name);

    item
}

#[proc_macro_attribute]
pub fn plugin_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input struct
    let input_struct = parse_macro_input!(item as ItemStruct);
    let struct_ident = &input_struct.ident;

    let method_names = PLUGIN_METHOD_NAMES.lock().unwrap().clone();

    let wrapper_methods = method_names.iter().map(|name| {
        let fn_ident = syn::Ident::new(name, proc_macro2::Span::call_site());

        quote! {
            async fn #fn_ident(&self, args: serde_json::Value) -> Result<serde_json::Value, pumpkin::plugin::Error> {
                crate::GLOBAL_RUNTIME.block_on(async move {
                    self.#fn_ident(args).await
                })
            }
        }
    });

    // Combine the original struct definition with the impl block and plugin() function
    let expanded = quote! {
        pub static GLOBAL_RUNTIME: std::sync::LazyLock<std::sync::Arc<tokio::runtime::Runtime>> =
            std::sync::LazyLock::new(|| std::sync::Arc::new(tokio::runtime::Runtime::new().unwrap()));

        #[unsafe(no_mangle)]
        pub static METADATA: pumpkin::plugin::PluginMetadata = pumpkin::plugin::PluginMetadata {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            authors: env!("CARGO_PKG_AUTHORS"),
            description: env!("CARGO_PKG_DESCRIPTION"),
        };

        #input_struct

        #[async_trait::async_trait]
        impl pumpkin::plugin::Plugin for #struct_ident {
            #(#wrapper_methods)*
        }

        #[unsafe(no_mangle)]
        pub fn plugin() -> Box<dyn pumpkin::plugin::Plugin> {
            Box::new(#struct_ident::new())
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn with_runtime(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemImpl);

    let use_global = attr.to_string() == "global";

    for item in &mut input.items {
        if let ImplItem::Fn(method) = item {
            let original_body = &method.block;

            method.block = if use_global {
                parse_quote!({
                    crate::GLOBAL_RUNTIME.block_on(async move {
                        #original_body
                    })
                })
            } else {
                parse_quote!({
                    tokio::runtime::Runtime::new()
                        .unwrap()
                        .block_on(async move {
                            #original_body
                        })
                })
            };
        }
    }

    TokenStream::from(quote!(#input))
}
