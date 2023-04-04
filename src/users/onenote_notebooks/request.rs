// GENERATED CODE

use crate::api_default_imports::*;
use crate::users::*;

resource_api_client!(
    OnenoteNotebooksApiClient,
    OnenoteNotebooksIdApiClient,
    ResourceIdentity::OnenoteNotebooks
);

impl OnenoteNotebooksApiClient {
    post!(
        doc: "Create notebook",
        name: create_notebooks,
        path: "/notebooks",
        body: true
    );
    get!(
        doc: "List notebooks",
        name: list_notebooks,
        path: "/notebooks"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_notebooks_count,
        path: "/notebooks/$count"
    );
    post!(
        doc: "Invoke action getNotebookFromWebUrl",
        name: get_notebook_from_web_url,
        path: "/notebooks/getNotebookFromWebUrl",
        body: true
    );
    get!(
        doc: "Invoke function getRecentNotebooks",
        name: get_recent_notebooks,
        path: "/notebooks/getRecentNotebooks(includePersonalNotebooks={{id}})",
        params: include_personal_notebooks
    );
}

impl OnenoteNotebooksIdApiClient {
    api_client_link_id!(section_group, OnenoteSectionGroupsIdApiClient);
    api_client_link!(section_groups, OnenoteSectionGroupsApiClient);
    api_client_link_id!(section, OnenoteSectionsIdApiClient);
    api_client_link!(sections, OnenoteSectionsApiClient);

    delete!(
        doc: "Delete navigation property notebooks for users",
        name: delete_notebooks,
        path: "/notebooks/{{RID}}"
    );
    get!(
        doc: "Get notebooks from users",
        name: get_notebooks,
        path: "/notebooks/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property notebooks in users",
        name: update_notebooks,
        path: "/notebooks/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action copyNotebook",
        name: copy_notebook,
        path: "/notebooks/{{RID}}/copyNotebook",
        body: true
    );
}
