// GENERATED CODE

use crate::api_default_imports::*;
use crate::communications::{
    call_records::CallRecordsApiClient, call_records::CallRecordsIdApiClient,
    calls::CallsApiClient, calls::CallsIdApiClient,
};

api_client!(CommunicationsApiClient, ResourceIdentity::Communications);

impl CommunicationsApiClient {
    api_client_link_id!(
        call_record,
        ResourceIdentity::CallRecords,
        CallRecordsIdApiClient
    );
    api_client_link_id!(call, ResourceIdentity::Calls, CallsIdApiClient);
    api_client_link!(
        call_records,
        ResourceIdentity::CallRecords,
        CallRecordsApiClient
    );
    api_client_link!(calls, ResourceIdentity::Calls, CallsApiClient);

    get!(
        doc: "Get communications",
        name: get_cloud_communications,
        path: "/communications"
    );
    patch!(
        doc: "Update communications",
        name: update_cloud_communications,
        path: "/communications",
        body: true
    );
    post!(
        doc: "Invoke action getPresencesByUserId",
        name: get_presences_by_user_id,
        path: "/communications/getPresencesByUserId",
        body: true
    );
}
