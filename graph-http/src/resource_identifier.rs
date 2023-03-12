use graph_core::resource::ResourceIdentity;

pub trait ResourceIdentifier {
    fn resource_identifier() -> ResourceIdentity;
}

pub trait ResourceIdentifierLink {}
