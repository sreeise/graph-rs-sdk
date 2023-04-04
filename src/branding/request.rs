// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(BrandingApiClient, ResourceIdentity::Branding);

impl BrandingApiClient {
    get!(
        doc: "Get branding",
        name: get_organizational_branding,
        path: "/branding"
    );
    patch!(
        doc: "Update branding",
        name: update_organizational_branding,
        path: "/branding",
        body: true
    );
    get!(
        doc: "Get backgroundImage for organizationalBranding from branding",
        name: get_background_image,
        path: "/branding/backgroundImage"
    );
    put!(
        doc: "Update backgroundImage for organizationalBranding in branding",
        name: update_background_image,
        path: "/branding/backgroundImage",
        body: true
    );
    get!(
        doc: "Get bannerLogo for organizationalBranding from branding",
        name: get_banner_logo,
        path: "/branding/bannerLogo"
    );
    put!(
        doc: "Update bannerLogo for organizationalBranding in branding",
        name: update_banner_logo,
        path: "/branding/bannerLogo",
        body: true
    );
    post!(
        doc: "Create organizationalBrandingLocalization",
        name: create_localizations,
        path: "/branding/localizations",
        body: true
    );
    get!(
        doc: "List localizations",
        name: list_localizations,
        path: "/branding/localizations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_localizations_count,
        path: "/branding/localizations/$count"
    );
    delete!(
        doc: "Delete navigation property localizations for branding",
        name: delete_localizations,
        path: "/branding/localizations/{{id}}",
        params: organizational_branding_localization_id
    );
    get!(
        doc: "Get localizations from branding",
        name: get_localizations,
        path: "/branding/localizations/{{id}}",
        params: organizational_branding_localization_id
    );
    patch!(
        doc: "Update the navigation property localizations in branding",
        name: update_localizations,
        path: "/branding/localizations/{{id}}",
        body: true,
        params: organizational_branding_localization_id
    );
    get!(
        doc: "Get backgroundImage for the navigation property localizations from branding",
        name: get_localizations_background_image,
        path: "/branding/localizations/{{id}}/backgroundImage",
        params: organizational_branding_localization_id
    );
    put!(
        doc: "Update backgroundImage for the navigation property localizations in branding",
        name: update_localizations_background_image,
        path: "/branding/localizations/{{id}}/backgroundImage",
        body: true,
        params: organizational_branding_localization_id
    );
    get!(
        doc: "Get bannerLogo for the navigation property localizations from branding",
        name: get_localizations_banner_logo,
        path: "/branding/localizations/{{id}}/bannerLogo",
        params: organizational_branding_localization_id
    );
    put!(
        doc: "Update bannerLogo for the navigation property localizations in branding",
        name: update_localizations_banner_logo,
        path: "/branding/localizations/{{id}}/bannerLogo",
        body: true,
        params: organizational_branding_localization_id
    );
    get!(
        doc: "Get squareLogo for the navigation property localizations from branding",
        name: get_localizations_square_logo,
        path: "/branding/localizations/{{id}}/squareLogo",
        params: organizational_branding_localization_id
    );
    put!(
        doc: "Update squareLogo for the navigation property localizations in branding",
        name: update_localizations_square_logo,
        path: "/branding/localizations/{{id}}/squareLogo",
        body: true,
        params: organizational_branding_localization_id
    );
    get!(
        doc: "Get squareLogo for organizationalBranding from branding",
        name: get_square_logo,
        path: "/branding/squareLogo"
    );
    put!(
        doc: "Update squareLogo for organizationalBranding in branding",
        name: update_square_logo,
        path: "/branding/squareLogo",
        body: true
    );
}
