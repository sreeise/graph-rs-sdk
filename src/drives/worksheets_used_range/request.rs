// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    WorksheetsUsedRangeApiClient,
    ResourceIdentity::WorksheetsUsedRange
);

impl WorksheetsUsedRangeApiClient {
    get!(
        doc: "Invoke function usedRange",
        name: workbook_worksheet,
        path: "/usedRange()"
    );
    get!(
        doc: "Invoke function usedRange",
        name: workbook_worksheet,
        path: "/usedRange(valuesOnly={{id}})",
        params: values_only
    );
}
