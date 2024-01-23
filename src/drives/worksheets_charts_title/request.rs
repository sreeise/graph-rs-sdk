// GENERATED CODE

use crate::api_default_imports::*;
use crate::drives::*;

resource_api_client!(
    WorksheetsChartsTitleApiClient,
    ResourceIdentity::WorksheetsChartsTitle
);

impl WorksheetsChartsTitleApiClient {
    api_client_link!(formatting, WorksheetsChartsFormatApiClient);

    delete!(
        doc: "Delete navigation property title for drives",
        name: delete_title,
        path: "/title"
    );
    get!(
        doc: "Get ChartTitle",
        name: get_title,
        path: "/title"
    );
    patch!(
        doc: "Update charttitle",
        name: update_title,
        path: "/title",
        body: true
    );
}
