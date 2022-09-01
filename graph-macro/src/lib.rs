use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn;

#[proc_macro_attribute]
pub fn graph_rs_json(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let name = &input.ident;

    let plural = format_ident!("{}s", name);

    let result = quote! {
        #input

        impl ODataNextLink<#name> for #name {
            fn next_link(&self) -> Option<String> {
                None
            }

            fn value(&mut self) -> &mut Vec<#name> {
                self.value()
            }
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct #plural {
            pub(crate) value: Vec<#name>,
            #[serde(rename = "@odata.nextLink")]
            pub(crate) next_link: Option<String>,
        }

        impl ODataNextLink<#name> for #plural {
            fn next_link(&self) -> Option<String> {
                self.next_link.clone()
            }

            fn value(&mut self) -> &mut Vec<#name> {
                &mut self.value
            }
        }
    };
    result.into()
}
