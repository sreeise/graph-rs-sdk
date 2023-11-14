// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(CreatedByUserApiClient, ResourceIdentity::CreatedByUser);

impl CreatedByUserApiClient {
    get!(
        doc: "Get createdByUser from drives",
        name: get_created_by_user,
        path: "/createdByUser"
    );
    get!(
        doc: "Get mailboxSettings property value",
        name: get_mailbox_settings,
        path: "/createdByUser/mailboxSettings"
    );
    patch!(
        doc: "Update property mailboxSettings value.",
        name: update_mailbox_settings,
        path: "/createdByUser/mailboxSettings",
        body: true
    );
    get!(
        doc: "Get serviceProvisioningErrors property value",
        name: list_service_provisioning_errors,
        path: "/createdByUser/serviceProvisioningErrors"
    );
    get!(
        doc: "Get the number of the resource",
        name: service_provisioning_errors,
        path: "/createdByUser/serviceProvisioningErrors/$count"
    );
}
