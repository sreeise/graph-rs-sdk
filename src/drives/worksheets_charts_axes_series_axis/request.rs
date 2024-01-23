// GENERATED CODE

use crate::api_default_imports::*;
use crate::drives::*;

resource_api_client!(
    WorksheetsChartsAxesSeriesAxisApiClient,
    ResourceIdentity::WorksheetsChartsAxesSeriesAxis
);

impl WorksheetsChartsAxesSeriesAxisApiClient {
    api_client_link!(title, WorksheetsChartsTitleApiClient);
    api_client_link!(formatting, WorksheetsChartsFormatApiClient);

    delete!(
        doc: "Delete navigation property seriesAxis for drives",
        name: delete_series_axis,
        path: "/seriesAxis"
    );
    get!(
        doc: "Get seriesAxis from drives",
        name: get_series_axis,
        path: "/seriesAxis"
    );
    patch!(
        doc: "Update the navigation property seriesAxis in drives",
        name: update_series_axis,
        path: "/seriesAxis",
        body: true
    );
    delete!(
        doc: "Delete navigation property majorGridlines for drives",
        name: delete_major_gridlines,
        path: "/seriesAxis/majorGridlines"
    );
    get!(
        doc: "Get majorGridlines from drives",
        name: get_major_gridlines,
        path: "/seriesAxis/majorGridlines"
    );
    patch!(
        doc: "Update the navigation property majorGridlines in drives",
        name: update_major_gridlines,
        path: "/seriesAxis/majorGridlines",
        body: true
    );
    delete!(
        doc: "Delete navigation property minorGridlines for drives",
        name: delete_minor_gridlines,
        path: "/seriesAxis/minorGridlines"
    );
    get!(
        doc: "Get ChartGridlines",
        name: get_minor_gridlines,
        path: "/seriesAxis/minorGridlines"
    );
    patch!(
        doc: "Update chartgridlines",
        name: update_minor_gridlines,
        path: "/seriesAxis/minorGridlines",
        body: true
    );
}
