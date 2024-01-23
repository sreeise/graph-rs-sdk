// GENERATED CODE

use crate::api_default_imports::*;
use crate::drives::*;

resource_api_client!(
    WorksheetsChartsAxesValueAxisApiClient,
    ResourceIdentity::WorksheetsChartsAxesValueAxis
);

impl WorksheetsChartsAxesValueAxisApiClient {
    api_client_link!(title, WorksheetsChartsTitleApiClient);
    api_client_link!(formatting, WorksheetsChartsFormatApiClient);

    delete!(
        doc: "Delete navigation property valueAxis for drives",
        name: delete_value_axis,
        path: "/valueAxis"
    );
    get!(
        doc: "Get ChartAxis",
        name: get_value_axis,
        path: "/valueAxis"
    );
    patch!(
        doc: "Update chartaxis",
        name: update_value_axis,
        path: "/valueAxis",
        body: true
    );
    delete!(
        doc: "Delete navigation property majorGridlines for drives",
        name: delete_major_gridlines,
        path: "/valueAxis/majorGridlines"
    );
    get!(
        doc: "Get majorGridlines from drives",
        name: get_major_gridlines,
        path: "/valueAxis/majorGridlines"
    );
    patch!(
        doc: "Update the navigation property majorGridlines in drives",
        name: update_major_gridlines,
        path: "/valueAxis/majorGridlines",
        body: true
    );
    delete!(
        doc: "Delete navigation property minorGridlines for drives",
        name: delete_minor_gridlines,
        path: "/valueAxis/minorGridlines"
    );
    get!(
        doc: "Get ChartGridlines",
        name: get_minor_gridlines,
        path: "/valueAxis/minorGridlines"
    );
    patch!(
        doc: "Update chartgridlines",
        name: update_minor_gridlines,
        path: "/valueAxis/minorGridlines",
        body: true
    );
}
