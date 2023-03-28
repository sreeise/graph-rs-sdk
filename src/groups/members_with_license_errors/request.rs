// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    MembersWithLicenseErrorsApiClient,
    MembersWithLicenseErrorsIdApiClient,
    ResourceIdentity::MembersWithLicenseErrors
);

impl MembersWithLicenseErrorsApiClient {
    get!(
        doc: "Get membersWithLicenseErrors from groups",
        name: list_members_with_license_errors,
        path: "/membersWithLicenseErrors"
    );
    get!(
        doc: "Get the number of the resource",
        name: count,
        path: "/membersWithLicenseErrors/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.application in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_application_type,
        path: "/membersWithLicenseErrors/graph.application"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_application_count,
        path: "/membersWithLicenseErrors/graph.application/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.device in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_device_type,
        path: "/membersWithLicenseErrors/graph.device"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_count,
        path: "/membersWithLicenseErrors/graph.device/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.group in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_group_type,
        path: "/membersWithLicenseErrors/graph.group"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_group_count,
        path: "/membersWithLicenseErrors/graph.group/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.orgContact in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_org_contact_type,
        path: "/membersWithLicenseErrors/graph.orgContact"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_org_contact_count,
        path: "/membersWithLicenseErrors/graph.orgContact/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.servicePrincipal in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_service_principal_type,
        path: "/membersWithLicenseErrors/graph.servicePrincipal"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_service_principal_count,
        path: "/membersWithLicenseErrors/graph.servicePrincipal/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.user in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_user_type,
        path: "/membersWithLicenseErrors/graph.user"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_user_count,
        path: "/membersWithLicenseErrors/graph.user/$count"
    );
}

impl MembersWithLicenseErrorsIdApiClient {
    get!(
        doc: "Get membersWithLicenseErrors from groups",
        name: get_members_with_license_errors,
        path: "/membersWithLicenseErrors/{{RID}}"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.application",
        name: get_directory_object_item_as_application_type,
        path: "/membersWithLicenseErrors/{{RID}}/graph.application"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.device",
        name: get_directory_object_item_as_device_type,
        path: "/membersWithLicenseErrors/{{RID}}/graph.device"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.group",
        name: get_directory_object_item_as_group_type,
        path: "/membersWithLicenseErrors/{{RID}}/graph.group"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.orgContact",
        name: get_directory_object_item_as_org_contact_type,
        path: "/membersWithLicenseErrors/{{RID}}/graph.orgContact"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.servicePrincipal",
        name: get_directory_object_item_as_service_principal_type,
        path: "/membersWithLicenseErrors/{{RID}}/graph.servicePrincipal"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.user",
        name: get_directory_object_item_as_user_type,
        path: "/membersWithLicenseErrors/{{RID}}/graph.user"
    );
}
