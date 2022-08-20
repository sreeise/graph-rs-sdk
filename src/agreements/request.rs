// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(AgreementsRequest,);
register_client!(AgreementsIdRequest, ());

impl<'a, Client> AgreementsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Add new entity to agreements",
        name: create_agreement,
        response: serde_json::Value,
        path: "/agreements",
        has_body: true
    });
    get!({
        doc: "Get entities from agreements",
        name: list_agreement,
        response: serde_json::Value,
        path: "/agreements",
        has_body: false
    });
}

impl<'a, Client> AgreementsIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    delete!({
        doc: "Delete entity from agreements",
        name: delete_agreement,
        response: NoContent,
        path: "/agreements/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get entity from agreements by key",
        name: get_agreement,
        response: serde_json::Value,
        path: "/agreements/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update entity in agreements",
        name: update_agreement,
        response: NoContent,
        path: "/agreements/{{RID}}",
        has_body: true
    });
    post!({
        doc: "Create new navigation property to acceptances for agreements",
        name: create_acceptances,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/acceptances",
        has_body: true
    });
    get!({
        doc: "Get acceptances from agreements",
        name: list_acceptances,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/acceptances",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_acceptances_count,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/acceptances/$count",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property acceptances for agreements",
        name: delete_acceptances,
        response: NoContent,
        path: "/agreements/{{RID}}/acceptances/{{id}}",
        params: [ agreement_acceptance_id ],
        has_body: false
    });
    get!({
        doc: "Get acceptances from agreements",
        name: get_acceptances,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/acceptances/{{id}}",
        params: [ agreement_acceptance_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property acceptances in agreements",
        name: update_acceptances,
        response: NoContent,
        path: "/agreements/{{RID}}/acceptances/{{id}}",
        params: [ agreement_acceptance_id ],
        has_body: true
    });
    delete!({
        doc: "Delete navigation property file for agreements",
        name: delete_file,
        response: NoContent,
        path: "/agreements/{{RID}}/file",
        has_body: false
    });
    get!({
        doc: "Get file from agreements",
        name: get_file,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/file",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property file in agreements",
        name: update_file,
        response: NoContent,
        path: "/agreements/{{RID}}/file",
        has_body: true
    });
    post!({
        doc: "Create new navigation property to localizations for agreements",
        name: create_localizations,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/file/localizations",
        has_body: true
    });
    get!({
        doc: "Get localizations from agreements",
        name: list_localizations,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/file/localizations",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_localizations_count,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/file/localizations/$count",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property localizations for agreements",
        name: delete_localizations,
        response: NoContent,
        path: "/agreements/{{RID}}/file/localizations/{{id}}",
        params: [ agreement_file_localization_id ],
        has_body: false
    });
    get!({
        doc: "Get localizations from agreements",
        name: get_localizations,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/file/localizations/{{id}}",
        params: [ agreement_file_localization_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property localizations in agreements",
        name: update_localizations,
        response: NoContent,
        path: "/agreements/{{RID}}/file/localizations/{{id}}",
        params: [ agreement_file_localization_id ],
        has_body: true
    });
    post!({
        doc: "Create new navigation property to versions for agreements",
        name: create_localizations_versions,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/file/localizations/{{id}}/versions",
        params: [ agreement_file_localization_id ],
        has_body: true
    });
    get!({
        doc: "Get versions from agreements",
        name: list_localizations_versions,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/file/localizations/{{id}}/versions",
        params: [ agreement_file_localization_id ],
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_localizations_version_count,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/file/localizations/{{id}}/versions/$count",
        params: [ agreement_file_localization_id ],
        has_body: false
    });
    delete!({
        doc: "Delete navigation property versions for agreements",
        name: delete_localizations_versions,
        response: NoContent,
        path: "/agreements/{{RID}}/file/localizations/{{id}}/versions/{{id2}}",
        params: [ agreement_file_localization_id  agreement_file_version_id ],
        has_body: false
    });
    get!({
        doc: "Get versions from agreements",
        name: get_localizations_versions,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/file/localizations/{{id}}/versions/{{id2}}",
        params: [ agreement_file_localization_id  agreement_file_version_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property versions in agreements",
        name: update_localizations_versions,
        response: NoContent,
        path: "/agreements/{{RID}}/file/localizations/{{id}}/versions/{{id2}}",
        params: [ agreement_file_localization_id  agreement_file_version_id ],
        has_body: true
    });
    post!({
        doc: "Create new navigation property to files for agreements",
        name: create_files,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/files",
        has_body: true
    });
    get!({
        doc: "Get files from agreements",
        name: list_files,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/files",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_files_count,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/files/$count",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property files for agreements",
        name: delete_files,
        response: NoContent,
        path: "/agreements/{{RID}}/files/{{id}}",
        params: [ agreement_file_localization_id ],
        has_body: false
    });
    get!({
        doc: "Get files from agreements",
        name: get_files,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/files/{{id}}",
        params: [ agreement_file_localization_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property files in agreements",
        name: update_files,
        response: NoContent,
        path: "/agreements/{{RID}}/files/{{id}}",
        params: [ agreement_file_localization_id ],
        has_body: true
    });
    post!({
        doc: "Create new navigation property to versions for agreements",
        name: create_files_versions,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/files/{{id}}/versions",
        params: [ agreement_file_localization_id ],
        has_body: true
    });
    get!({
        doc: "Get versions from agreements",
        name: list_files_versions,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/files/{{id}}/versions",
        params: [ agreement_file_localization_id ],
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_files_versions_count,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/files/{{id}}/versions/$count",
        params: [ agreement_file_localization_id ],
        has_body: false
    });
    delete!({
        doc: "Delete navigation property versions for agreements",
        name: delete_files_versions,
        response: NoContent,
        path: "/agreements/{{RID}}/files/{{id}}/versions/{{id2}}",
        params: [ agreement_file_localization_id  agreement_file_version_id ],
        has_body: false
    });
    get!({
        doc: "Get versions from agreements",
        name: get_files_versions,
        response: serde_json::Value,
        path: "/agreements/{{RID}}/files/{{id}}/versions/{{id2}}",
        params: [ agreement_file_localization_id  agreement_file_version_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property versions in agreements",
        name: update_files_versions,
        response: NoContent,
        path: "/agreements/{{RID}}/files/{{id}}/versions/{{id2}}",
        params: [ agreement_file_localization_id  agreement_file_version_id ],
        has_body: true
    });
}
