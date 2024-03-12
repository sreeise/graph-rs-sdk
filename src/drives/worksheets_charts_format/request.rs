// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    WorksheetsChartsFormatApiClient,
    ResourceIdentity::WorksheetsChartsFormat
);

impl WorksheetsChartsFormatApiClient {
    delete!(
        doc: "Delete navigation property format for drives",
        name: delete_format,
        path: "/format"
    );
    get!(
        doc: "Get format from drives",
        name: get_format,
        path: "/format"
    );
    patch!(
        doc: "Update the navigation property format in drives",
        name: update_format,
        path: "/format",
        body: true
    );
    delete!(
        doc: "Delete navigation property fill for drives",
        name: delete_fill,
        path: "/format/fill"
    );
    get!(
        doc: "Get fill from drives",
        name: get_fill,
        path: "/format/fill"
    );
    patch!(
        doc: "Update the navigation property fill in drives",
        name: update_fill,
        path: "/format/fill",
        body: true
    );
    post!(
        doc: "Invoke action clear",
        name: clear,
        path: "/format/fill/clear"
    );
    post!(
        doc: "Invoke action setSolidColor",
        name: set_solid_color,
        path: "/format/fill/setSolidColor",
        body: true
    );
    delete!(
        doc: "Delete navigation property font for drives",
        name: delete_font,
        path: "/format/font"
    );
    get!(
        doc: "Get font from drives",
        name: get_font,
        path: "/format/font"
    );
    patch!(
        doc: "Update the navigation property font in drives",
        name: update_font,
        path: "/format/font",
        body: true
    );
}
