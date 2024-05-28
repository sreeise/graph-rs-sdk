// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    OnlineMeetingsApiClient,
    OnlineMeetingsIdApiClient,
    ResourceIdentity::OnlineMeetings
);

impl OnlineMeetingsApiClient {
    post!(
        doc: "Create onlineMeeting",
        name: create_online_meetings,
        path: "/onlineMeetings",
        body: true
    );
    get!(
        doc: "Get onlineMeeting",
        name: list_online_meetings,
        path: "/onlineMeetings"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_online_meetings_count,
        path: "/onlineMeetings/$count"
    );
    post!(
        doc: "Invoke action createOrGet",
        name: create_or_get,
        path: "/onlineMeetings/createOrGet",
        body: true
    );
}

impl OnlineMeetingsIdApiClient {
    delete!(
        doc: "Delete navigation property onlineMeetings for users",
        name: delete_online_meetings,
        path: "/onlineMeetings/{{RID}}"
    );
    get!(
        doc: "Get onlineMeetings from users",
        name: get_online_meetings,
        path: "/onlineMeetings/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property onlineMeetings in users",
        name: update_online_meetings,
        path: "/onlineMeetings/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to attendanceReports for users",
        name: create_attendance_reports,
        path: "/onlineMeetings/{{RID}}/attendanceReports",
        body: true
    );
    get!(
        doc: "List meetingAttendanceReports",
        name: list_attendance_reports,
        path: "/onlineMeetings/{{RID}}/attendanceReports"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_attendance_reports_count,
        path: "/onlineMeetings/{{RID}}/attendanceReports/$count"
    );
    delete!(
        doc: "Delete navigation property attendanceReports for users",
        name: delete_attendance_reports,
        path: "/onlineMeetings/{{RID}}/attendanceReports/{{id}}",
        params: meeting_attendance_report_id
    );
    get!(
        doc: "Get attendanceReports from users",
        name: get_attendance_reports,
        path: "/onlineMeetings/{{RID}}/attendanceReports/{{id}}",
        params: meeting_attendance_report_id
    );
    patch!(
        doc: "Update the navigation property attendanceReports in users",
        name: update_attendance_reports,
        path: "/onlineMeetings/{{RID}}/attendanceReports/{{id}}",
        body: true,
        params: meeting_attendance_report_id
    );
    post!(
        doc: "Create new navigation property to attendanceRecords for users",
        name: create_attendance_records,
        path: "/onlineMeetings/{{RID}}/attendanceReports/{{id}}/attendanceRecords",
        body: true,
        params: meeting_attendance_report_id
    );
    get!(
        doc: "List attendanceRecords",
        name: list_attendance_records,
        path: "/onlineMeetings/{{RID}}/attendanceReports/{{id}}/attendanceRecords",
        params: meeting_attendance_report_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_attendance_records_count,
        path: "/onlineMeetings/{{RID}}/attendanceReports/{{id}}/attendanceRecords/$count",
        params: meeting_attendance_report_id
    );
    delete!(
        doc: "Delete navigation property attendanceRecords for users",
        name: delete_attendance_records,
        path: "/onlineMeetings/{{RID}}/attendanceReports/{{id}}/attendanceRecords/{{id2}}",
        params: meeting_attendance_report_id, attendance_record_id
    );
    get!(
        doc: "Get attendanceRecords from users",
        name: get_attendance_records,
        path: "/onlineMeetings/{{RID}}/attendanceReports/{{id}}/attendanceRecords/{{id2}}",
        params: meeting_attendance_report_id, attendance_record_id
    );
    patch!(
        doc: "Update the navigation property attendanceRecords in users",
        name: update_attendance_records,
        path: "/onlineMeetings/{{RID}}/attendanceReports/{{id}}/attendanceRecords/{{id2}}",
        body: true,
        params: meeting_attendance_report_id, attendance_record_id
    );
    get!(
        doc: "Get attendeeReport for the navigation property onlineMeetings from users",
        name: get_online_meetings_attendee_report,
        path: "/onlineMeetings/{{RID}}/attendeeReport"
    );
    put!(
        doc: "Update attendeeReport for the navigation property onlineMeetings in users",
        name: update_online_meetings_attendee_report,
        path: "/onlineMeetings/{{RID}}/attendeeReport",
        body: true
    );
}
