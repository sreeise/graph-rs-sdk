// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    VirtualEventsSessionsApiClient,
    VirtualEventsSessionsIdApiClient,
    ResourceIdentity::VirtualEventsSessions
);

impl VirtualEventsSessionsApiClient {
    post!(
        doc: "Create new navigation property to sessions for solutions",
        name: create_sessions,
        path: "/sessions",
        body: true
    );
    get!(
        doc: "Get sessions from solutions",
        name: list_sessions,
        path: "/sessions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_sessions_count,
        path: "/sessions/$count"
    );
}

impl VirtualEventsSessionsIdApiClient {
    delete!(
        doc: "Delete navigation property sessions for solutions",
        name: delete_sessions,
        path: "/sessions/{{RID}}"
    );
    get!(
        doc: "Get sessions from solutions",
        name: get_sessions,
        path: "/sessions/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property sessions in solutions",
        name: update_sessions,
        path: "/sessions/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to attendanceReports for solutions",
        name: create_attendance_reports,
        path: "/sessions/{{RID}}/attendanceReports",
        body: true
    );
    get!(
        doc: "Get attendanceReports from solutions",
        name: list_attendance_reports,
        path: "/sessions/{{RID}}/attendanceReports"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_attendance_reports_count,
        path: "/sessions/{{RID}}/attendanceReports/$count"
    );
    delete!(
        doc: "Delete navigation property attendanceReports for solutions",
        name: delete_attendance_reports,
        path: "/sessions/{{RID}}/attendanceReports/{{id}}",
        params: meeting_attendance_report_id
    );
    get!(
        doc: "Get attendanceReports from solutions",
        name: get_attendance_reports,
        path: "/sessions/{{RID}}/attendanceReports/{{id}}",
        params: meeting_attendance_report_id
    );
    patch!(
        doc: "Update the navigation property attendanceReports in solutions",
        name: update_attendance_reports,
        path: "/sessions/{{RID}}/attendanceReports/{{id}}",
        body: true,
        params: meeting_attendance_report_id
    );
    post!(
        doc: "Create new navigation property to attendanceRecords for solutions",
        name: create_attendance_records,
        path: "/sessions/{{RID}}/attendanceReports/{{id}}/attendanceRecords",
        body: true,
        params: meeting_attendance_report_id
    );
    get!(
        doc: "Get attendanceRecords from solutions",
        name: list_attendance_records,
        path: "/sessions/{{RID}}/attendanceReports/{{id}}/attendanceRecords",
        params: meeting_attendance_report_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_attendance_records_count,
        path: "/sessions/{{RID}}/attendanceReports/{{id}}/attendanceRecords/$count",
        params: meeting_attendance_report_id
    );
    delete!(
        doc: "Delete navigation property attendanceRecords for solutions",
        name: delete_attendance_records,
        path: "/sessions/{{RID}}/attendanceReports/{{id}}/attendanceRecords/{{id2}}",
        params: meeting_attendance_report_id, attendance_record_id
    );
    get!(
        doc: "Get attendanceRecords from solutions",
        name: get_attendance_records,
        path: "/sessions/{{RID}}/attendanceReports/{{id}}/attendanceRecords/{{id2}}",
        params: meeting_attendance_report_id, attendance_record_id
    );
    patch!(
        doc: "Update the navigation property attendanceRecords in solutions",
        name: update_attendance_records,
        path: "/sessions/{{RID}}/attendanceReports/{{id}}/attendanceRecords/{{id2}}",
        body: true,
        params: meeting_attendance_report_id, attendance_record_id
    );
}
