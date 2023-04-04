// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    DirectReportsApiClient,
    DirectReportsIdApiClient,
    ResourceIdentity::DirectReports
);

impl DirectReportsApiClient {
    get!(
        doc: "Get directReports from users",
        name: list_direct_reports,
        path: "/directReports"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_direct_reports_count,
        path: "/directReports/$count"
    );

    get!(
        doc: "Get the number of the resource",
        name: get_user_count,
        path: "/directReports/graph.user/$count"
    );
}

impl DirectReportsIdApiClient {
    get!(
        doc: "Get directReports from users",
        name: get_direct_reports,
        path: "/directReports/{{RID}}"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.orgContact",
        name: get_directory_object_item_as_org_contact_type,
        path: "/directReports/{{RID}}/graph.orgContact"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.user",
        name: get_directory_object_item_as_user_type,
        path: "/directReports/{{RID}}/graph.user"
    );
}
