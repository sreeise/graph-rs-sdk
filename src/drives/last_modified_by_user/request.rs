// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    LastModifiedByUserApiClient,
    ResourceIdentity::LastModifiedByUser
);

impl LastModifiedByUserApiClient {
    get!(
        doc: "Get lastModifiedByUser from drives",
        name: get_last_modified_by_user,
        path: "/lastModifiedByUser"
    );
    get!(
        doc: "Get mailboxSettings property value",
        name: get_mailbox_settings,
        path: "/lastModifiedByUser/mailboxSettings"
    );
    patch!(
        doc: "Update property mailboxSettings value.",
        name: update_mailbox_settings,
        path: "/lastModifiedByUser/mailboxSettings",
        body: true
    );
    get!(
        doc: "Get serviceProvisioningErrors property value",
        name: list_service_provisioning_errors,
        path: "/lastModifiedByUser/serviceProvisioningErrors"
    );
    get!(
        doc: "Get the number of the resource",
        name: service_provisioning_errors,
        path: "/lastModifiedByUser/serviceProvisioningErrors/$count"
    );
}
