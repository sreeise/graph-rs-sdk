macro_rules! api_client_impl {
    ($api_client_method_name:ident, $api_client_return_type:ty) => {
        api_client_impl_link!($api_client_method_name, $api_client_return_type);
    };

    ($api_client_method_name:ident, $api_client_return_type:ty, $api_client_method_name2:ident, $api_client_with_id_return_type:ty) => {
        api_client_impl_link!($api_client_method_name, $api_client_return_type);
        api_client_id_impl_link!($api_client_method_name2, $api_client_with_id_return_type);
    };
}

macro_rules! api_client_impl_link {
    ($name:ident, $return_type:ty) => {
        pub fn $name(&self) -> $return_type {
            let resource_identity = <$return_type as ResourceIdentifier>::resource_identifier();
            <$return_type>::new(
                self.client.clone(),
                ResourceProvisioner::resource_config_with_url(
                    self.endpoint.clone(),
                    resource_identity,
                ),
                Handlebars::new(),
            )
        }
    };

    ($name:ident, $return_type:ty, $resource_identity:expr) => {
        pub fn $name(&self) -> $return_type {
            <$return_type>::new(
                self.client.clone(),
                ResourceProvisioner::resource_config_with_url(
                    self.endpoint.clone(),
                    $resource_identity,
                ),
                Handlebars::new(),
            )
        }
    };
}

macro_rules! api_client_id_impl_link {
    ($name:ident, $return_type:ty) => {
        pub fn $name<S: AsRef<str>>(&self, id: S) -> $return_type {
            let resource_identity = <$return_type as ResourceIdentifier>::resource_identifier();
            let (resource_config, registry) =
                ResourceProvisioner::config_and_registry_with_id_and_url(
                    id.as_ref(),
                    self.endpoint.clone(),
                    resource_identity,
                );
            <$return_type>::new(self.client.clone(), resource_config, registry)
        }
    };

    ($name:ident, $return_type:ty, $resource_identity:expr) => {
        pub fn $name<S: AsRef<str>>(&self, id: S) -> $return_type {
            let (resource_config, registry) =
                ResourceProvisioner::config_and_registry_with_id_and_url(
                    id.as_ref(),
                    self.endpoint.clone(),
                    $resource_identity,
                );
            <$return_type>::new(self.client.clone(), resource_config, registry)
        }
    };
}

macro_rules! api_client_link {
    ($name:ident, $return_type:ty) => {
        pub fn $name(&self) -> $return_type {
            let resource_identity = <$return_type as ResourceIdentifier>::resource_identifier();
            let mut resource_config = self.resource_config.clone();

            if let Some(resource_identity_id) = resource_config.resource_identity_id.as_ref() {
                resource_config.url.extend_path(&[
                    resource_config.resource_identity.to_string(),
                    resource_identity_id.to_string(),
                ]);
            } else {
                resource_config
                    .url
                    .extend_path(&[resource_config.resource_identity.to_string()]);
            }

            resource_config.resource_identity_id = None;
            resource_config.resource_identity = resource_identity;
            <$return_type>::new(self.client.clone(), resource_config, Handlebars::new())
        }
    };

    ($name:ident, $resource_identity:expr, $return_type:ty) => {
        pub fn $name(&self) -> $return_type {
            let mut resource_config = self.resource_config.clone();

            if let Some(resource_identity_id) = resource_config.resource_identity_id.as_ref() {
                resource_config.url.extend_path(&[
                    resource_config.resource_identity.to_string(),
                    resource_identity_id.to_string(),
                ]);
            } else {
                resource_config
                    .url
                    .extend_path(&[resource_config.resource_identity.to_string()]);
            }

            resource_config.resource_identity_id = None;
            resource_config.resource_identity = $resource_identity;
            <$return_type>::new(self.client.clone(), resource_config, Handlebars::new())
        }
    };

    ($name:ident, $resource_path:expr, $resource_identity:expr, $return_type:ty) => {
        pub fn $name(&self) -> $return_type {
            let mut resource_config = self.resource_config.clone();

            if let Some(resource_identity_id) = resource_config.resource_identity_id.as_ref() {
                resource_config
                    .url
                    .extend_path(&[$resource_path.to_string(), resource_identity_id.to_string()]);
            } else {
                resource_config
                    .url
                    .extend_path(&[$resource_path.to_string()]);
            }

            resource_config.resource_identity_id = None;
            resource_config.resource_identity = $resource_identity;
            <$return_type>::new(self.client.clone(), resource_config, Handlebars::new())
        }
    };

    ($name:ident, $return_type:ty, $name_with_id:ident, $return_type_with_id:ty) => {
        api_client_link!($name, $return_type);
        api_client_link_id!($name_with_id, $return_type_with_id);
    };
}

