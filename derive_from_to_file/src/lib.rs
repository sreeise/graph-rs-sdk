#![recursion_limit = "256"]
extern crate proc_macro;
extern crate quote;
extern crate serde_derive;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;

#[proc_macro_derive(FromToFile)]
pub fn derive_from_to(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl from_to_file::FromToFile for #name {
            type Err = graph_error::GraphFailure;
            type Output = ();

            fn to_json_file<P: AsRef<std::path::Path>>(&self, path: P) -> std::result::Result<Self::Output, Self::Err> {
                if path.as_ref().exists() {
                    std::fs::remove_file(&path)?;
                }

                let mut file = std::fs::OpenOptions::new().create(true).write(true).open(&path)?;
                let serialized = serde_json::to_string(&self)?;
                file.write_all(serialized.as_bytes())?;
                file.sync_all()?;
                Ok(())
            }

            fn from_json_file<P: AsRef<std::path::Path>>(path: P) -> std::result::Result<Self, Self::Err> {
                let f = std::fs::File::open(path)?;
                let self_as_json = serde_json::from_reader(f)?;
                Ok(self_as_json)
            }

            fn to_yaml_file<P: AsRef<std::path::Path>>(&self, path: P) -> std::result::Result<Self::Output, Self::Err> {
                if path.as_ref().exists() {
                    std::fs::remove_file(&path)?;
                }

                let mut file = std::fs::OpenOptions::new().create(true).write(true).open(&path)?;
                let serialized = serde_yaml::to_string(&self)?;
                file.write_all(serialized.as_bytes())?;
                file.sync_all()?;
                Ok(())
            }

            fn from_yaml_file<P: AsRef<std::path::Path>>(path: P) -> std::result::Result<Self, Self::Err> {
                let f = std::fs::File::open(path)?;
                let self_as_json = serde_yaml::from_reader(f)?;
                Ok(self_as_json)
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
