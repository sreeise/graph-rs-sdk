use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(CommunicationsRequest,);
register_client!(CallRecordsRequest,);
register_client!(CallsRequest,);
register_client!(CloudCommunicationsRequest,);
register_client!(OnlineMeetingsRequest,);
register_client!(ParticipantsRequest,);
register_client!(SessionsRequest,);

impl<'a, Client> CommunicationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn call_records(&self) -> CallRecordsRequest<'a, Client> {
        CallRecordsRequest::new(&self.client)
    }
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
    get!({
        doc: "# Get callRecords from communications",
        name: get_call_records,
        response: serde_json::Value,
        path: "/communications/callRecords/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property callRecords in communications",
        name: update_call_records,
        response: GraphResponse<Content>,
        path: "/communications/callRecords/{{id}}",
        params: 1,
        has_body: true
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
}

impl<'a, Client> CallRecordsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn sessions(&self) -> SessionsRequest<'a, Client> {
        SessionsRequest::new(&self.client)
    }
    get!({
        doc: "# Get sessions from communications",
        name: list_sessions,
        response: Collection<serde_json::Value>,
        path: "/communications/callRecords/{{id}}/sessions",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to sessions for communications",
        name: create_sessions,
        response: serde_json::Value,
        path: "/communications/callRecords/{{id}}/sessions",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get sessions from communications",
        name: get_sessions,
        response: serde_json::Value,
        path: "/communications/callRecords/{{id}}/sessions/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property sessions in communications",
        name: update_sessions,
        response: GraphResponse<Content>,
        path: "/communications/callRecords/{{id}}/sessions/{{id2}}",
        params: 2,
        has_body: true
    });
}

impl<'a, Client> SessionsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get segments from communications",
        name: list_segments,
        response: Collection<serde_json::Value>,
        path: "/communications/callRecords/{{id}}/sessions/{{id2}}/segments",
        params: 2,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to segments for communications",
        name: create_segments,
        response: serde_json::Value,
        path: "/communications/callRecords/{{id}}/sessions/{{id2}}/segments",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get segments from communications",
        name: get_segments,
        response: serde_json::Value,
        path: "/communications/callRecords/{{id}}/sessions/{{id2}}/segments/{{id3}}",
        params: 3,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property segments in communications",
        name: update_segments,
        response: GraphResponse<Content>,
        path: "/communications/callRecords/{{id}}/sessions/{{id2}}/segments/{{id3}}",
        params: 3,
        has_body: true
    });
}

impl<'a, Client> CallsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn participants(&self) -> ParticipantsRequest<'a, Client> {
        ParticipantsRequest::new(&self.client)
    }
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
        doc: "# Invoke action playPrompt",
        name: play_prompt,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/playPrompt",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action cancelMediaProcessing",
        name: cancel_media_processing,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/cancelMediaProcessing",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action changeScreenSharingRole",
        name: change_screen_sharing_role,
        response: GraphResponse<Content>,
        path: "/communications/calls/{{id}}/changeScreenSharingRole",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action reject",
        name: reject,
        response: GraphResponse<Content>,
        path: "/communications/calls/{{id}}/reject",
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
    post!({
        doc: "# Invoke action subscribeToTone",
        name: subscribe_to_tone,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/subscribeToTone",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action mute",
        name: mute,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/mute",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action updateRecordingStatus",
        name: update_recording_status,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/updateRecordingStatus",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action recordResponse",
        name: record_response,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/recordResponse",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action keepAlive",
        name: keep_alive,
        response: GraphResponse<Content>,
        path: "/communications/calls/{{id}}/keepAlive",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action redirect",
        name: redirect,
        response: GraphResponse<Content>,
        path: "/communications/calls/{{id}}/redirect",
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
    post!({
        doc: "# Invoke action transfer",
        name: transfer,
        response: GraphResponse<Content>,
        path: "/communications/calls/{{id}}/transfer",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action unmute",
        name: unmute,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/unmute",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action answer",
        name: answer,
        response: GraphResponse<Content>,
        path: "/communications/calls/{{id}}/answer",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action logTeleconferenceDeviceQuality",
        name: log_teleconference_device_quality,
        response: GraphResponse<Content>,
        path: "/communications/calls/logTeleconferenceDeviceQuality",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> ParticipantsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action mute",
        name: mute,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/participants/{{id2}}/mute",
        params: 2,
        has_body: true
    });
    post!({
        doc: "# Invoke action invite",
        name: invite,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}/participants/invite",
        params: 1,
        has_body: true
    });
}

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

impl<'a, Client> OnlineMeetingsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action createOrGet",
        name: create_or_get,
        response: serde_json::Value,
        path: "/communications/onlineMeetings/createOrGet",
        params: 0,
        has_body: true
    });
}
