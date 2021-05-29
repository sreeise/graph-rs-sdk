// GENERATED CODE | DO NOT CHANGE

use crate::{client::Graph, core::ResourceIdentity};
use graph_http::{types::NoContent, IntoResponse};
use handlebars::*;
use reqwest::Method;

register_client!(CallRequest,);
register_client!(CallsRequest, ());
register_client!(ParticipantsRequest,);

impl<'a, Client> CallRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get calls from communications",
        name: list_calls,
        response: serde_json::Value,
        path: "/calls",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to calls for communications",
        name: create_calls,
        response: serde_json::Value,
        path: "/calls",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action logTeleconferenceDeviceQuality",
        name: log_teleconference_device_quality,
        response: NoContent,
        path: "/calls/logTeleconferenceDeviceQuality",
        params: 0,
        has_body: true
    });

    pub fn id<ID: AsRef<str>>(&self, id: ID) -> CallsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Calls);
        CallsRequest::new(id.as_ref(), self.client)
    }
}

impl<'a, Client> CallsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get calls from communications",
        name: get_calls,
        response: serde_json::Value,
        path: "/calls/{{RID}}",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property calls in communications",
        name: update_calls,
        response: NoContent,
        path: "/calls/{{RID}}",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action answer",
        name: answer,
        response: NoContent,
        path: "/calls/{{RID}}/answer",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action cancelMediaProcessing",
        name: cancel_media_processing,
        response: serde_json::Value,
        path: "/calls/{{RID}}/cancelMediaProcessing",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action changeScreenSharingRole",
        name: change_screen_sharing_role,
        response: NoContent,
        path: "/calls/{{RID}}/changeScreenSharingRole",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action keepAlive",
        name: keep_alive,
        response: NoContent,
        path: "/calls/{{RID}}/keepAlive",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Invoke action mute",
        name: mute,
        response: serde_json::Value,
        path: "/calls/{{RID}}/mute",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get operations from communications",
        name: list_operations,
        response: serde_json::Value,
        path: "/calls/{{RID}}/operations",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to operations for communications",
        name: create_operations,
        response: serde_json::Value,
        path: "/calls/{{RID}}/operations",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get operations from communications",
        name: get_operations,
        response: serde_json::Value,
        path: "/calls/{{RID}}/operations/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property operations in communications",
        name: update_operations,
        response: NoContent,
        path: "/calls/{{RID}}/operations/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get participants from communications",
        name: list_participants,
        response: serde_json::Value,
        path: "/calls/{{RID}}/participants",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to participants for communications",
        name: create_participants,
        response: serde_json::Value,
        path: "/calls/{{RID}}/participants",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get participants from communications",
        name: get_participants,
        response: serde_json::Value,
        path: "/calls/{{RID}}/participants/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property participants in communications",
        name: update_participants,
        response: NoContent,
        path: "/calls/{{RID}}/participants/{{id}}",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action playPrompt",
        name: play_prompt,
        response: serde_json::Value,
        path: "/calls/{{RID}}/playPrompt",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action recordResponse",
        name: record_response,
        response: serde_json::Value,
        path: "/calls/{{RID}}/recordResponse",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action redirect",
        name: redirect,
        response: NoContent,
        path: "/calls/{{RID}}/redirect",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action reject",
        name: reject,
        response: NoContent,
        path: "/calls/{{RID}}/reject",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action subscribeToTone",
        name: subscribe_to_tone,
        response: serde_json::Value,
        path: "/calls/{{RID}}/subscribeToTone",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action transfer",
        name: transfer,
        response: NoContent,
        path: "/calls/{{RID}}/transfer",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action unmute",
        name: unmute,
        response: serde_json::Value,
        path: "/calls/{{RID}}/unmute",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action updateRecordingStatus",
        name: update_recording_status,
        response: serde_json::Value,
        path: "/calls/{{RID}}/updateRecordingStatus",
        params: 0,
        has_body: true
    });

    pub fn participants(&self) -> ParticipantsRequest<'a, Client> {
        ParticipantsRequest::new(self.client)
    }
}

impl<'a, Client> ParticipantsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action invite",
        name: invite,
        response: serde_json::Value,
        path: "/calls/{{RID}}/participants/invite",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action mute",
        name: mute,
        response: serde_json::Value,
        path: "/calls/{{RID}}/participants/{{id}}/mute",
        params: 1,
        has_body: true
    });
}
