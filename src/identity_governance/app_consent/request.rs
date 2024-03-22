// GENERATED CODE

use crate::api_default_imports::*;

api_client!(AppConsentApiClient, ResourceIdentity::AppConsent);

impl AppConsentApiClient {
    delete!(
        doc: "Delete navigation property appConsent for identityGovernance",
        name: delete_app_consent,
        path: "/appConsent"
    );
    get!(
        doc: "Get appConsent from identityGovernance",
        name: get_app_consent,
        path: "/appConsent"
    );
    patch!(
        doc: "Update the navigation property appConsent in identityGovernance",
        name: update_app_consent,
        path: "/appConsent",
        body: true
    );
    post!(
        doc: "Create new navigation property to appConsentRequests for identityGovernance",
        name: create_app_consent_requests,
        path: "/appConsent/appConsentRequests",
        body: true
    );
    get!(
        doc: "List appConsentRequests",
        name: list_app_consent_requests,
        path: "/appConsent/appConsentRequests"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_app_consent_requests_count,
        path: "/appConsent/appConsentRequests/$count"
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_app_consent_requests_by_current_user,
        path: "/appConsent/appConsentRequests/filterByCurrentUser(on='{{id}}')",
        params: on
    );
    delete!(
        doc: "Delete navigation property appConsentRequests for identityGovernance",
        name: delete_app_consent_requests,
        path: "/appConsent/appConsentRequests/{{id}}",
        params: app_consent_request_id
    );
    get!(
        doc: "Get appConsentRequests from identityGovernance",
        name: get_app_consent_requests,
        path: "/appConsent/appConsentRequests/{{id}}",
        params: app_consent_request_id
    );
    patch!(
        doc: "Update the navigation property appConsentRequests in identityGovernance",
        name: update_app_consent_requests,
        path: "/appConsent/appConsentRequests/{{id}}",
        body: true,
        params: app_consent_request_id
    );
    post!(
        doc: "Create new navigation property to userConsentRequests for identityGovernance",
        name: create_user_consent_requests,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests",
        body: true,
        params: app_consent_request_id
    );
    get!(
        doc: "List userConsentRequests",
        name: list_user_consent_requests,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests",
        params: app_consent_request_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_user_consent_requests_count,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/$count",
        params: app_consent_request_id
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_user_consent_requests_by_current_user,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/filterByCurrentUser(on='{{id2}}')",
        params: on
    );
    delete!(
        doc: "Delete navigation property userConsentRequests for identityGovernance",
        name: delete_user_consent_requests,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}",
        params: app_consent_request_id, user_consent_request_id
    );
    get!(
        doc: "Get userConsentRequests from identityGovernance",
        name: get_user_consent_requests,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}",
        params: app_consent_request_id, user_consent_request_id
    );
    patch!(
        doc: "Update the navigation property userConsentRequests in identityGovernance",
        name: update_user_consent_requests,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}",
        body: true,
        params: app_consent_request_id, user_consent_request_id
    );
    delete!(
        doc: "Delete navigation property approval for identityGovernance",
        name: delete_approval,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval",
        params: app_consent_request_id, user_consent_request_id
    );
    get!(
        doc: "Get approval from identityGovernance",
        name: get_approval,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval",
        params: app_consent_request_id, user_consent_request_id
    );
    patch!(
        doc: "Update the navigation property approval in identityGovernance",
        name: update_approval,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval",
        body: true,
        params: app_consent_request_id, user_consent_request_id
    );
    post!(
        doc: "Create new navigation property to stages for identityGovernance",
        name: create_stages,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages",
        body: true,
        params: app_consent_request_id, user_consent_request_id
    );
    get!(
        doc: "List approval stages",
        name: list_stages,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages",
        params: app_consent_request_id, user_consent_request_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_stages_count,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages/$count",
        params: app_consent_request_id, user_consent_request_id
    );
    delete!(
        doc: "Delete navigation property stages for identityGovernance",
        name: delete_stages,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages/{{id3}}",
        params: app_consent_request_id, user_consent_request_id, approval_stage_id
    );
    get!(
        doc: "Get stages from identityGovernance",
        name: get_stages,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages/{{id3}}",
        params: app_consent_request_id, user_consent_request_id, approval_stage_id
    );
    patch!(
        doc: "Update the navigation property stages in identityGovernance",
        name: update_stages,
        path: "/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages/{{id3}}",
        body: true,
        params: app_consent_request_id, user_consent_request_id, approval_stage_id
    );
}
