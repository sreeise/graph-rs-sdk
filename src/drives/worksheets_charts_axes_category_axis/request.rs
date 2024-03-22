// GENERATED CODE

use crate::api_default_imports::*;
use crate::drives::*;

api_client!(
    WorksheetsChartsAxesCategoryAxisApiClient,
    ResourceIdentity::WorksheetsChartsAxesCategoryAxis
);

impl WorksheetsChartsAxesCategoryAxisApiClient {
    api_client_link!(title, WorksheetsChartsTitleApiClient);
    api_client_link!(formatting, WorksheetsChartsFormatApiClient);

    delete!(
        doc: "Delete navigation property categoryAxis for drives",
        name: delete_category_axis,
        path: "/categoryAxis"
    );
    get!(
        doc: "Get categoryAxis from drives",
        name: get_category_axis,
        path: "/categoryAxis"
    );
    patch!(
        doc: "Update the navigation property categoryAxis in drives",
        name: update_category_axis,
        path: "/categoryAxis",
        body: true
    );
    delete!(
        doc: "Delete navigation property majorGridlines for drives",
        name: delete_major_gridlines,
        path: "/categoryAxis/majorGridlines"
    );
    get!(
        doc: "Get majorGridlines from drives",
        name: get_major_gridlines,
        path: "/categoryAxis/majorGridlines"
    );
    patch!(
        doc: "Update the navigation property majorGridlines in drives",
        name: update_major_gridlines,
        path: "/categoryAxis/majorGridlines",
        body: true
    );
    delete!(
        doc: "Delete navigation property minorGridlines for drives",
        name: delete_minor_gridlines,
        path: "/categoryAxis/minorGridlines"
    );
    get!(
        doc: "Get ChartGridlines",
        name: get_minor_gridlines,
        path: "/categoryAxis/minorGridlines"
    );
    patch!(
        doc: "Update chartgridlines",
        name: update_minor_gridlines,
        path: "/categoryAxis/minorGridlines",
        body: true
    );
}
