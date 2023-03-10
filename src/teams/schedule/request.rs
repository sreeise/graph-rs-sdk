// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(ScheduleApiClient, ResourceIdentity::Schedule);

impl ScheduleApiClient {
    delete!(
        doc: "Delete navigation property schedule for teams",
        name: delete_schedule,
        path: "/schedule"
    );
    get!(
        doc: "Get schedule",
        name: get_schedule,
        path: "/schedule"
    );
    put!(
        doc: "Update the navigation property schedule in teams",
        name: update_schedule,
        path: "/schedule",
        body: true
    );
    post!(
        doc: "Create new navigation property to offerShiftRequests for teams",
        name: create_offer_shift_requests,
        path: "/schedule/offerShiftRequests",
        body: true
    );
    get!(
        doc: "List offerShiftRequest",
        name: list_offer_shift_requests,
        path: "/schedule/offerShiftRequests"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_offer_shift_requests_count,
        path: "/schedule/offerShiftRequests/$count"
    );
    delete!(
        doc: "Delete navigation property offerShiftRequests for teams",
        name: delete_offer_shift_requests,
        path: "/schedule/offerShiftRequests/{{id}}",
        params: offer_shift_request_id
    );
    get!(
        doc: "Get offerShiftRequests from teams",
        name: get_offer_shift_requests,
        path: "/schedule/offerShiftRequests/{{id}}",
        params: offer_shift_request_id
    );
    patch!(
        doc: "Update the navigation property offerShiftRequests in teams",
        name: update_offer_shift_requests,
        path: "/schedule/offerShiftRequests/{{id}}",
        body: true,
        params: offer_shift_request_id
    );
    post!(
        doc: "Create openShiftChangeRequest",
        name: create_open_shift_change_requests,
        path: "/schedule/openShiftChangeRequests",
        body: true
    );
    get!(
        doc: "List openShiftChangeRequests",
        name: list_open_shift_change_requests,
        path: "/schedule/openShiftChangeRequests"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_open_shift_change_requests_count,
        path: "/schedule/openShiftChangeRequests/$count"
    );
    delete!(
        doc: "Delete navigation property openShiftChangeRequests for teams",
        name: delete_open_shift_change_requests,
        path: "/schedule/openShiftChangeRequests/{{id}}",
        params: open_shift_change_request_id
    );
    get!(
        doc: "Get openShiftChangeRequests from teams",
        name: get_open_shift_change_requests,
        path: "/schedule/openShiftChangeRequests/{{id}}",
        params: open_shift_change_request_id
    );
    patch!(
        doc: "Update the navigation property openShiftChangeRequests in teams",
        name: update_open_shift_change_requests,
        path: "/schedule/openShiftChangeRequests/{{id}}",
        body: true,
        params: open_shift_change_request_id
    );
    post!(
        doc: "Create new navigation property to openShifts for teams",
        name: create_open_shifts,
        path: "/schedule/openShifts",
        body: true
    );
    get!(
        doc: "List openShifts",
        name: list_open_shifts,
        path: "/schedule/openShifts"
    );
    get!(
        doc: "Get the number of the resource",
        name: open_shifts_bdbd,
        path: "/schedule/openShifts/$count"
    );
    delete!(
        doc: "Delete navigation property openShifts for teams",
        name: delete_open_shifts,
        path: "/schedule/openShifts/{{id}}",
        params: open_shift_id
    );
    get!(
        doc: "Get openShifts from teams",
        name: get_open_shifts,
        path: "/schedule/openShifts/{{id}}",
        params: open_shift_id
    );
    patch!(
        doc: "Update the navigation property openShifts in teams",
        name: update_open_shifts,
        path: "/schedule/openShifts/{{id}}",
        body: true,
        params: open_shift_id
    );
    post!(
        doc: "Create schedulingGroup",
        name: create_scheduling_groups,
        path: "/schedule/schedulingGroups",
        body: true
    );
    get!(
        doc: "List scheduleGroups",
        name: list_scheduling_groups,
        path: "/schedule/schedulingGroups"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_scheduling_groups_count,
        path: "/schedule/schedulingGroups/$count"
    );
    delete!(
        doc: "Delete navigation property schedulingGroups for teams",
        name: delete_scheduling_groups,
        path: "/schedule/schedulingGroups/{{id}}",
        params: scheduling_group_id
    );
    get!(
        doc: "Get schedulingGroups from teams",
        name: get_scheduling_groups,
        path: "/schedule/schedulingGroups/{{id}}",
        params: scheduling_group_id
    );
    patch!(
        doc: "Update the navigation property schedulingGroups in teams",
        name: update_scheduling_groups,
        path: "/schedule/schedulingGroups/{{id}}",
        body: true,
        params: scheduling_group_id
    );
    post!(
        doc: "Invoke action share",
        name: share,
        path: "/schedule/share",
        body: true
    );
    post!(
        doc: "Create shift",
        name: create_shifts,
        path: "/schedule/shifts",
        body: true
    );
    get!(
        doc: "List shifts",
        name: list_shifts,
        path: "/schedule/shifts"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_shifts_count,
        path: "/schedule/shifts/$count"
    );
    delete!(
        doc: "Delete navigation property shifts for teams",
        name: delete_shifts,
        path: "/schedule/shifts/{{id}}",
        params: shift_id
    );
    get!(
        doc: "Get shifts from teams",
        name: get_shifts,
        path: "/schedule/shifts/{{id}}",
        params: shift_id
    );
    patch!(
        doc: "Update the navigation property shifts in teams",
        name: update_shifts,
        path: "/schedule/shifts/{{id}}",
        body: true,
        params: shift_id
    );
    post!(
        doc: "Create swapShiftsChangeRequest",
        name: create_swap_shifts_change_requests,
        path: "/schedule/swapShiftsChangeRequests",
        body: true
    );
    get!(
        doc: "List swapShiftsChangeRequest",
        name: list_swap_shifts_change_requests,
        path: "/schedule/swapShiftsChangeRequests"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_swap_shifts_change_requests_count,
        path: "/schedule/swapShiftsChangeRequests/$count"
    );
    delete!(
        doc: "Delete navigation property swapShiftsChangeRequests for teams",
        name: delete_swap_shifts_change_requests,
        path: "/schedule/swapShiftsChangeRequests/{{id}}",
        params: swap_shifts_change_request_id
    );
    get!(
        doc: "Get swapShiftsChangeRequests from teams",
        name: get_swap_shifts_change_requests,
        path: "/schedule/swapShiftsChangeRequests/{{id}}",
        params: swap_shifts_change_request_id
    );
    patch!(
        doc: "Update the navigation property swapShiftsChangeRequests in teams",
        name: update_swap_shifts_change_requests,
        path: "/schedule/swapShiftsChangeRequests/{{id}}",
        body: true,
        params: swap_shifts_change_request_id
    );
    post!(
        doc: "Create timeOffReason",
        name: create_time_off_reasons,
        path: "/schedule/timeOffReasons",
        body: true
    );
    get!(
        doc: "List timeOffReasons",
        name: list_time_off_reasons,
        path: "/schedule/timeOffReasons"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_time_off_reasons_count,
        path: "/schedule/timeOffReasons/$count"
    );
    delete!(
        doc: "Delete navigation property timeOffReasons for teams",
        name: delete_time_off_reasons,
        path: "/schedule/timeOffReasons/{{id}}",
        params: time_off_reason_id
    );
    get!(
        doc: "Get timeOffReasons from teams",
        name: get_time_off_reasons,
        path: "/schedule/timeOffReasons/{{id}}",
        params: time_off_reason_id
    );
    patch!(
        doc: "Update the navigation property timeOffReasons in teams",
        name: update_time_off_reasons,
        path: "/schedule/timeOffReasons/{{id}}",
        body: true,
        params: time_off_reason_id
    );
    post!(
        doc: "Create new navigation property to timeOffRequests for teams",
        name: create_time_off_requests,
        path: "/schedule/timeOffRequests",
        body: true
    );
    get!(
        doc: "List timeOffRequest",
        name: list_time_off_requests,
        path: "/schedule/timeOffRequests"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_time_off_requests_count,
        path: "/schedule/timeOffRequests/$count"
    );
    delete!(
        doc: "Delete navigation property timeOffRequests for teams",
        name: delete_time_off_requests,
        path: "/schedule/timeOffRequests/{{id}}",
        params: time_off_request_id
    );
    get!(
        doc: "Get timeOffRequests from teams",
        name: get_time_off_requests,
        path: "/schedule/timeOffRequests/{{id}}",
        params: time_off_request_id
    );
    patch!(
        doc: "Update the navigation property timeOffRequests in teams",
        name: update_time_off_requests,
        path: "/schedule/timeOffRequests/{{id}}",
        body: true,
        params: time_off_request_id
    );
    post!(
        doc: "Create timeOff",
        name: create_times_off,
        path: "/schedule/timesOff",
        body: true
    );
    get!(
        doc: "List timesOff",
        name: list_times_off,
        path: "/schedule/timesOff"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_times_off_count,
        path: "/schedule/timesOff/$count"
    );
    delete!(
        doc: "Delete navigation property timesOff for teams",
        name: delete_times_off,
        path: "/schedule/timesOff/{{id}}",
        params: time_off_id
    );
    get!(
        doc: "Get timesOff from teams",
        name: get_times_off,
        path: "/schedule/timesOff/{{id}}",
        params: time_off_id
    );
    patch!(
        doc: "Update the navigation property timesOff in teams",
        name: update_times_off,
        path: "/schedule/timesOff/{{id}}",
        body: true,
        params: time_off_id
    );
}
