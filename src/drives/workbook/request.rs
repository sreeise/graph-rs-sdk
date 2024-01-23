// GENERATED CODE

use crate::api_default_imports::*;
use crate::drives::*;

resource_api_client!(WorkbookApiClient, ResourceIdentity::Workbook);

impl WorkbookApiClient {
    api_client_link!(functions, WorkbookFunctionsApiClient);
    api_client_link!(tables, WorkbookTablesApiClient);
    api_client_link!(worksheets, WorksheetsApiClient);
    api_client_link_id!(worksheet, WorksheetsIdApiClient);
    api_client_link_id!(table, WorkbookTablesIdApiClient);

    delete!(
        doc: "Delete navigation property workbook for drives",
        name: delete_workbook,
        path: "/workbook"
    );
    get!(
        doc: "Get workbook from drives",
        name: get_workbook,
        path: "/workbook"
    );
    patch!(
        doc: "Update the navigation property workbook in drives",
        name: update_workbook,
        path: "/workbook",
        body: true
    );
    delete!(
        doc: "Delete navigation property application for drives",
        name: delete_application,
        path: "/workbook/application"
    );
    get!(
        doc: "Get workbookApplication",
        name: get_application,
        path: "/workbook/application"
    );
    patch!(
        doc: "Update the navigation property application in drives",
        name: update_application,
        path: "/workbook/application",
        body: true
    );
    post!(
        doc: "Invoke action calculate",
        name: calculate,
        path: "/workbook/application/calculate",
        body: true
    );
    post!(
        doc: "Invoke action closeSession",
        name: close_session,
        path: "/workbook/closeSession"
    );
    post!(
        doc: "Invoke action createSession",
        name: create_session,
        path: "/workbook/createSession",
        body: true
    );
    post!(
        doc: "Create new navigation property to operations for drives",
        name: create_operations,
        path: "/workbook/operations",
        body: true
    );
    get!(
        doc: "Get workbookOperation",
        name: list_operations,
        path: "/workbook/operations"
    );
    get!(
        doc: "Get the number of the resource",
        name: operations,
        path: "/workbook/operations/$count"
    );
    delete!(
        doc: "Delete navigation property operations for drives",
        name: delete_operations,
        path: "/workbook/operations/{{id}}",
        params: workbook_operation_id
    );
    get!(
        doc: "Get workbookOperation",
        name: get_operations,
        path: "/workbook/operations/{{id}}",
        params: workbook_operation_id
    );
    patch!(
        doc: "Update the navigation property operations in drives",
        name: update_operations,
        path: "/workbook/operations/{{id}}",
        body: true,
        params: workbook_operation_id
    );
    post!(
        doc: "Invoke action refreshSession",
        name: refresh_session,
        path: "/workbook/refreshSession"
    );
    get!(
        doc: "Invoke function sessionInfoResource",
        name: session_info_resource,
        path: "/workbook/sessionInfoResource(key='{{id}}')",
        params: key
    );
    get!(
        doc: "Invoke function tableRowOperationResult",
        name: table_row_operation_result,
        path: "/workbook/tableRowOperationResult(key='{{id}}')",
        params: key
    );
}
