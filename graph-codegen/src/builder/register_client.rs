use bytes::Bytes;
use inflector::Inflector;

#[derive(Debug, Clone, Copy)]
pub enum RegisterClient {
    BaseClient,
    IdentClient,
}

impl RegisterClient {
    pub fn format(self, name: &str) -> Bytes {
        Bytes::copy_from_slice(self.register_client(name.as_ref()).as_bytes())
    }

    fn register_client(&self, client_name: &str) -> String {
        let ends_with = client_name.ends_with("Request");
        let client_pascal_casing = client_name.to_pascal_case();
        match self {
            RegisterClient::BaseClient => {
                if ends_with {
                    format!("register_client!({},);\n", client_pascal_casing)
                } else {
                    format!("register_client!({}Request,);\n", client_pascal_casing)
                }
            }
            RegisterClient::IdentClient => {
                if ends_with {
                    format!("register_client!({}, ());\n", client_pascal_casing)
                } else {
                    format!("register_client!({}Request, ());\n", client_pascal_casing)
                }
            }
        }
    }
}
