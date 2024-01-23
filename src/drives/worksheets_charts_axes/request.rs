// GENERATED CODE

use crate::api_default_imports::*;
use crate::drives::*;

resource_api_client!(
    WorksheetsChartsAxesApiClient,
    ResourceIdentity::WorksheetsChartsAxes
);

impl WorksheetsChartsAxesApiClient {
    api_client_link!(category_axis, WorksheetsChartsAxesApiClient);
    api_client_link!(title, WorksheetsChartsTitleApiClient);
    api_client_link!(value_axis, WorksheetsChartsSeriesApiClient);
    api_client_link!(data_labels, WorksheetsChartsDataLabelsApiClient);
    api_client_link!(formatting, WorksheetsChartsFormatApiClient);
    api_client_link!(series_axis, WorksheetsChartsLegendApiClient);

    delete!(
        doc: "Delete navigation property axes for drives",
        name: delete_axes,
        path: "/axes"
    );
    get!(
        doc: "Get axes from drives",
        name: get_axes,
        path: "/axes"
    );
    patch!(
        doc: "Update the navigation property axes in drives",
        name: update_axes,
        path: "/axes",
        body: true
    );
}