macro_rules! api_client_link_id {
    ($name:ident, $return_type:ty) => {
        pub fn $name<ID: AsRef<str>>(&self, id: ID) -> $return_type {
            let resource_identity = <$return_type as ResourceIdentifier>::resource_identifier();
            let mut resource_config = self.resource_config.clone();

            if let Some(resource_identity_id) = resource_config.resource_identity_id.as_ref() {
                resource_config.url.extend_path(&[
                    resource_config.resource_identity.to_string(),
                    resource_identity_id.to_string(),
                ]);
            } else {
                resource_config
                    .url
                    .extend_path(&[resource_config.resource_identity.to_string()]);
            }

            let id_str = id.as_ref();
            resource_config.resource_identity = resource_identity;
            resource_config.resource_identity_id = Some(id_str.to_string());
            <$return_type>::new(
                self.client.clone(),
                resource_config,
                ResourceProvisioner::registry_with_id(id_str),
            )
        }
    };

    ($name:ident, $resource_identity:expr, $return_type:ty) => {
        pub fn $name<ID: AsRef<str>>(&self, id: ID) -> $return_type {
            let mut resource_config = self.resource_config.clone();

            if let Some(resource_identity_id) = resource_config.resource_identity_id.as_ref() {
                resource_config.url.extend_path(&[
                    resource_config.resource_identity.to_string(),
                    resource_identity_id.to_string(),
                ]);
            } else {
                resource_config
                    .url
                    .extend_path(&[resource_config.resource_identity.to_string()]);
            }

            let id_str = id.as_ref();
            resource_config.resource_identity = $resource_identity;
            resource_config.resource_identity_id = Some(id_str.to_string());
            <$return_type>::new(
                self.client.clone(),
                resource_config,
                ResourceProvisioner::registry_with_id(id_str),
            )
        }
    };

    ($name:ident, $resource_path:expr, $resource_identity:expr, $return_type:ty) => {
        pub fn $name<ID: AsRef<str>>(&self, id: ID) -> $return_type {
            let mut resource_config = self.resource_config.clone();

            if let Some(resource_identity_id) = resource_config.resource_identity_id.as_ref() {
                resource_config
                    .url
                    .extend_path(&[$resource_path.to_string(), resource_identity_id.to_string()]);
            } else {
                resource_config
                    .url
                    .extend_path(&[$resource_path.to_string()]);
            }

            let id_str = id.as_ref();
            resource_config.resource_identity = $resource_identity;
            resource_config.resource_identity_id = Some(id_str.to_string());
            <$return_type>::new(
                self.client.clone(),
                resource_config,
                ResourceProvisioner::registry_with_id(id_str),
            )
        }
    };
}

macro_rules! resource_id_tunnel {
    ($return_type:ty) => {
        pub fn id<ID: AsRef<str>>(&self, id: ID) -> $return_type {
            let mut resource_config = self.resource_config.clone();
            resource_config.resource_identity_id = Some(id.as_ref().to_string());
            <$return_type>::new(
                self.client.clone(),
                resource_config,
                ResourceProvisioner::registry_with_id(id.as_ref()),
            )
        }
    };

    ($resource_identity:expr, $return_type:ty) => {
        pub fn id<ID: AsRef<str>>(&self, id: ID) -> $return_type {
            let mut resource_config = self.resource_config.clone();
            let id_str = id.as_ref();
            resource_config.resource_identity = $resource_identity;
            resource_config.resource_identity_id = Some(id_str.to_string());
            <$return_type>::new(
                self.client.clone(),
                resource_config,
                ResourceProvisioner::registry_with_id(id_str),
            )
        }
    };

    ($name:ident, $resource_identity:expr, $return_type:ty) => {
        pub fn $name<ID: AsRef<str>>(&self, id: ID) -> $return_type {
            let mut resource_config = self.resource_config.clone();

            if let Some(resource_identity_id) = resource_config.resource_identity_id.as_ref() {
                resource_config.url.extend_path(&[
                    resource_config.resource_identity.to_string(),
                    resource_identity_id.to_string(),
                ]);
            } else {
                resource_config
                    .url
                    .extend_path(&[resource_config.resource_identity.to_string()]);
            }

            let id_str = id.as_ref();
            resource_config.resource_identity = $resource_identity;
            resource_config.resource_identity_id = Some(id_str.to_string());
            <$return_type>::new(
                self.client.clone(),
                resource_config,
                ResourceProvisioner::registry_with_id(id_str),
            )
        }
    };
}
