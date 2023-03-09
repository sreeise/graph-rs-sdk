// GENERATED CODE

use crate::api_default_imports::*;
use crate::communications::*;

resource_api_client!(
    CallRecordsApiClient,
    CallRecordsIdApiClient,
    ResourceIdentity::CallRecords
);

impl CallRecordsApiClient {
    post!(
        doc: "Create new navigation property to callRecords for communications",
        name: create_call_records,
        path: "/callRecords",
        body: true
    );
    get!(
        doc: "Get callRecords from communications",
        name: list_call_records,
        path: "/callRecords"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_call_records_count,
        path: "/callRecords/$count"
    );
    get!(
        doc: "Invoke function getDirectRoutingCalls",
        name: get_direct_routing_calls,
        path: "/callRecords/callRecords.getDirectRoutingCalls(fromDateTime={{id}},toDateTime={{id2}})",
        params: from_date_time, to_date_time
    );
    get!(
        doc: "Invoke function getPstnCalls",
        name: get_pstn_calls,
        path: "/callRecords/callRecords.getPstnCalls(fromDateTime={{id}},toDateTime={{id2}})",
        params: from_date_time, to_date_time
    );
}

impl CallRecordsIdApiClient {
    api_client_link!(sessions, CallRecordsSessionsApiClient);
    api_client_link_id!(session, CallRecordsSessionsIdApiClient);

    delete!(
        doc: "Delete navigation property callRecords for communications",
        name: delete_call_records,
        path: "/callRecords/{{RID}}"
    );
    get!(
        doc: "Get callRecords from communications",
        name: get_call_records,
        path: "/callRecords/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property callRecords in communications",
        name: update_call_records,
        path: "/callRecords/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to sessions for communications",
        name: create_sessions,
        path: "/callRecords/{{RID}}/sessions",
        body: true
    );
    get!(
        doc: "List callRecord sessions",
        name: list_sessions,
        path: "/callRecords/{{RID}}/sessions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_sessions_count,
        path: "/callRecords/{{RID}}/sessions/$count"
    );
    delete!(
        doc: "Delete navigation property sessions for communications",
        name: delete_sessions,
        path: "/callRecords/{{RID}}/sessions/{{id}}",
        params: session_id
    );
    get!(
        doc: "Get sessions from communications",
        name: get_sessions,
        path: "/callRecords/{{RID}}/sessions/{{id}}",
        params: session_id
    );
    patch!(
        doc: "Update the navigation property sessions in communications",
        name: update_sessions,
        path: "/callRecords/{{RID}}/sessions/{{id}}",
        body: true,
        params: session_id
    );
    post!(
        doc: "Create new navigation property to segments for communications",
        name: create_segments,
        path: "/callRecords/{{RID}}/sessions/{{id}}/segments",
        body: true,
        params: session_id
    );
    get!(
        doc: "Get segments from communications",
        name: list_segments,
        path: "/callRecords/{{RID}}/sessions/{{id}}/segments",
        params: session_id
    );
    get!(
        doc: "Get the number of the resource",
        name: count,
        path: "/callRecords/{{RID}}/sessions/{{id}}/segments/$count",
        params: session_id
    );
    delete!(
        doc: "Delete navigation property segments for communications",
        name: delete_segments,
        path: "/callRecords/{{RID}}/sessions/{{id}}/segments/{{id2}}",
        params: session_id, segment_id
    );
    get!(
        doc: "Get segments from communications",
        name: get_segments,
        path: "/callRecords/{{RID}}/sessions/{{id}}/segments/{{id2}}",
        params: session_id, segment_id
    );
    patch!(
        doc: "Update the navigation property segments in communications",
        name: update_segments,
        path: "/callRecords/{{RID}}/sessions/{{id}}/segments/{{id2}}",
        body: true,
        params: session_id, segment_id
    );
}
