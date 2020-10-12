use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::{GraphResponse, IntoResponse};
use reqwest::Method;

register_client!(CommunicationsRequest,);
register_client!(ParticipantsRequest,);
register_client!(OnlineMeetingsRequest,);
register_client!(CallsRequest,);
register_client!(CloudCommunicationsRequest,);

#[allow(dead_code)]
impl<'a, Client> OnlineMeetingsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action createOrGet",
        name: create_or_get,
        response: serde_json::Value,
        path: "/communications/onlineMeetings/microsoft.graph.createOrGet",
        params: 0,
        has_body: true
    });
}

#[allow(dead_code)]
impl<'a, Client> ParticipantsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action invite",
        name: invite,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/participants/microsoft.graph.invite",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action mute",
        name: mute,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/participants/{{id2}}/microsoft.graph.mute",
        params: 2,
        has_body: true
    });
}

#[allow(dead_code)]
impl<'a, Client> CommunicationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn calls(&self) -> CallsRequest<'a, Client> {
        CallsRequest::new(&self.client)
    }
    pub fn cloud_communications(&self) -> CloudCommunicationsRequest<'a, Client> {
        CloudCommunicationsRequest::new(&self.client)
    }
    pub fn online_meetings(&self) -> OnlineMeetingsRequest<'a, Client> {
        OnlineMeetingsRequest::new(&self.client)
    }
    get!({
        doc: "# Get calls from communications",
        name: list_calls,
        response: Collection<serde_json::Value>,
        path: "/communications/calls",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to calls for communications",
        name: create_calls,
        response: serde_json::Value,
        path: "/communications/calls",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get calls from communications",
        name: get_calls,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calls in communications",
        name: update_calls,
        response: GraphResponse<Content>,
        path: "/communications/calls/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property calls for communications",
        name: delete_calls,
        response: GraphResponse<Content>,
        path: "/communications/calls/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get onlineMeetings from communications",
        name: list_online_meetings,
        response: Collection<serde_json::Value>,
        path: "/communications/onlineMeetings",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to onlineMeetings for communications",
        name: create_online_meetings,
        response: serde_json::Value,
        path: "/communications/onlineMeetings",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get callRecords from communications",
        name: list_call_records,
        response: Collection<serde_json::Value>,
        path: "/communications/callRecords",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to callRecords for communications",
        name: create_call_records,
        response: serde_json::Value,
        path: "/communications/callRecords",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get callRecords from communications",
        name: get_call_records,
        response: serde_json::Value,
        path: "/communications/callRecords()",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property callRecords in communications",
        name: update_call_records,
        response: GraphResponse<Content>,
        path: "/communications/callRecords()",
        params: 0,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property callRecords for communications",
        name: delete_call_records,
        response: GraphResponse<Content>,
        path: "/communications/callRecords()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get onlineMeetings from communications",
        name: get_online_meetings,
        response: serde_json::Value,
        path: "/communications/onlineMeetings/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property onlineMeetings in communications",
        name: update_online_meetings,
        response: GraphResponse<Content>,
        path: "/communications/onlineMeetings/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property onlineMeetings for communications",
        name: delete_online_meetings,
        response: GraphResponse<Content>,
        path: "/communications/onlineMeetings/{{id}}",
        params: 1,
        has_body: false
    });
}

#[allow(dead_code)]
impl<'a, Client> CallsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn participants(&self) -> ParticipantsRequest<'a, Client> {
        ParticipantsRequest::new(&self.client)
    }
    post!({
        doc: "# Invoke action recordResponse",
        name: record_response,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/microsoft.graph.recordResponse",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action changeScreenSharingRole",
        name: change_screen_sharing_role,
        response: GraphResponse<Content>,
        path: "/communications/calls/{{id}}/microsoft.graph.changeScreenSharingRole",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action subscribeToTone",
        name: subscribe_to_tone,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/microsoft.graph.subscribeToTone",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get participants from communications",
        name: list_participants,
        response: Collection<serde_json::Value>,
        path: "/communications/calls/{{id}}/participants",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to participants for communications",
        name: create_participants,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/participants",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action transfer",
        name: transfer,
        response: GraphResponse<Content>,
        path: "/communications/calls/{{id}}/microsoft.graph.transfer",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get participants from communications",
        name: get_participants,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/participants/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property participants in communications",
        name: update_participants,
        response: GraphResponse<Content>,
        path: "/communications/calls/{{id}}/participants/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property participants for communications",
        name: delete_participants,
        response: GraphResponse<Content>,
        path: "/communications/calls/{{id}}/participants/{{id2}}",
        params: 2,
        has_body: false
    });
    post!({
        doc: "# Invoke action reject",
        name: reject,
        response: GraphResponse<Content>,
        path: "/communications/calls/{{id}}/microsoft.graph.reject",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action updateRecordingStatus",
        name: update_recording_status,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/microsoft.graph.updateRecordingStatus",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get operations from communications",
        name: get_operations,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/operations/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property operations in communications",
        name: update_operations,
        response: GraphResponse<Content>,
        path: "/communications/calls/{{id}}/operations/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property operations for communications",
        name: delete_operations,
        response: GraphResponse<Content>,
        path: "/communications/calls/{{id}}/operations/{{id2}}",
        params: 2,
        has_body: false
    });
    post!({
        doc: "# Invoke action unmute",
        name: unmute,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/microsoft.graph.unmute",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action answer",
        name: answer,
        response: GraphResponse<Content>,
        path: "/communications/calls/{{id}}/microsoft.graph.answer",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get operations from communications",
        name: list_operations,
        response: Collection<serde_json::Value>,
        path: "/communications/calls/{{id}}/operations",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to operations for communications",
        name: create_operations,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/operations",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action keepAlive",
        name: keep_alive,
        response: GraphResponse<Content>,
        path: "/communications/calls/{{id}}/microsoft.graph.keepAlive",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action playPrompt",
        name: play_prompt,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/microsoft.graph.playPrompt",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action mute",
        name: mute,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/microsoft.graph.mute",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action redirect",
        name: redirect,
        response: GraphResponse<Content>,
        path: "/communications/calls/{{id}}/microsoft.graph.redirect",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action logTeleconferenceDeviceQuality",
        name: log_teleconference_device_quality,
        response: GraphResponse<Content>,
        path: "/communications/calls/microsoft.graph.logTeleconferenceDeviceQuality",
        params: 0,
        has_body: true
    });
}

#[allow(dead_code)]
impl<'a, Client> CloudCommunicationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get communications",
        name: get_cloud_communications,
        response: serde_json::Value,
        path: "/communications",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update communications",
        name: update_cloud_communications,
        response: GraphResponse<Content>,
        path: "/communications",
        params: 0,
        has_body: true
    });
}
