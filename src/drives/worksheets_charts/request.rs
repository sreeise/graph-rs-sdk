// GENERATED CODE

use crate::api_default_imports::*;
use crate::drives::*;

resource_api_client!(
    WorksheetsChartsApiClient,
    WorksheetsChartsIdApiClient,
    ResourceIdentity::WorksheetsCharts
);

impl WorksheetsChartsApiClient {
    post!(
        doc: "Create Chart",
        name: create_charts,
        path: "/charts",
        body: true
    );
    get!(
        doc: "List charts",
        name: list_charts,
        path: "/charts"
    );
    post!(
        doc: "Invoke action add",
        name: add,
        path: "/charts/add",
        body: true
    );
    get!(
        doc: "Invoke function count",
        name: count,
        path: "/charts/count()"
    );
    get!(
        doc: "Invoke function item",
        name: item,
        path: "/charts/item(name='{{id}}')",
        params: name
    );
    get!(
        doc: "Invoke function itemAt",
        name: item_at,
        path: "/charts/itemAt(index={{id}})",
        params: index
    );
}

impl WorksheetsChartsIdApiClient {
    api_client_link!(axes, WorksheetsChartsAxesApiClient);
    api_client_link!(series, WorksheetsChartsSeriesApiClient);
    api_client_link!(formatting, WorksheetsChartsFormatApiClient);
    api_client_link!(title, WorksheetsChartsTitleApiClient);
    api_client_link!(legend, WorksheetsChartsLegendApiClient);

    delete!(
        doc: "Chart: delete",
        name: delete_charts,
        path: "/charts/{{RID}}"
    );
    get!(
        doc: "Get Chart",
        name: get_charts,
        path: "/charts/{{RID}}"
    );
    patch!(
        doc: "Update chart",
        name: update_charts,
        path: "/charts/{{RID}}",
        body: true
    );
    get!(
        doc: "Invoke function image",
        name: get_image,
        path: "/charts/{{RID}}/image()"
    );
    get!(
        doc: "Invoke function image",
        name: get_image_with_width,
        path: "/charts/{{RID}}/image(width={{id}})",
        params: width
    );
    get!(
        doc: "Invoke function image",
        name: get_image_with_width_and_height,
        path: "/charts/{{RID}}/image(width={{id}},height={{id2}})",
        params: width, height
    );
    get!(
        doc: "Invoke function image",
        name: get_image_with_width_and_height_and_fitting_mode,
        path: "/charts/{{RID}}/image(width={{id}},height={{id2}},fittingMode='{{id3}}')",
        params: width, height, fitting_mode
    );
    post!(
        doc: "Invoke action setData",
        name: set_data,
        path: "/charts/{{RID}}/setData",
        body: true
    );
    post!(
        doc: "Invoke action setPosition",
        name: set_position,
        path: "/charts/{{RID}}/setPosition",
        body: true
    );
    get!(
        doc: "Get worksheet from drives",
        name: get_worksheet,
        path: "/charts/{{RID}}/worksheet"
    );
}
