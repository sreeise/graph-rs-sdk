#![recursion_limit = "128"]
extern crate proc_macro;
extern crate quote;
extern crate serde_derive;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;

/// FromFile derive for the transform_request::FromFile trait.
#[proc_macro_derive(FromFile)]
pub fn derive_from_file(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl transform_request::FromFile for #name {
            type Err = transform_request::RequestError;

            fn from_file<P: AsRef<std::path::Path>>(path: P) -> std::result::Result<Self, Self::Err> {
                let f = std::fs::File::open(path)?;
                let self_as_json = serde_json::from_reader(f)?;
                Ok(self_as_json)
            }
        }
    };

    proc_macro::TokenStream::from(expanded).into()
}

/// ToFile derive for the transform_request::ToFile trait.
#[proc_macro_derive(ToFile)]
pub fn derive_to_file(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl transform_request::ToFile for #name {
            type Err = transform_request::RequestError;
            type Output = ();

            fn to_file<P: AsRef<std::path::Path>>(&self, path: P) -> std::result::Result<Self::Output, Self::Err> {
                if path.as_ref().exists() {
                    std::fs::remove_file(&path)?;
                }

                let mut file = std::fs::OpenOptions::new().create(true).write(true).open(&path)?;
                let serialized = serde_json::to_string(&self)?;
                file.write_all(serialized.as_bytes())?;
                file.sync_all()?;
                Ok(())
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
