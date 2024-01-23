// GENERATED CODE

use crate::api_default_imports::*;
use crate::drives::*;

resource_api_client!(
    WorksheetsChartsDataLabelsApiClient,
    ResourceIdentity::WorksheetsChartsDataLabels
);

impl WorksheetsChartsDataLabelsApiClient {
    api_client_link!(formatting, WorksheetsChartsFormatApiClient);

    delete!(
        doc: "Delete navigation property dataLabels for drives",
        name: delete_data_labels,
        path: "/dataLabels"
    );
    get!(
        doc: "Get ChartDataLabels",
        name: get_data_labels,
        path: "/dataLabels"
    );
    patch!(
        doc: "Update chartdatalabels",
        name: update_data_labels,
        path: "/dataLabels",
        body: true
    );
}
