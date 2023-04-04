// GENERATED CODE

use crate::api_default_imports::*;
use crate::users::*;

resource_api_client!(
    OnenoteSectionsApiClient,
    OnenoteSectionsIdApiClient,
    ResourceIdentity::OnenoteSections
);

impl OnenoteSectionsApiClient {
    post!(
        doc: "Create new navigation property to sections for users",
        name: create_sections,
        path: "/sections",
        body: true
    );
    get!(
        doc: "List sections",
        name: list_sections,
        path: "/sections"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_sections_count,
        path: "/sections/$count"
    );
}

impl OnenoteSectionsIdApiClient {
    api_client_link!(pages, OnenotePagesApiClient);
    api_client_link_id!(section, OnenoteSectionsIdApiClient);
    api_client_link!(sections, OnenoteSectionsApiClient);
    api_client_link_id!(page, OnenotePagesIdApiClient);

    delete!(
        doc: "Delete navigation property sections for users",
        name: delete_sections,
        path: "/sections/{{RID}}"
    );
    get!(
        doc: "Get sections from users",
        name: get_sections,
        path: "/sections/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property sections in users",
        name: update_sections,
        path: "/sections/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action copyToNotebook",
        name: copy_to_notebook,
        path: "/sections/{{RID}}/copyToNotebook",
        body: true
    );
    post!(
        doc: "Invoke action copyToSectionGroup",
        name: copy_to_section_group,
        path: "/sections/{{RID}}/copyToSectionGroup",
        body: true
    );
    get!(
        doc: "Get parentNotebook from users",
        name: get_parent_notebook,
        path: "/sections/{{RID}}/parentNotebook"
    );
    get!(
        doc: "Get parentSectionGroup from users",
        name: get_parent_section_group,
        path: "/sections/{{RID}}/parentSectionGroup"
    );
}
