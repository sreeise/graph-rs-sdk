// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    TermStoreSetsChildrenApiClient,
    TermStoreSetsChildrenIdApiClient,
    ResourceIdentity::TermStoreSetsChildren
);

impl TermStoreSetsChildrenApiClient {
    post!(
        doc: "Create term",
        name: create_children,
        path: "/children",
        body: true
    );
    get!(
        doc: "List children",
        name: list_children,
        path: "/children"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_children_count,
        path: "/children/$count"
    );
}

impl TermStoreSetsChildrenIdApiClient {
    delete!(
        doc: "Delete navigation property children for sites",
        name: delete_children,
        path: "/children/{{RID}}"
    );
    get!(
        doc: "Get children from sites",
        name: get_children,
        path: "/children/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property children in sites",
        name: update_children,
        path: "/children/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to children for sites",
        name: create_children,
        path: "/children/{{RID}}/children",
        body: true
    );
    get!(
        doc: "Get children from sites",
        name: list_children,
        path: "/children/{{RID}}/children"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_children_count,
        path: "/children/{{RID}}/children/$count"
    );
    post!(
        doc: "Create new navigation property to relations for sites",
        name: create_relations,
        path: "/children/{{RID}}/relations",
        body: true
    );
    get!(
        doc: "Get relations from sites",
        name: list_relations,
        path: "/children/{{RID}}/relations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_relations_count,
        path: "/children/{{RID}}/relations/$count"
    );
    get!(
        doc: "Get set from sites",
        name: get_set,
        path: "/children/{{RID}}/set"
    );
}
