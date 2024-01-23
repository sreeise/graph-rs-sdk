// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    WorksheetsChartsSeriesApiClient,
    WorksheetsChartsSeriesIdApiClient,
    ResourceIdentity::WorksheetsChartsSeries
);

impl WorksheetsChartsSeriesApiClient {
    post!(
        doc: "Create ChartSeries",
        name: create_series,
        path: "/series",
        body: true
    );
    get!(
        doc: "List ChartSeriesCollection",
        name: list_series,
        path: "/series"
    );
    get!(
        doc: "Invoke function count",
        name: count,
        path: "/series/count()"
    );
    get!(
        doc: "Invoke function itemAt",
        name: item_at,
        path: "/series/itemAt(index={{id}})",
        params: index
    );
}

impl WorksheetsChartsSeriesIdApiClient {
    delete!(
        doc: "Delete navigation property series for drives",
        name: delete_series,
        path: "/series/{{RID}}"
    );
    get!(
        doc: "Get ChartSeries",
        name: get_series,
        path: "/series/{{RID}}"
    );
    patch!(
        doc: "Update chartseries",
        name: update_series,
        path: "/series/{{RID}}",
        body: true
    );
    post!(
        doc: "Create ChartPoints",
        name: create_points,
        path: "/series/{{RID}}/points",
        body: true
    );
    get!(
        doc: "List ChartPointsCollection",
        name: list_points,
        path: "/series/{{RID}}/points"
    );
    get!(
        doc: "Invoke function count",
        name: count,
        path: "/series/{{RID}}/points/count()"
    );
    get!(
        doc: "Invoke function itemAt",
        name: item_at,
        path: "/series/{{RID}}/points/itemAt(index={{id}})",
        params: index
    );
    delete!(
        doc: "Delete navigation property points for drives",
        name: delete_points,
        path: "/series/{{RID}}/points/{{id}}",
        params: workbook_chart_point_id
    );
    get!(
        doc: "Get ChartPoint",
        name: get_points,
        path: "/series/{{RID}}/points/{{id}}",
        params: workbook_chart_point_id
    );
    patch!(
        doc: "Update the navigation property points in drives",
        name: update_points,
        path: "/series/{{RID}}/points/{{id}}",
        body: true,
        params: workbook_chart_point_id
    );
}
