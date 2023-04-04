// GENERATED CODE

use crate::api_default_imports::*;
use crate::users::*;

resource_api_client!(
    OnenoteSectionGroupsApiClient,
    OnenoteSectionGroupsIdApiClient,
    ResourceIdentity::OnenoteSectionGroups
);

impl OnenoteSectionGroupsApiClient {
    post!(
        doc: "Create new navigation property to sectionGroups for users",
        name: create_section_groups,
        path: "/sectionGroups",
        body: true
    );
    get!(
        doc: "List sectionGroups",
        name: list_section_groups,
        path: "/sectionGroups"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_section_groups_count,
        path: "/sectionGroups/$count"
    );
}

impl OnenoteSectionGroupsIdApiClient {
    api_client_link!(sections, OnenoteSectionsApiClient);
    api_client_link_id!(section, OnenoteSectionsIdApiClient);

    delete!(
        doc: "Delete navigation property sectionGroups for users",
        name: delete_section_groups,
        path: "/sectionGroups/{{RID}}"
    );
    get!(
        doc: "Get sectionGroups from users",
        name: get_section_groups,
        path: "/sectionGroups/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property sectionGroups in users",
        name: update_section_groups,
        path: "/sectionGroups/{{RID}}",
        body: true
    );
    get!(
        doc: "Get parentNotebook from users",
        name: get_parent_notebook,
        path: "/sectionGroups/{{RID}}/parentNotebook"
    );
    get!(
        doc: "Get parentSectionGroup from users",
        name: get_parent_section_group,
        path: "/sectionGroups/{{RID}}/parentSectionGroup"
    );
    get!(
        doc: "List sectionGroups",
        name: list_section_groups,
        path: "/sectionGroups/{{RID}}/sectionGroups"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_section_groups_count,
        path: "/sectionGroups/{{RID}}/sectionGroups/$count"
    );
    get!(
        doc: "Get sectionGroups from users",
        name: get_section_groups_section_group,
        path: "/sectionGroups/{{RID}}/sectionGroups/{{id}}",
        params: section_group_id_1
    );
}
