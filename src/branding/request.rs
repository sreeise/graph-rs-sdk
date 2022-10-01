// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(BrandingRequest,);

impl<'a, Client> BrandingRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "Get branding",
        name: get_organizational_branding,
        response: serde_json::Value,
        path: "/branding",
        has_body: false
    });
    patch!({
        doc: "Update branding",
        name: update_organizational_branding,
        response: NoContent,
        path: "/branding",
        has_body: true
    });
    get!({
        doc: "Get backgroundImage for organizationalBranding from branding",
        name: get_background_image,
        response: serde_json::Value,
        path: "/branding/backgroundImage",
        has_body: false
    });
    put!({
        doc: "Update backgroundImage for organizationalBranding in branding",
        name: update_background_image,
        response: NoContent,
        path: "/branding/backgroundImage",
        has_body: true
    });
    get!({
        doc: "Get bannerLogo for organizationalBranding from branding",
        name: get_banner_logo,
        response: serde_json::Value,
        path: "/branding/bannerLogo",
        has_body: false
    });
    put!({
        doc: "Update bannerLogo for organizationalBranding in branding",
        name: update_banner_logo,
        response: NoContent,
        path: "/branding/bannerLogo",
        has_body: true
    });
    post!({
        doc: "Create new navigation property to localizations for branding",
        name: create_localizations,
        response: serde_json::Value,
        path: "/branding/localizations",
        has_body: true
    });
    get!({
        doc: "Get localizations from branding",
        name: list_localizations,
        response: serde_json::Value,
        path: "/branding/localizations",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_localizations_count,
        response: serde_json::Value,
        path: "/branding/localizations/$count",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property localizations for branding",
        name: delete_localizations,
        response: NoContent,
        path: "/branding/localizations/{{id}}",
        params: [ organizational_branding_localization_id ],
        has_body: false
    });
    get!({
        doc: "Get localizations from branding",
        name: get_localizations,
        response: serde_json::Value,
        path: "/branding/localizations/{{id}}",
        params: [ organizational_branding_localization_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property localizations in branding",
        name: update_localizations,
        response: NoContent,
        path: "/branding/localizations/{{id}}",
        params: [ organizational_branding_localization_id ],
        has_body: true
    });
    get!({
        doc: "Get backgroundImage for the navigation property localizations from branding",
        name: get_localizations_background_image,
        response: serde_json::Value,
        path: "/branding/localizations/{{id}}/backgroundImage",
        params: [ organizational_branding_localization_id ],
        has_body: false
    });
    put!({
        doc: "Update backgroundImage for the navigation property localizations in branding",
        name: update_localizations_background_image,
        response: NoContent,
        path: "/branding/localizations/{{id}}/backgroundImage",
        params: [ organizational_branding_localization_id ],
        has_body: true
    });
    get!({
        doc: "Get bannerLogo for the navigation property localizations from branding",
        name: get_localizations_banner_logo,
        response: serde_json::Value,
        path: "/branding/localizations/{{id}}/bannerLogo",
        params: [ organizational_branding_localization_id ],
        has_body: false
    });
    put!({
        doc: "Update bannerLogo for the navigation property localizations in branding",
        name: update_localizations_banner_logo,
        response: NoContent,
        path: "/branding/localizations/{{id}}/bannerLogo",
        params: [ organizational_branding_localization_id ],
        has_body: true
    });
    get!({
        doc: "Get squareLogo for the navigation property localizations from branding",
        name: get_localizations_square_logo,
        response: serde_json::Value,
        path: "/branding/localizations/{{id}}/squareLogo",
        params: [ organizational_branding_localization_id ],
        has_body: false
    });
    put!({
        doc: "Update squareLogo for the navigation property localizations in branding",
        name: update_localizations_square_logo,
        response: NoContent,
        path: "/branding/localizations/{{id}}/squareLogo",
        params: [ organizational_branding_localization_id ],
        has_body: true
    });
    get!({
        doc: "Get squareLogo for organizationalBranding from branding",
        name: get_square_logo,
        response: serde_json::Value,
        path: "/branding/squareLogo",
        has_body: false
    });
    put!({
        doc: "Update squareLogo for organizationalBranding in branding",
        name: update_square_logo,
        response: NoContent,
        path: "/branding/squareLogo",
        has_body: true
    });
}
