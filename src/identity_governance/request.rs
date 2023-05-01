// GENERATED CODE

use crate::api_default_imports::*;
use crate::identity_governance::{
    AccessReviewsApiClient, AppConsentApiClient, EntitlementManagementApiClient,
};

api_client!(
    IdentityGovernanceApiClient,
    ResourceIdentity::IdentityGovernance
);

impl IdentityGovernanceApiClient {
    api_client_link!(
        access_reviews,
        ResourceIdentity::AccessReviews,
        AccessReviewsApiClient
    );
    api_client_link!(app_consent, AppConsentApiClient);
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
