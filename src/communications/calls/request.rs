// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(CallsApiClient, CallsIdApiClient, ResourceIdentity::Calls);

impl CallsApiClient {
    post!(
        doc: "Create call",
        name: create_calls,
        path: "/calls",
        body: true
    );
    get!(
        doc: "Get calls from communications",
        name: list_calls,
        path: "/calls"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_calls_count,
        path: "/calls/$count"
    );
    post!(
        doc: "Invoke action logTeleconferenceDeviceQuality",
        name: log_teleconference_device_quality,
        path: "/calls/logTeleconferenceDeviceQuality",
        body: true
    );
}

impl CallsIdApiClient {
    delete!(
        doc: "Delete navigation property calls for communications",
        name: delete_calls,
        path: "/calls/{{RID}}"
    );
    get!(
        doc: "Get calls from communications",
        name: get_calls,
        path: "/calls/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property calls in communications",
        name: update_calls,
        path: "/calls/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action addLargeGalleryView",
        name: add_large_gallery_view,
        path: "/calls/{{RID}}/addLargeGalleryView",
        body: true
    );
    post!(
        doc: "Invoke action answer",
        name: answer,
        path: "/calls/{{RID}}/answer",
        body: true
    );
    post!(
        doc: "Create new navigation property to audioRoutingGroups for communications",
        name: create_audio_routing_groups,
        path: "/calls/{{RID}}/audioRoutingGroups",
        body: true
    );
    get!(
        doc: "Get audioRoutingGroups from communications",
        name: list_audio_routing_groups,
        path: "/calls/{{RID}}/audioRoutingGroups"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_audio_routing_groups_count,
        path: "/calls/{{RID}}/audioRoutingGroups/$count"
    );
    delete!(
        doc: "Delete navigation property audioRoutingGroups for communications",
        name: delete_audio_routing_groups,
        path: "/calls/{{RID}}/audioRoutingGroups/{{id}}",
        params: audio_routing_group_id
    );
    get!(
        doc: "Get audioRoutingGroups from communications",
        name: get_audio_routing_groups,
        path: "/calls/{{RID}}/audioRoutingGroups/{{id}}",
        params: audio_routing_group_id
    );
    patch!(
        doc: "Update the navigation property audioRoutingGroups in communications",
        name: update_audio_routing_groups,
        path: "/calls/{{RID}}/audioRoutingGroups/{{id}}",
        body: true,
        params: audio_routing_group_id
    );
    post!(
        doc: "Invoke action cancelMediaProcessing",
        name: cancel_media_processing,
        path: "/calls/{{RID}}/cancelMediaProcessing",
        body: true
    );
    post!(
        doc: "Invoke action changeScreenSharingRole",
        name: change_screen_sharing_role,
        path: "/calls/{{RID}}/changeScreenSharingRole",
        body: true
    );
    post!(
        doc: "Create new navigation property to contentSharingSessions for communications",
        name: create_content_sharing_sessions,
        path: "/calls/{{RID}}/contentSharingSessions",
        body: true
    );
    get!(
        doc: "List contentSharingSessions",
        name: list_content_sharing_sessions,
        path: "/calls/{{RID}}/contentSharingSessions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_content_sharing_sessions_count,
        path: "/calls/{{RID}}/contentSharingSessions/$count"
    );
    delete!(
        doc: "Delete navigation property contentSharingSessions for communications",
        name: delete_content_sharing_sessions,
        path: "/calls/{{RID}}/contentSharingSessions/{{id}}",
        params: content_sharing_session_id
    );
    get!(
        doc: "Get contentSharingSessions from communications",
        name: get_content_sharing_sessions,
        path: "/calls/{{RID}}/contentSharingSessions/{{id}}",
        params: content_sharing_session_id
    );
    patch!(
        doc: "Update the navigation property contentSharingSessions in communications",
        name: update_content_sharing_sessions,
        path: "/calls/{{RID}}/contentSharingSessions/{{id}}",
        body: true,
        params: content_sharing_session_id
    );
    post!(
        doc: "Invoke action keepAlive",
        name: keep_alive,
        path: "/calls/{{RID}}/keepAlive"
    );
    post!(
        doc: "Invoke action mute",
        name: mute,
        path: "/calls/{{RID}}/mute",
        body: true
    );
    post!(
        doc: "Create new navigation property to operations for communications",
        name: create_operations,
        path: "/calls/{{RID}}/operations",
        body: true
    );
    get!(
        doc: "Get operations from communications",
        name: list_operations,
        path: "/calls/{{RID}}/operations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_operations_count,
        path: "/calls/{{RID}}/operations/$count"
    );
    delete!(
        doc: "Delete navigation property operations for communications",
        name: delete_operations,
        path: "/calls/{{RID}}/operations/{{id}}",
        params: comms_operation_id
    );
    get!(
        doc: "Get operations from communications",
        name: get_operations,
        path: "/calls/{{RID}}/operations/{{id}}",
        params: comms_operation_id
    );
    patch!(
        doc: "Update the navigation property operations in communications",
        name: update_operations,
        path: "/calls/{{RID}}/operations/{{id}}",
        body: true,
        params: comms_operation_id
    );
    post!(
        doc: "Create new navigation property to participants for communications",
        name: create_participants,
        path: "/calls/{{RID}}/participants",
        body: true
    );
    get!(
        doc: "List participants",
        name: list_participants,
        path: "/calls/{{RID}}/participants"
    );
    get!(
        doc: "Get the number of the resource",
        name: count,
        path: "/calls/{{RID}}/participants/$count"
    );
    post!(
        doc: "Invoke action invite",
        name: invite,
        path: "/calls/{{RID}}/participants/invite",
        body: true
    );
    delete!(
        doc: "Delete navigation property participants for communications",
        name: delete_participants,
        path: "/calls/{{RID}}/participants/{{id}}",
        params: participant_id
    );
    get!(
        doc: "Get participants from communications",
        name: get_participants,
        path: "/calls/{{RID}}/participants/{{id}}",
        params: participant_id
    );
    patch!(
        doc: "Update the navigation property participants in communications",
        name: update_participants,
        path: "/calls/{{RID}}/participants/{{id}}",
        body: true,
        params: participant_id
    );
    post!(
        doc: "Invoke action mute",
        name: mute_participants,
        path: "/calls/{{RID}}/participants/{{id}}/mute",
        body: true,
        params: participant_id
    );
    post!(
        doc: "Invoke action startHoldMusic",
        name: start_hold_music,
        path: "/calls/{{RID}}/participants/{{id}}/startHoldMusic",
        body: true,
        params: participant_id
    );
    post!(
        doc: "Invoke action stopHoldMusic",
        name: stop_hold_music,
        path: "/calls/{{RID}}/participants/{{id}}/stopHoldMusic",
        body: true,
        params: participant_id
    );
    post!(
        doc: "Invoke action playPrompt",
        name: play_prompt,
        path: "/calls/{{RID}}/playPrompt",
        body: true
    );
    post!(
        doc: "Invoke action recordResponse",
        name: record_response,
        path: "/calls/{{RID}}/recordResponse",
        body: true
    );
    post!(
        doc: "Invoke action redirect",
        name: redirect,
        path: "/calls/{{RID}}/redirect",
        body: true
    );
    post!(
        doc: "Invoke action reject",
        name: reject,
        path: "/calls/{{RID}}/reject",
        body: true
    );
    post!(
        doc: "Invoke action subscribeToTone",
        name: subscribe_to_tone,
        path: "/calls/{{RID}}/subscribeToTone",
        body: true
    );
    post!(
        doc: "Invoke action transfer",
        name: transfer,
        path: "/calls/{{RID}}/transfer",
        body: true
    );
    post!(
        doc: "Invoke action unmute",
        name: unmute,
        path: "/calls/{{RID}}/unmute",
        body: true
    );
    post!(
        doc: "Invoke action updateRecordingStatus",
        name: update_recording_status,
        path: "/calls/{{RID}}/updateRecordingStatus",
        body: true
    );
}
