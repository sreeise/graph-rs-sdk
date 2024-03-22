// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    WorksheetsChartsLegendApiClient,
    ResourceIdentity::WorksheetsChartsLegend
);

impl WorksheetsChartsLegendApiClient {
    delete!(
        doc: "Delete navigation property legend for drives",
        name: delete_legend,
        path: "/legend"
    );
    get!(
        doc: "Get ChartLegend",
        name: get_legend,
        path: "/legend"
    );
    patch!(
        doc: "Update chartlegend",
        name: update_legend,
        path: "/legend",
        body: true
    );
}
