// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(AppConsentRequest,);

impl<'a, Client> AppConsentRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    delete!({
        doc: "Delete navigation property appConsent for identityGovernance",
        name: delete_app_consent,
        response: NoContent,
        path: "/appConsent",
        has_body: false
    });
    get!({
        doc: "Get appConsent from identityGovernance",
        name: get_app_consent,
        response: serde_json::Value,
        path: "/appConsent",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property appConsent in identityGovernance",
        name: update_app_consent,
        response: NoContent,
        path: "/appConsent",
        has_body: true
    });
    post!({
        doc: "Create new navigation property to appConsentRequests for identityGovernance",
        name: create_app_consent_requests,
        response: serde_json::Value,
        path: "/appConsent/appConsentRequests",
        has_body: true
    });
    get!({
        doc: "Get appConsentRequests from identityGovernance",
        name: list_app_consent_requests,
        response: serde_json::Value,
        path: "/appConsent/appConsentRequests",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_app_consent_requests_count,
        response: serde_json::Value,
        path: "/appConsent/appConsentRequests/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function filterByCurrentUser",
        name: filter_app_consent_requests_by_current_user,
        response: serde_json::Value,
        path: "/appConsent/appConsentRequests/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: [ on ],
        has_body: false
    });
    delete!({
        doc: "Delete navigation property appConsentRequests for identityGovernance",
        name: delete_app_consent_requests,
        response: NoContent,
        path: "/appConsent/appConsentRequests/{{id}}",
        params: [ app_consent_request_id ],
        has_body: false
    });
    get!({
        doc: "Get appConsentRequests from identityGovernance",
        name: get_app_consent_requests,
        response: serde_json::Value,
        path: "/appConsent/appConsentRequests/{{id}}",
        params: [ app_consent_request_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property appConsentRequests in identityGovernance",
        name: update_app_consent_requests,
        response: NoContent,
        path: "/appConsent/appConsentRequests/{{id}}",
        params: [ app_consent_request_id ],
        has_body: true
    });
    post!({
        doc: "Create new navigation property to userConsentRequests for identityGovernance",
        name: create_user_consent_requests,
        response: serde_json::Value,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests",
        params: [ app_consent_request_id ],
        has_body: true
    });
    get!({
        doc: "Get userConsentRequests from identityGovernance",
        name: list_user_consent_requests,
        response: serde_json::Value,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests",
        params: [ app_consent_request_id ],
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_user_consent_requests_count,
        response: serde_json::Value,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/$count",
        params: [ app_consent_request_id ],
        has_body: false
    });
    get!({
        doc: "Invoke function filterByCurrentUser",
        name: filter_user_consent_requests_by_current_user,
        response: serde_json::Value,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/microsoft.graph.filterByCurrentUser(on='{{id2}}')",
        params: [ on ],
        has_body: false
    });
    delete!({
        doc: "Delete navigation property userConsentRequests for identityGovernance",
        name: delete_user_consent_requests,
        response: NoContent,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}",
        params: [ app_consent_request_id  user_consent_request_id ],
        has_body: false
    });
    get!({
        doc: "Get userConsentRequests from identityGovernance",
        name: get_user_consent_requests,
        response: serde_json::Value,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}",
        params: [ app_consent_request_id  user_consent_request_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property userConsentRequests in identityGovernance",
        name: update_user_consent_requests,
        response: NoContent,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}",
        params: [ app_consent_request_id  user_consent_request_id ],
        has_body: true
    });
    delete!({
        doc: "Delete navigation property approval for identityGovernance",
        name: delete_approval,
        response: NoContent,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval",
        params: [ app_consent_request_id  user_consent_request_id ],
        has_body: false
    });
    get!({
        doc: "Get approval from identityGovernance",
        name: get_approval,
        response: serde_json::Value,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval",
        params: [ app_consent_request_id  user_consent_request_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property approval in identityGovernance",
        name: update_approval,
        response: NoContent,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval",
        params: [ app_consent_request_id  user_consent_request_id ],
        has_body: true
    });
    post!({
        doc: "Create new navigation property to stages for identityGovernance",
        name: create_stages,
        response: serde_json::Value,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages",
        params: [ app_consent_request_id  user_consent_request_id ],
        has_body: true
    });
    get!({
        doc: "Get stages from identityGovernance",
        name: list_stages,
        response: serde_json::Value,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages",
        params: [ app_consent_request_id  user_consent_request_id ],
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_user_consent_requests_approval_stages_count,
        response: serde_json::Value,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages/$count",
        params: [ app_consent_request_id  user_consent_request_id ],
        has_body: false
    });
    delete!({
        doc: "Delete navigation property stages for identityGovernance",
        name: delete_stages,
        response: NoContent,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages/{{id3}}",
        params: [ app_consent_request_id  user_consent_request_id  approval_stage_id ],
        has_body: false
    });
    get!({
        doc: "Get stages from identityGovernance",
        name: get_stages,
        response: serde_json::Value,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages/{{id3}}",
        params: [ app_consent_request_id  user_consent_request_id  approval_stage_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property stages in identityGovernance",
        name: update_stages,
        response: NoContent,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages/{{id3}}",
        params: [ app_consent_request_id  user_consent_request_id  approval_stage_id ],
        has_body: true
    });
}
