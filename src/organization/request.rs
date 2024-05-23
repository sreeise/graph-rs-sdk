// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    OrganizationApiClient,
    OrganizationIdApiClient,
    ResourceIdentity::Organization
);

impl OrganizationApiClient {
    post!(
        doc: "Add new entity to organization",
        name: create_organization,
        path: "/organization",
        body: true
    );
    get!(
        doc: "List organization",
        name: list_organization,
        path: "/organization"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_organization_count,
        path: "/organization/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/organization/delta()"
    );
    post!(
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        path: "/organization/getAvailableExtensionProperties",
        body: true
    );
    post!(
        doc: "Invoke action getByIds",
        name: get_by_ids,
        path: "/organization/getByIds",
        body: true
    );
    post!(
        doc: "Invoke action validateProperties",
        name: validate_properties,
        path: "/organization/validateProperties",
        body: true
    );
}

impl OrganizationIdApiClient {
    delete!(
        doc: "Delete entity from organization",
        name: delete_organization,
        path: "/organization/{{RID}}"
    );
    get!(
        doc: "Get organization",
        name: get_organization,
        path: "/organization/{{RID}}"
    );
    patch!(
        doc: "Update organization",
        name: update_organization,
        path: "/organization/{{RID}}",
        body: true
    );
    delete!(
        doc: "Delete organizationalBranding",
        name: delete_branding,
        path: "/organization/{{RID}}/branding"
    );
    get!(
        doc: "Get organizationalBranding",
        name: get_branding,
        path: "/organization/{{RID}}/branding"
    );
    patch!(
        doc: "Update organizationalBranding",
        name: update_branding,
        path: "/organization/{{RID}}/branding",
        body: true
    );
    get!(
        doc: "Get backgroundImage for the navigation property branding from organization",
        name: get_branding_background_image,
        path: "/organization/{{RID}}/branding/backgroundImage"
    );
    put!(
        doc: "Update backgroundImage for the navigation property branding in organization",
        name: update_branding_background_image,
        path: "/organization/{{RID}}/branding/backgroundImage",
        body: true
    );
    get!(
        doc: "Get bannerLogo for the navigation property branding from organization",
        name: get_branding_banner_logo,
        path: "/organization/{{RID}}/branding/bannerLogo"
    );
    put!(
        doc: "Update bannerLogo for the navigation property branding in organization",
        name: update_branding_banner_logo,
        path: "/organization/{{RID}}/branding/bannerLogo",
        body: true
    );
    post!(
        doc: "Create organizationalBrandingLocalization",
        name: create_localizations,
        path: "/organization/{{RID}}/branding/localizations",
        body: true
    );
    get!(
        doc: "List localizations",
        name: list_localizations,
        path: "/organization/{{RID}}/branding/localizations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_localizations_count,
        path: "/organization/{{RID}}/branding/localizations/$count"
    );
    delete!(
        doc: "Delete navigation property localizations for organization",
        name: delete_localizations,
        path: "/organization/{{RID}}/branding/localizations/{{id}}",
        params: organizational_branding_localization_id
    );
    get!(
        doc: "Get localizations from organization",
        name: get_localizations,
        path: "/organization/{{RID}}/branding/localizations/{{id}}",
        params: organizational_branding_localization_id
    );
    patch!(
        doc: "Update the navigation property localizations in organization",
        name: update_localizations,
        path: "/organization/{{RID}}/branding/localizations/{{id}}",
        body: true,
        params: organizational_branding_localization_id
    );
    get!(
        doc: "Get backgroundImage for the navigation property localizations from organization",
        name: get_localizations_background_image,
        path: "/organization/{{RID}}/branding/localizations/{{id}}/backgroundImage",
        params: organizational_branding_localization_id
    );
    put!(
        doc: "Update backgroundImage for the navigation property localizations in organization",
        name: update_localizations_background_image,
        path: "/organization/{{RID}}/branding/localizations/{{id}}/backgroundImage",
        body: true,
        params: organizational_branding_localization_id
    );
    get!(
        doc: "Get bannerLogo for the navigation property localizations from organization",
        name: get_localizations_banner_logo,
        path: "/organization/{{RID}}/branding/localizations/{{id}}/bannerLogo",
        params: organizational_branding_localization_id
    );
    put!(
        doc: "Update bannerLogo for the navigation property localizations in organization",
        name: update_localizations_banner_logo,
        path: "/organization/{{RID}}/branding/localizations/{{id}}/bannerLogo",
        body: true,
        params: organizational_branding_localization_id
    );
    get!(
        doc: "Get squareLogo for the navigation property localizations from organization",
        name: get_localizations_square_logo,
        path: "/organization/{{RID}}/branding/localizations/{{id}}/squareLogo",
        params: organizational_branding_localization_id
    );
    put!(
        doc: "Update squareLogo for the navigation property localizations in organization",
        name: update_localizations_square_logo,
        path: "/organization/{{RID}}/branding/localizations/{{id}}/squareLogo",
        body: true,
        params: organizational_branding_localization_id
    );
    get!(
        doc: "Get squareLogo for the navigation property branding from organization",
        name: get_branding_square_logo,
        path: "/organization/{{RID}}/branding/squareLogo"
    );
    put!(
        doc: "Update squareLogo for the navigation property branding in organization",
        name: update_branding_square_logo,
        path: "/organization/{{RID}}/branding/squareLogo",
        body: true
    );
    get!(
        doc: "List certificateBasedAuthConfigurations",
        name: list_certificate_based_auth_configuration,
        path: "/organization/{{RID}}/certificateBasedAuthConfiguration"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_certificate_based_auth_configuration_count,
        path: "/organization/{{RID}}/certificateBasedAuthConfiguration/$count"
    );
    get!(
        doc: "Get certificateBasedAuthConfiguration from organization",
        name: get_certificate_based_auth_configuration,
        path: "/organization/{{RID}}/certificateBasedAuthConfiguration/{{id}}",
        params: certificate_based_auth_configuration_id
    );
    post!(
        doc: "Invoke action checkMemberGroups",
        name: check_member_groups,
        path: "/organization/{{RID}}/checkMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberObjects",
        name: check_member_objects,
        path: "/organization/{{RID}}/checkMemberObjects",
        body: true
    );
    post!(
        doc: "Create new navigation property to extensions for organization",
        name: create_extensions,
        path: "/organization/{{RID}}/extensions",
        body: true
    );
    get!(
        doc: "Get extensions from organization",
        name: list_extensions,
        path: "/organization/{{RID}}/extensions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_extensions_count,
        path: "/organization/{{RID}}/extensions/$count"
    );
    delete!(
        doc: "Delete navigation property extensions for organization",
        name: delete_extensions,
        path: "/organization/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    get!(
        doc: "Get extensions from organization",
        name: get_extensions,
        path: "/organization/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    patch!(
        doc: "Update the navigation property extensions in organization",
        name: update_extensions,
        path: "/organization/{{RID}}/extensions/{{id}}",
        body: true,
        params: extension_id
    );
    post!(
        doc: "Invoke action getMemberGroups",
        name: get_member_groups,
        path: "/organization/{{RID}}/getMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action getMemberObjects",
        name: get_member_objects,
        path: "/organization/{{RID}}/getMemberObjects",
        body: true
    );
    post!(
        doc: "Invoke action restore",
        name: restore,
        path: "/organization/{{RID}}/restore"
    );
    post!(
        doc: "Invoke action setMobileDeviceManagementAuthority",
        name: set_mobile_device_management_authority,
        path: "/organization/{{RID}}/setMobileDeviceManagementAuthority"
    );
}
