// GENERATED CODE

use crate::api_default_imports::*;
use crate::identity_governance::{
    AccessPackagesApiClient, AccessPackagesIdApiClient, AccessReviewsApiClient,
    EntitlementManagementApiClient,
};

resource_api_client!(
    IdentityGovernanceApiClient,
    ResourceIdentity::IdentityGovernance
);

impl IdentityGovernanceApiClient {
    api_client_link!(
        access_reviews,
        ResourceIdentity::AccessReviews,
        AccessReviewsApiClient
    );
    api_client_link!(
        entitlement_management,
        ResourceIdentity::EntitlementManagement,
        EntitlementManagementApiClient
    );

    get!(
        doc: "Get identityGovernance",
        name: get_identity_governance,
        path: "/identityGovernance"
    );
    patch!(
        doc: "Update identityGovernance",
        name: update_identity_governance,
        path: "/identityGovernance",
        body: true
    );
    delete!(
        doc: "Delete navigation property appConsent for identityGovernance",
        name: delete_app_consent,
        path: "/identityGovernance/appConsent"
    );
    get!(
        doc: "Get appConsent from identityGovernance",
        name: get_app_consent,
        path: "/identityGovernance/appConsent"
    );
    patch!(
        doc: "Update the navigation property appConsent in identityGovernance",
        name: update_app_consent,
        path: "/identityGovernance/appConsent",
        body: true
    );
    post!(
        doc: "Create new navigation property to appConsentRequests for identityGovernance",
        name: create_app_consent_requests,
        path: "/identityGovernance/appConsent/appConsentRequests",
        body: true
    );
    get!(
        doc: "List appConsentRequests",
        name: list_app_consent_requests,
        path: "/identityGovernance/appConsent/appConsentRequests"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_app_consent_requests_count,
        path: "/identityGovernance/appConsent/appConsentRequests/$count"
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_app_consent_requests_by_current_user,
        path: "/identityGovernance/appConsent/appConsentRequests/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: on
    );
    delete!(
        doc: "Delete navigation property appConsentRequests for identityGovernance",
        name: delete_app_consent_requests,
        path: "/identityGovernance/appConsent/appConsentRequests/{{id}}",
        params: app_consent_request_id
    );
    get!(
        doc: "Get appConsentRequests from identityGovernance",
        name: get_app_consent_requests,
        path: "/identityGovernance/appConsent/appConsentRequests/{{id}}",
        params: app_consent_request_id
    );
    patch!(
        doc: "Update the navigation property appConsentRequests in identityGovernance",
        name: update_app_consent_requests,
        path: "/identityGovernance/appConsent/appConsentRequests/{{id}}",
        body: true,
        params: app_consent_request_id
    );
    post!(
        doc: "Create new navigation property to userConsentRequests for identityGovernance",
        name: create_user_consent_requests,
        path: "/identityGovernance/appConsent/appConsentRequests/{{id}}/userConsentRequests",
        body: true,
        params: app_consent_request_id
    );
    get!(
        doc: "List userConsentRequests",
        name: list_user_consent_requests,
        path: "/identityGovernance/appConsent/appConsentRequests/{{id}}/userConsentRequests",
        params: app_consent_request_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_user_consent_requests_count,
        path: "/identityGovernance/appConsent/appConsentRequests/{{id}}/userConsentRequests/$count",
        params: app_consent_request_id
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_user_consent_requests_by_current_user,
        path: "/identityGovernance/appConsent/appConsentRequests/{{id}}/userConsentRequests/microsoft.graph.filterByCurrentUser(on='{{id2}}')",
        params: on
    );
    delete!(
        doc: "Delete navigation property userConsentRequests for identityGovernance",
        name: delete_user_consent_requests,
        path: "/identityGovernance/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}",
        params: app_consent_request_id, user_consent_request_id
    );
    get!(
        doc: "Get userConsentRequests from identityGovernance",
        name: get_user_consent_requests,
        path: "/identityGovernance/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}",
        params: app_consent_request_id, user_consent_request_id
    );
    patch!(
        doc: "Update the navigation property userConsentRequests in identityGovernance",
        name: update_user_consent_requests,
        path: "/identityGovernance/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}",
        body: true,
        params: app_consent_request_id, user_consent_request_id
    );
    delete!(
        doc: "Delete navigation property approval for identityGovernance",
        name: delete_approval,
        path: "/identityGovernance/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval",
        params: app_consent_request_id, user_consent_request_id
    );
    get!(
        doc: "Get approval from identityGovernance",
        name: get_approval,
        path: "/identityGovernance/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval",
        params: app_consent_request_id, user_consent_request_id
    );
    patch!(
        doc: "Update the navigation property approval in identityGovernance",
        name: update_approval,
        path: "/identityGovernance/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval",
        body: true,
        params: app_consent_request_id, user_consent_request_id
    );
    post!(
        doc: "Create new navigation property to stages for identityGovernance",
        name: create_stages,
        path: "/identityGovernance/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages",
        body: true,
        params: app_consent_request_id, user_consent_request_id
    );
    get!(
        doc: "List approval stages",
        name: list_stages,
        path: "/identityGovernance/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages",
        params: app_consent_request_id, user_consent_request_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_stages_count,
        path: "/identityGovernance/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages/$count",
        params: app_consent_request_id, user_consent_request_id
    );
    delete!(
        doc: "Delete navigation property stages for identityGovernance",
        name: delete_stages,
        path: "/identityGovernance/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages/{{id3}}",
        params: app_consent_request_id, user_consent_request_id, approval_stage_id
    );
    get!(
        doc: "Get stages from identityGovernance",
        name: get_stages,
        path: "/identityGovernance/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages/{{id3}}",
        params: app_consent_request_id, user_consent_request_id, approval_stage_id
    );
    patch!(
        doc: "Update the navigation property stages in identityGovernance",
        name: update_stages,
        path: "/identityGovernance/appConsent/appConsentRequests/{{id}}/userConsentRequests/{{id2}}/approval/stages/{{id3}}",
        body: true,
        params: app_consent_request_id, user_consent_request_id, approval_stage_id
    );
    delete!(
        doc: "Delete navigation property termsOfUse for identityGovernance",
        name: delete_terms_of_use,
        path: "/identityGovernance/termsOfUse"
    );
    get!(
        doc: "Get termsOfUse from identityGovernance",
        name: get_terms_of_use,
        path: "/identityGovernance/termsOfUse"
    );
    patch!(
        doc: "Update the navigation property termsOfUse in identityGovernance",
        name: update_terms_of_use,
        path: "/identityGovernance/termsOfUse",
        body: true
    );
    post!(
        doc: "Create new navigation property to agreementAcceptances for identityGovernance",
        name: create_agreement_acceptances,
        path: "/identityGovernance/termsOfUse/agreementAcceptances",
        body: true
    );
    get!(
        doc: "Get agreementAcceptances from identityGovernance",
        name: list_agreement_acceptances,
        path: "/identityGovernance/termsOfUse/agreementAcceptances"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_agreement_acceptances_count,
        path: "/identityGovernance/termsOfUse/agreementAcceptances/$count"
    );
    delete!(
        doc: "Delete navigation property agreementAcceptances for identityGovernance",
        name: delete_agreement_acceptances,
        path: "/identityGovernance/termsOfUse/agreementAcceptances/{{id}}",
        params: agreement_acceptance_id
    );
    get!(
        doc: "Get agreementAcceptances from identityGovernance",
        name: get_agreement_acceptances,
        path: "/identityGovernance/termsOfUse/agreementAcceptances/{{id}}",
        params: agreement_acceptance_id
    );
    patch!(
        doc: "Update the navigation property agreementAcceptances in identityGovernance",
        name: update_agreement_acceptances,
        path: "/identityGovernance/termsOfUse/agreementAcceptances/{{id}}",
        body: true,
        params: agreement_acceptance_id
    );
    post!(
        doc: "Create agreement",
        name: create_agreements,
        path: "/identityGovernance/termsOfUse/agreements",
        body: true
    );
    get!(
        doc: "List agreements",
        name: list_agreements,
        path: "/identityGovernance/termsOfUse/agreements"
    );
    get!(
        doc: "Get the number of the resource",
        name: count,
        path: "/identityGovernance/termsOfUse/agreements/$count"
    );
    delete!(
        doc: "Delete navigation property agreements for identityGovernance",
        name: delete_agreements,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}",
        params: agreement_id
    );
    get!(
        doc: "Get agreements from identityGovernance",
        name: get_agreements,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}",
        params: agreement_id
    );
    patch!(
        doc: "Update the navigation property agreements in identityGovernance",
        name: update_agreements,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}",
        body: true,
        params: agreement_id
    );
    post!(
        doc: "Create new navigation property to acceptances for identityGovernance",
        name: create_acceptances,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/acceptances",
        body: true,
        params: agreement_id
    );
    get!(
        doc: "List acceptances",
        name: list_acceptances,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/acceptances",
        params: agreement_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_acceptances_count,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/acceptances/$count",
        params: agreement_id
    );
    delete!(
        doc: "Delete navigation property acceptances for identityGovernance",
        name: delete_acceptances,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/acceptances/{{id2}}",
        params: agreement_id, agreement_acceptance_id
    );
    get!(
        doc: "Get acceptances from identityGovernance",
        name: get_acceptances,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/acceptances/{{id2}}",
        params: agreement_id, agreement_acceptance_id
    );
    patch!(
        doc: "Update the navigation property acceptances in identityGovernance",
        name: update_acceptances,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/acceptances/{{id2}}",
        body: true,
        params: agreement_id, agreement_acceptance_id
    );
    delete!(
        doc: "Delete navigation property file for identityGovernance",
        name: delete_file,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/file",
        params: agreement_id
    );
    get!(
        doc: "Get agreementFile",
        name: get_file,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/file",
        params: agreement_id
    );
    patch!(
        doc: "Update the navigation property file in identityGovernance",
        name: update_file,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/file",
        body: true,
        params: agreement_id
    );
    post!(
        doc: "Create new navigation property to localizations for identityGovernance",
        name: create_localizations,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations",
        body: true,
        params: agreement_id
    );
    get!(
        doc: "Get localizations from identityGovernance",
        name: list_localizations,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations",
        params: agreement_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_localizations_count,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/$count",
        params: agreement_id
    );
    delete!(
        doc: "Delete navigation property localizations for identityGovernance",
        name: delete_localizations,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}",
        params: agreement_id, agreement_file_localization_id
    );
    get!(
        doc: "Get localizations from identityGovernance",
        name: get_localizations,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}",
        params: agreement_id, agreement_file_localization_id
    );
    patch!(
        doc: "Update the navigation property localizations in identityGovernance",
        name: update_localizations,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}",
        body: true,
        params: agreement_id, agreement_file_localization_id
    );
    post!(
        doc: "Create new navigation property to versions for identityGovernance",
        name: create_file_localizations_versions,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}/versions",
        body: true,
        params: agreement_id, agreement_file_localization_id
    );
    get!(
        doc: "Get versions from identityGovernance",
        name: list_file_localizations_versions,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}/versions",
        params: agreement_id, agreement_file_localization_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_file_localizations_versions_count,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}/versions/$count",
        params: agreement_id, agreement_file_localization_id
    );
    delete!(
        doc: "Delete navigation property versions for identityGovernance",
        name: delete_file_localizations_versions,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}/versions/{{id3}}",
        params: agreement_id, agreement_file_localization_id, agreement_file_version_id
    );
    get!(
        doc: "Get versions from identityGovernance",
        name: get_file_localizations_versions,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}/versions/{{id3}}",
        params: agreement_id, agreement_file_localization_id, agreement_file_version_id
    );
    patch!(
        doc: "Update the navigation property versions in identityGovernance",
        name: update_file_localizations_versions,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/file/localizations/{{id2}}/versions/{{id3}}",
        body: true,
        params: agreement_id, agreement_file_localization_id, agreement_file_version_id
    );
    post!(
        doc: "Create agreementFileLocalization",
        name: create_files,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/files",
        body: true,
        params: agreement_id
    );
    get!(
        doc: "Get files from identityGovernance",
        name: list_files,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/files",
        params: agreement_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_files_count,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/files/$count",
        params: agreement_id
    );
    delete!(
        doc: "Delete navigation property files for identityGovernance",
        name: delete_files,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/files/{{id2}}",
        params: agreement_id, agreement_file_localization_id
    );
    get!(
        doc: "Get files from identityGovernance",
        name: get_files,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/files/{{id2}}",
        params: agreement_id, agreement_file_localization_id
    );
    patch!(
        doc: "Update the navigation property files in identityGovernance",
        name: update_files,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/files/{{id2}}",
        body: true,
        params: agreement_id, agreement_file_localization_id
    );
    post!(
        doc: "Create new navigation property to versions for identityGovernance",
        name: create_versions,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/files/{{id2}}/versions",
        body: true,
        params: agreement_id, agreement_file_localization_id
    );
    get!(
        doc: "Get versions from identityGovernance",
        name: list_versions,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/files/{{id2}}/versions",
        params: agreement_id, agreement_file_localization_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_versions_count,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/files/{{id2}}/versions/$count",
        params: agreement_id, agreement_file_localization_id
    );
    delete!(
        doc: "Delete navigation property versions for identityGovernance",
        name: delete_versions,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/files/{{id2}}/versions/{{id3}}",
        params: agreement_id, agreement_file_localization_id, agreement_file_version_id
    );
    get!(
        doc: "Get versions from identityGovernance",
        name: get_versions,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/files/{{id2}}/versions/{{id3}}",
        params: agreement_id, agreement_file_localization_id, agreement_file_version_id
    );
    patch!(
        doc: "Update the navigation property versions in identityGovernance",
        name: update_versions,
        path: "/identityGovernance/termsOfUse/agreements/{{id}}/files/{{id2}}/versions/{{id3}}",
        body: true,
        params: agreement_id, agreement_file_localization_id, agreement_file_version_id
    );
}
