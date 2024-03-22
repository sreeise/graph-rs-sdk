// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    CallRecordsSessionsApiClient,
    CallRecordsSessionsIdApiClient,
    ResourceIdentity::CallRecordsSessions
);

impl CallRecordsSessionsApiClient {
    post!(
        doc: "Create new navigation property to sessions for communications",
        name: create_sessions,
        path: "/sessions",
        body: true
    );
    get!(
        doc: "List callRecord sessions",
        name: list_sessions,
        path: "/sessions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_sessions_count,
        path: "/sessions/$count"
    );
}

impl CallRecordsSessionsIdApiClient {
    delete!(
        doc: "Delete navigation property sessions for communications",
        name: delete_sessions,
        path: "/sessions/{{RID}}"
    );
    get!(
        doc: "Get sessions from communications",
        name: get_sessions,
        path: "/sessions/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property sessions in communications",
        name: update_sessions,
        path: "/sessions/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to segments for communications",
        name: create_segments,
        path: "/sessions/{{RID}}/segments",
        body: true
    );
    get!(
        doc: "Get segments from communications",
        name: list_segments,
        path: "/sessions/{{RID}}/segments"
    );
    get!(
        doc: "Get the number of the resource",
        name: count,
        path: "/sessions/{{RID}}/segments/$count"
    );
    delete!(
        doc: "Delete navigation property segments for communications",
        name: delete_segments,
        path: "/sessions/{{RID}}/segments/{{id}}",
        params: segment_id
    );
    get!(
        doc: "Get segments from communications",
        name: get_segments,
        path: "/sessions/{{RID}}/segments/{{id}}",
        params: segment_id
    );
    patch!(
        doc: "Update the navigation property segments in communications",
        name: update_segments,
        path: "/sessions/{{RID}}/segments/{{id}}",
        body: true,
        params: segment_id
    );
}
