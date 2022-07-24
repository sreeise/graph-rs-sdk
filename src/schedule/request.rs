// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(ScheduleRequest,);

impl<'a, Client> ScheduleRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "Get schedule from teams",
        name: get_schedule,
        response: serde_json::Value,
        path: "/schedule",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property schedule for teams",
        name: delete_schedule,
        response: NoContent,
        path: "/schedule",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property schedule in teams",
        name: update_schedule,
        response: NoContent,
        path: "/schedule",
        has_body: true
    });
    post!({
        doc: "Invoke action share",
        name: share,
        response: NoContent,
        path: "/schedule/microsoft.graph.share",
        has_body: true
    });
    post!({
        doc: "Create new navigation property to offerShiftRequests for teams",
        name: create_offer_shift_requests,
        response: serde_json::Value,
        path: "/schedule/offerShiftRequests",
        has_body: true
    });
    get!({
        doc: "Get offerShiftRequests from teams",
        name: list_offer_shift_requests,
        response: serde_json::Value,
        path: "/schedule/offerShiftRequests",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_offer_shift_requests_count,
        response: serde_json::Value,
        path: "/schedule/offerShiftRequests/$count",
        has_body: false
    });
    get!({
        doc: "Get offerShiftRequests from teams",
        name: get_offer_shift_requests,
        response: serde_json::Value,
        path: "/schedule/offerShiftRequests/{{id}}",
        params: [ offer_shift_request_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property offerShiftRequests in teams",
        name: update_offer_shift_requests,
        response: NoContent,
        path: "/schedule/offerShiftRequests/{{id}}",
        params: [ offer_shift_request_id ],
        has_body: true
    });
    delete!({
        doc: "Delete navigation property offerShiftRequests for teams",
        name: delete_offer_shift_requests,
        response: NoContent,
        path: "/schedule/offerShiftRequests/{{id}}",
        params: [ offer_shift_request_id ],
        has_body: false
    });
    get!({
        doc: "Get openShiftChangeRequests from teams",
        name: list_open_shift_change_requests,
        response: serde_json::Value,
        path: "/schedule/openShiftChangeRequests",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to openShiftChangeRequests for teams",
        name: create_open_shift_change_requests,
        response: serde_json::Value,
        path: "/schedule/openShiftChangeRequests",
        has_body: true
    });
    get!({
        doc: "Get the number of the resource",
        name: get_open_shift_change_requests_count,
        response: serde_json::Value,
        path: "/schedule/openShiftChangeRequests/$count",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property openShiftChangeRequests in teams",
        name: update_open_shift_change_requests,
        response: NoContent,
        path: "/schedule/openShiftChangeRequests/{{id}}",
        params: [ open_shift_change_request_id ],
        has_body: true
    });
    delete!({
        doc: "Delete navigation property openShiftChangeRequests for teams",
        name: delete_open_shift_change_requests,
        response: NoContent,
        path: "/schedule/openShiftChangeRequests/{{id}}",
        params: [ open_shift_change_request_id ],
        has_body: false
    });
    get!({
        doc: "Get openShiftChangeRequests from teams",
        name: get_open_shift_change_requests,
        response: serde_json::Value,
        path: "/schedule/openShiftChangeRequests/{{id}}",
        params: [ open_shift_change_request_id ],
        has_body: false
    });
    post!({
        doc: "Create new navigation property to openShifts for teams",
        name: create_open_shifts,
        response: serde_json::Value,
        path: "/schedule/openShifts",
        has_body: true
    });
    get!({
        doc: "Get openShifts from teams",
        name: list_open_shifts,
        response: serde_json::Value,
        path: "/schedule/openShifts",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_open_shifts_count,
        response: serde_json::Value,
        path: "/schedule/openShifts/$count",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property openShifts in teams",
        name: update_open_shifts,
        response: NoContent,
        path: "/schedule/openShifts/{{id}}",
        params: [ open_shift_id ],
        has_body: true
    });
    get!({
        doc: "Get openShifts from teams",
        name: get_open_shifts,
        response: serde_json::Value,
        path: "/schedule/openShifts/{{id}}",
        params: [ open_shift_id ],
        has_body: false
    });
    delete!({
        doc: "Delete navigation property openShifts for teams",
        name: delete_open_shifts,
        response: NoContent,
        path: "/schedule/openShifts/{{id}}",
        params: [ open_shift_id ],
        has_body: false
    });
    post!({
        doc: "Create new navigation property to schedulingGroups for teams",
        name: create_scheduling_groups,
        response: serde_json::Value,
        path: "/schedule/schedulingGroups",
        has_body: true
    });
    get!({
        doc: "Get schedulingGroups from teams",
        name: list_scheduling_groups,
        response: serde_json::Value,
        path: "/schedule/schedulingGroups",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_scheduling_groups_count,
        response: serde_json::Value,
        path: "/schedule/schedulingGroups/$count",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property schedulingGroups in teams",
        name: update_scheduling_groups,
        response: NoContent,
        path: "/schedule/schedulingGroups/{{id}}",
        params: [ scheduling_group_id ],
        has_body: true
    });
    delete!({
        doc: "Delete navigation property schedulingGroups for teams",
        name: delete_scheduling_groups,
        response: NoContent,
        path: "/schedule/schedulingGroups/{{id}}",
        params: [ scheduling_group_id ],
        has_body: false
    });
    get!({
        doc: "Get schedulingGroups from teams",
        name: get_scheduling_groups,
        response: serde_json::Value,
        path: "/schedule/schedulingGroups/{{id}}",
        params: [ scheduling_group_id ],
        has_body: false
    });
    get!({
        doc: "Get shifts from teams",
        name: list_shifts,
        response: serde_json::Value,
        path: "/schedule/shifts",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to shifts for teams",
        name: create_shifts,
        response: serde_json::Value,
        path: "/schedule/shifts",
        has_body: true
    });
    get!({
        doc: "Get the number of the resource",
        name: get_shifts_count,
        response: serde_json::Value,
        path: "/schedule/shifts/$count",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property shifts for teams",
        name: delete_shifts,
        response: NoContent,
        path: "/schedule/shifts/{{id}}",
        params: [ shift_id ],
        has_body: false
    });
    get!({
        doc: "Get shifts from teams",
        name: get_shifts,
        response: serde_json::Value,
        path: "/schedule/shifts/{{id}}",
        params: [ shift_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property shifts in teams",
        name: update_shifts,
        response: NoContent,
        path: "/schedule/shifts/{{id}}",
        params: [ shift_id ],
        has_body: true
    });
    get!({
        doc: "Get swapShiftsChangeRequests from teams",
        name: list_swap_shifts_change_requests,
        response: serde_json::Value,
        path: "/schedule/swapShiftsChangeRequests",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to swapShiftsChangeRequests for teams",
        name: create_swap_shifts_change_requests,
        response: serde_json::Value,
        path: "/schedule/swapShiftsChangeRequests",
        has_body: true
    });
    get!({
        doc: "Get the number of the resource",
        name: get_swap_shifts_change_requests_count,
        response: serde_json::Value,
        path: "/schedule/swapShiftsChangeRequests/$count",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property swapShiftsChangeRequests for teams",
        name: delete_swap_shifts_change_requests,
        response: NoContent,
        path: "/schedule/swapShiftsChangeRequests/{{id}}",
        params: [ swap_shifts_change_request_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property swapShiftsChangeRequests in teams",
        name: update_swap_shifts_change_requests,
        response: NoContent,
        path: "/schedule/swapShiftsChangeRequests/{{id}}",
        params: [ swap_shifts_change_request_id ],
        has_body: true
    });
    get!({
        doc: "Get swapShiftsChangeRequests from teams",
        name: get_swap_shifts_change_requests,
        response: serde_json::Value,
        path: "/schedule/swapShiftsChangeRequests/{{id}}",
        params: [ swap_shifts_change_request_id ],
        has_body: false
    });
    get!({
        doc: "Get timeOffReasons from teams",
        name: list_time_off_reasons,
        response: serde_json::Value,
        path: "/schedule/timeOffReasons",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to timeOffReasons for teams",
        name: create_time_off_reasons,
        response: serde_json::Value,
        path: "/schedule/timeOffReasons",
        has_body: true
    });
    get!({
        doc: "Get the number of the resource",
        name: get_time_off_reasons_count,
        response: serde_json::Value,
        path: "/schedule/timeOffReasons/$count",
        has_body: false
    });
    get!({
        doc: "Get timeOffReasons from teams",
        name: get_time_off_reasons,
        response: serde_json::Value,
        path: "/schedule/timeOffReasons/{{id}}",
        params: [ time_off_reason_id ],
        has_body: false
    });
    delete!({
        doc: "Delete navigation property timeOffReasons for teams",
        name: delete_time_off_reasons,
        response: NoContent,
        path: "/schedule/timeOffReasons/{{id}}",
        params: [ time_off_reason_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property timeOffReasons in teams",
        name: update_time_off_reasons,
        response: NoContent,
        path: "/schedule/timeOffReasons/{{id}}",
        params: [ time_off_reason_id ],
        has_body: true
    });
    post!({
        doc: "Create new navigation property to timeOffRequests for teams",
        name: create_time_off_requests,
        response: serde_json::Value,
        path: "/schedule/timeOffRequests",
        has_body: true
    });
    get!({
        doc: "Get timeOffRequests from teams",
        name: list_time_off_requests,
        response: serde_json::Value,
        path: "/schedule/timeOffRequests",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_time_off_requests_count,
        response: serde_json::Value,
        path: "/schedule/timeOffRequests/$count",
        has_body: false
    });
    get!({
        doc: "Get timeOffRequests from teams",
        name: get_time_off_requests,
        response: serde_json::Value,
        path: "/schedule/timeOffRequests/{{id}}",
        params: [ time_off_request_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property timeOffRequests in teams",
        name: update_time_off_requests,
        response: NoContent,
        path: "/schedule/timeOffRequests/{{id}}",
        params: [ time_off_request_id ],
        has_body: true
    });
    delete!({
        doc: "Delete navigation property timeOffRequests for teams",
        name: delete_time_off_requests,
        response: NoContent,
        path: "/schedule/timeOffRequests/{{id}}",
        params: [ time_off_request_id ],
        has_body: false
    });
    get!({
        doc: "Get timesOff from teams",
        name: list_times_off,
        response: serde_json::Value,
        path: "/schedule/timesOff",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to timesOff for teams",
        name: create_times_off,
        response: serde_json::Value,
        path: "/schedule/timesOff",
        has_body: true
    });
    get!({
        doc: "Get the number of the resource",
        name: get_times_off_count,
        response: serde_json::Value,
        path: "/schedule/timesOff/$count",
        has_body: false
    });
    get!({
        doc: "Get timesOff from teams",
        name: get_times_off,
        response: serde_json::Value,
        path: "/schedule/timesOff/{{id}}",
        params: [ time_off_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property timesOff in teams",
        name: update_times_off,
        response: NoContent,
        path: "/schedule/timesOff/{{id}}",
        params: [ time_off_id ],
        has_body: true
    });
    delete!({
        doc: "Delete navigation property timesOff for teams",
        name: delete_times_off,
        response: NoContent,
        path: "/schedule/timesOff/{{id}}",
        params: [ time_off_id ],
        has_body: false
    });
}
