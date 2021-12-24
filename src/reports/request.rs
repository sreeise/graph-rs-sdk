// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;
use graph_http::{AsyncDownload, AsyncHttpClient, BlockingDownload, BlockingHttpClient};

register_client!(ReportsRequest,);

impl<'a, Client> ReportsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "Get reports",
        name: get_report_root,
        response: serde_json::Value,
        path: "/reports",
        has_body: false
    });
    patch!({
        doc: "Update reports",
        name: update_report_root,
        response: NoContent,
        path: "/reports",
        has_body: true
    });
    get!({
        doc: "Get dailyPrintUsageByPrinter from reports",
        name: list_daily_print_usage_by_printer,
        response: serde_json::Value,
        path: "/reports/dailyPrintUsageByPrinter",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to dailyPrintUsageByPrinter for reports",
        name: create_daily_print_usage_by_printer,
        response: serde_json::Value,
        path: "/reports/dailyPrintUsageByPrinter",
        has_body: true
    });
    delete!({
        doc: "Delete navigation property dailyPrintUsageByPrinter for reports",
        name: delete_daily_print_usage_by_printer,
        response: NoContent,
        path: "/reports/dailyPrintUsageByPrinter//{{id}}",
        params: [ print_usage_by_printer_id ],
        has_body: false
    });
    get!({
        doc: "Get dailyPrintUsageByPrinter from reports",
        name: get_daily_print_usage_by_printer,
        response: serde_json::Value,
        path: "/reports/dailyPrintUsageByPrinter//{{id}}",
        params: [ print_usage_by_printer_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property dailyPrintUsageByPrinter in reports",
        name: update_daily_print_usage_by_printer,
        response: NoContent,
        path: "/reports/dailyPrintUsageByPrinter//{{id}}",
        params: [ print_usage_by_printer_id ],
        has_body: true
    });
    get!({
        doc: "Get dailyPrintUsageByUser from reports",
        name: list_daily_print_usage_by_user,
        response: serde_json::Value,
        path: "/reports/dailyPrintUsageByUser",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to dailyPrintUsageByUser for reports",
        name: create_daily_print_usage_by_user,
        response: serde_json::Value,
        path: "/reports/dailyPrintUsageByUser",
        has_body: true
    });
    delete!({
        doc: "Delete navigation property dailyPrintUsageByUser for reports",
        name: delete_daily_print_usage_by_user,
        response: NoContent,
        path: "/reports/dailyPrintUsageByUser//{{id}}",
        params: [ print_usage_by_user_id ],
        has_body: false
    });
    get!({
        doc: "Get dailyPrintUsageByUser from reports",
        name: get_daily_print_usage_by_user,
        response: serde_json::Value,
        path: "/reports/dailyPrintUsageByUser//{{id}}",
        params: [ print_usage_by_user_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property dailyPrintUsageByUser in reports",
        name: update_daily_print_usage_by_user,
        response: NoContent,
        path: "/reports/dailyPrintUsageByUser//{{id}}",
        params: [ print_usage_by_user_id ],
        has_body: true
    });
    get!({
        doc: "Invoke function getGroupArchivedPrintJobs",
        name: get_group_archived_print_jobs,
        response: serde_json::Value,
        path: "/reports/microsoft.graph.getGroupArchivedPrintJobs(groupId='{{id}}',startDateTime={{id2}},endDateTime={{id3}})",
        params: [ group_id  start_date_time  end_date_time ],
        has_body: false
    });
    get!({
        doc: "Invoke function getPrinterArchivedPrintJobs",
        name: get_printer_archived_print_jobs,
        response: serde_json::Value,
        path: "/reports/microsoft.graph.getPrinterArchivedPrintJobs(printerId='{{id}}',startDateTime={{id2}},endDateTime={{id3}})",
        params: [ printer_id  start_date_time  end_date_time ],
        has_body: false
    });
    get!({
        doc: "Invoke function getUserArchivedPrintJobs",
        name: get_user_archived_print_jobs,
        response: serde_json::Value,
        path: "/reports/microsoft.graph.getUserArchivedPrintJobs(userId='{{id}}',startDateTime={{id2}},endDateTime={{id3}})",
        params: [ user_id  start_date_time  end_date_time ],
        has_body: false
    });
    post!({
        doc: "Create new navigation property to monthlyPrintUsageByPrinter for reports",
        name: create_monthly_print_usage_by_printer,
        response: serde_json::Value,
        path: "/reports/monthlyPrintUsageByPrinter",
        has_body: true
    });
    get!({
        doc: "Get monthlyPrintUsageByPrinter from reports",
        name: list_monthly_print_usage_by_printer,
        response: serde_json::Value,
        path: "/reports/monthlyPrintUsageByPrinter",
        has_body: false
    });
    get!({
        doc: "Get monthlyPrintUsageByPrinter from reports",
        name: get_monthly_print_usage_by_printer,
        response: serde_json::Value,
        path: "/reports/monthlyPrintUsageByPrinter//{{id}}",
        params: [ print_usage_by_printer_id ],
        has_body: false
    });
    delete!({
        doc: "Delete navigation property monthlyPrintUsageByPrinter for reports",
        name: delete_monthly_print_usage_by_printer,
        response: NoContent,
        path: "/reports/monthlyPrintUsageByPrinter//{{id}}",
        params: [ print_usage_by_printer_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property monthlyPrintUsageByPrinter in reports",
        name: update_monthly_print_usage_by_printer,
        response: NoContent,
        path: "/reports/monthlyPrintUsageByPrinter//{{id}}",
        params: [ print_usage_by_printer_id ],
        has_body: true
    });
    get!({
        doc: "Get monthlyPrintUsageByUser from reports",
        name: list_monthly_print_usage_by_user,
        response: serde_json::Value,
        path: "/reports/monthlyPrintUsageByUser",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to monthlyPrintUsageByUser for reports",
        name: create_monthly_print_usage_by_user,
        response: serde_json::Value,
        path: "/reports/monthlyPrintUsageByUser",
        has_body: true
    });
    patch!({
        doc: "Update the navigation property monthlyPrintUsageByUser in reports",
        name: update_monthly_print_usage_by_user,
        response: NoContent,
        path: "/reports/monthlyPrintUsageByUser//{{id}}",
        params: [ print_usage_by_user_id ],
        has_body: true
    });
    delete!({
        doc: "Delete navigation property monthlyPrintUsageByUser for reports",
        name: delete_monthly_print_usage_by_user,
        response: NoContent,
        path: "/reports/monthlyPrintUsageByUser//{{id}}",
        params: [ print_usage_by_user_id ],
        has_body: false
    });
    get!({
        doc: "Get monthlyPrintUsageByUser from reports",
        name: get_monthly_print_usage_by_user,
        response: serde_json::Value,
        path: "/reports/monthlyPrintUsageByUser//{{id}}",
        params: [ print_usage_by_user_id ],
        has_body: false
    });
}

impl<'a> ReportsRequest<'a, BlockingHttpClient> {
    download!({
        doc: "Invoke function deviceConfigurationDeviceActivity",
        name: device_configuration_device_activity,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.deviceConfigurationDeviceActivity()",
        has_body: false
    });
    download!({
        doc: "Invoke function deviceConfigurationUserActivity",
        name: device_configuration_user_activity,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.deviceConfigurationUserActivity()",
        has_body: false
    });
    download!({
        doc: "Invoke function getEmailActivityCounts",
        name: get_email_activity_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getEmailActivityCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getEmailActivityUserCounts",
        name: get_email_activity_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getEmailActivityUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getEmailActivityUserDetail",
        name: get_email_activity_user_detail_fe_32,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getEmailActivityUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    download!({
        doc: "Invoke function getEmailActivityUserDetail",
        name: get_email_activity_user_detail_ddb_2,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getEmailActivityUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getEmailAppUsageAppsUserCounts",
        name: get_email_app_usage_apps_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getEmailAppUsageAppsUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getEmailAppUsageUserCounts",
        name: get_email_app_usage_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getEmailAppUsageUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getEmailAppUsageUserDetail",
        name: get_email_app_usage_user_detail_6_2ec,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getEmailAppUsageUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    download!({
        doc: "Invoke function getEmailAppUsageUserDetail",
        name: get_email_app_usage_user_detail_54_6b,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getEmailAppUsageUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getEmailAppUsageVersionsUserCounts",
        name: get_email_app_usage_versions_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getEmailAppUsageVersionsUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getMailboxUsageDetail",
        name: get_mailbox_usage_detail,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getMailboxUsageDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getMailboxUsageMailboxCounts",
        name: get_mailbox_usage_mailbox_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getMailboxUsageMailboxCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getMailboxUsageQuotaStatusMailboxCounts",
        name: get_mailbox_usage_quota_status_mailbox_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getMailboxUsageQuotaStatusMailboxCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getMailboxUsageStorage",
        name: get_mailbox_usage_storage,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getMailboxUsageStorage(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getOffice365ActivationCounts",
        name: get_office_365_activation_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOffice365ActivationCounts()",
        has_body: false
    });
    download!({
        doc: "Invoke function getOffice365ActivationsUserCounts",
        name: get_office_365_activations_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOffice365ActivationsUserCounts()",
        has_body: false
    });
    download!({
        doc: "Invoke function getOffice365ActivationsUserDetail",
        name: get_office_365_activations_user_detail,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOffice365ActivationsUserDetail()",
        has_body: false
    });
    download!({
        doc: "Invoke function getOffice365ActiveUserCounts",
        name: get_office_365_active_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOffice365ActiveUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getOffice365ActiveUserDetail",
        name: get_office_365_active_user_detail_d_389,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOffice365ActiveUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    download!({
        doc: "Invoke function getOffice365ActiveUserDetail",
        name: get_office_365_active_user_detail_6_8ad,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOffice365ActiveUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getOffice365GroupsActivityCounts",
        name: get_office_365_groups_activity_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOffice365GroupsActivityCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getOffice365GroupsActivityDetail",
        name: get_office_365_groups_activity_detail_8_1cc,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOffice365GroupsActivityDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    download!({
        doc: "Invoke function getOffice365GroupsActivityDetail",
        name: get_office_365_groups_activity_detail_3_8f_6,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOffice365GroupsActivityDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getOffice365GroupsActivityFileCounts",
        name: get_office_365_groups_activity_file_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOffice365GroupsActivityFileCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getOffice365GroupsActivityGroupCounts",
        name: get_office_365_groups_activity_group_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOffice365GroupsActivityGroupCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getOffice365GroupsActivityStorage",
        name: get_office_365_groups_activity_storage,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOffice365GroupsActivityStorage(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getOffice365ServicesUserCounts",
        name: get_office_365_services_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOffice365ServicesUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getOneDriveActivityFileCounts",
        name: get_one_drive_activity_file_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOneDriveActivityFileCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getOneDriveActivityUserCounts",
        name: get_one_drive_activity_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOneDriveActivityUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getOneDriveActivityUserDetail",
        name: get_one_drive_activity_user_detail_0_5f_1,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOneDriveActivityUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    download!({
        doc: "Invoke function getOneDriveActivityUserDetail",
        name: get_one_drive_activity_user_detail_c_424,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOneDriveActivityUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getOneDriveUsageAccountCounts",
        name: get_one_drive_usage_account_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOneDriveUsageAccountCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getOneDriveUsageAccountDetail",
        name: get_one_drive_usage_account_detail_e_827,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOneDriveUsageAccountDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    download!({
        doc: "Invoke function getOneDriveUsageAccountDetail",
        name: get_one_drive_usage_account_detail_dd_7f,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOneDriveUsageAccountDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getOneDriveUsageFileCounts",
        name: get_one_drive_usage_file_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOneDriveUsageFileCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getOneDriveUsageStorage",
        name: get_one_drive_usage_storage,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getOneDriveUsageStorage(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSharePointActivityFileCounts",
        name: get_share_point_activity_file_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSharePointActivityFileCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSharePointActivityPages",
        name: get_share_point_activity_pages,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSharePointActivityPages(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSharePointActivityUserCounts",
        name: get_share_point_activity_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSharePointActivityUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSharePointActivityUserDetail",
        name: get_share_point_activity_user_detail_f_3be,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSharePointActivityUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSharePointActivityUserDetail",
        name: get_share_point_activity_user_detail_b_778,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSharePointActivityUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSharePointSiteUsageDetail",
        name: get_share_point_site_usage_detail_d_2_7a,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSharePointSiteUsageDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSharePointSiteUsageDetail",
        name: get_share_point_site_usage_detail_20_4b,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSharePointSiteUsageDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSharePointSiteUsageFileCounts",
        name: get_share_point_site_usage_file_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSharePointSiteUsageFileCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSharePointSiteUsagePages",
        name: get_share_point_site_usage_pages,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSharePointSiteUsagePages(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSharePointSiteUsageSiteCounts",
        name: get_share_point_site_usage_site_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSharePointSiteUsageSiteCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSharePointSiteUsageStorage",
        name: get_share_point_site_usage_storage,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSharePointSiteUsageStorage(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSkypeForBusinessActivityCounts",
        name: get_skype_for_business_activity_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessActivityCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSkypeForBusinessActivityUserCounts",
        name: get_skype_for_business_activity_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessActivityUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSkypeForBusinessActivityUserDetail",
        name: get_skype_for_business_activity_user_detail_e_4c_9,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessActivityUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSkypeForBusinessActivityUserDetail",
        name: get_skype_for_business_activity_user_detail_74_4e,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessActivityUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSkypeForBusinessDeviceUsageDistributionUserCounts",
        name: get_skype_for_business_device_usage_distribution_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessDeviceUsageDistributionUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSkypeForBusinessDeviceUsageUserCounts",
        name: get_skype_for_business_device_usage_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessDeviceUsageUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSkypeForBusinessDeviceUsageUserDetail",
        name: get_skype_for_business_device_usage_user_detail_a_692,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessDeviceUsageUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSkypeForBusinessDeviceUsageUserDetail",
        name: get_skype_for_business_device_usage_user_detail_e_753,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessDeviceUsageUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSkypeForBusinessOrganizerActivityCounts",
        name: get_skype_for_business_organizer_activity_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessOrganizerActivityCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSkypeForBusinessOrganizerActivityMinuteCounts",
        name: get_skype_for_business_organizer_activity_minute_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessOrganizerActivityMinuteCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSkypeForBusinessOrganizerActivityUserCounts",
        name: get_skype_for_business_organizer_activity_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessOrganizerActivityUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSkypeForBusinessParticipantActivityCounts",
        name: get_skype_for_business_participant_activity_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessParticipantActivityCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSkypeForBusinessParticipantActivityMinuteCounts",
        name: get_skype_for_business_participant_activity_minute_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessParticipantActivityMinuteCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSkypeForBusinessParticipantActivityUserCounts",
        name: get_skype_for_business_participant_activity_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessParticipantActivityUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSkypeForBusinessPeerToPeerActivityCounts",
        name: get_skype_for_business_peer_to_peer_activity_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessPeerToPeerActivityCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSkypeForBusinessPeerToPeerActivityMinuteCounts",
        name: get_skype_for_business_peer_to_peer_activity_minute_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessPeerToPeerActivityMinuteCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getSkypeForBusinessPeerToPeerActivityUserCounts",
        name: get_skype_for_business_peer_to_peer_activity_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessPeerToPeerActivityUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getTeamsDeviceUsageDistributionUserCounts",
        name: get_teams_device_usage_distribution_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getTeamsDeviceUsageDistributionUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getTeamsDeviceUsageUserCounts",
        name: get_teams_device_usage_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getTeamsDeviceUsageUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getTeamsDeviceUsageUserDetail",
        name: get_teams_device_usage_user_detail_7148,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getTeamsDeviceUsageUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    download!({
        doc: "Invoke function getTeamsDeviceUsageUserDetail",
        name: get_teams_device_usage_user_detail_7565,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getTeamsDeviceUsageUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getTeamsUserActivityCounts",
        name: get_teams_user_activity_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getTeamsUserActivityCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getTeamsUserActivityUserCounts",
        name: get_teams_user_activity_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getTeamsUserActivityUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getTeamsUserActivityUserDetail",
        name: get_teams_user_activity_user_detail_a_3f_1,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getTeamsUserActivityUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    download!({
        doc: "Invoke function getTeamsUserActivityUserDetail",
        name: get_teams_user_activity_user_detail_eb_13,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getTeamsUserActivityUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getYammerActivityCounts",
        name: get_yammer_activity_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getYammerActivityCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getYammerActivityUserCounts",
        name: get_yammer_activity_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getYammerActivityUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getYammerActivityUserDetail",
        name: get_yammer_activity_user_detail_ac_30,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getYammerActivityUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    download!({
        doc: "Invoke function getYammerActivityUserDetail",
        name: get_yammer_activity_user_detail_1_5a_5,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getYammerActivityUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getYammerDeviceUsageDistributionUserCounts",
        name: get_yammer_device_usage_distribution_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getYammerDeviceUsageDistributionUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getYammerDeviceUsageUserCounts",
        name: get_yammer_device_usage_user_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getYammerDeviceUsageUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getYammerDeviceUsageUserDetail",
        name: get_yammer_device_usage_user_detail_d_0ac,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getYammerDeviceUsageUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    download!({
        doc: "Invoke function getYammerDeviceUsageUserDetail",
        name: get_yammer_device_usage_user_detail_cfad,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getYammerDeviceUsageUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getYammerGroupsActivityCounts",
        name: get_yammer_groups_activity_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getYammerGroupsActivityCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getYammerGroupsActivityDetail",
        name: get_yammer_groups_activity_detail_da_9a,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getYammerGroupsActivityDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    download!({
        doc: "Invoke function getYammerGroupsActivityDetail",
        name: get_yammer_groups_activity_detail_0d_7d,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getYammerGroupsActivityDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function getYammerGroupsActivityGroupCounts",
        name: get_yammer_groups_activity_group_counts,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.getYammerGroupsActivityGroupCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    download!({
        doc: "Invoke function managedDeviceEnrollmentFailureDetails",
        name: managed_device_enrollment_failure_details_02_7e,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.managedDeviceEnrollmentFailureDetails()",
        has_body: false
    });
    download!({
        doc: "Invoke function managedDeviceEnrollmentFailureDetails",
        name: managed_device_enrollment_failure_details_2b_3d,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.managedDeviceEnrollmentFailureDetails(skip={{id}},top={{id2}},filter='{{id3}}',skipToken='{{id4}}')",
        params: [ skip  top  filter  skip_token ],
        has_body: false
    });
    download!({
        doc: "Invoke function managedDeviceEnrollmentTopFailures",
        name: managed_device_enrollment_top_failures_4669,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.managedDeviceEnrollmentTopFailures()",
        has_body: false
    });
    download!({
        doc: "Invoke function managedDeviceEnrollmentTopFailures",
        name: managed_device_enrollment_top_failures_afd_1,
        response: BlockingDownload,
        path: "/reports/microsoft.graph.managedDeviceEnrollmentTopFailures(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
}

impl<'a> ReportsRequest<'a, AsyncHttpClient> {
    async_download!({
        doc: "Invoke function deviceConfigurationDeviceActivity",
        name: device_configuration_device_activity,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.deviceConfigurationDeviceActivity()",
        has_body: false
    });
    async_download!({
        doc: "Invoke function deviceConfigurationUserActivity",
        name: device_configuration_user_activity,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.deviceConfigurationUserActivity()",
        has_body: false
    });
    async_download!({
        doc: "Invoke function getEmailActivityCounts",
        name: get_email_activity_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getEmailActivityCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getEmailActivityUserCounts",
        name: get_email_activity_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getEmailActivityUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getEmailActivityUserDetail",
        name: get_email_activity_user_detail_fe_32,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getEmailActivityUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getEmailActivityUserDetail",
        name: get_email_activity_user_detail_ddb_2,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getEmailActivityUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getEmailAppUsageAppsUserCounts",
        name: get_email_app_usage_apps_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getEmailAppUsageAppsUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getEmailAppUsageUserCounts",
        name: get_email_app_usage_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getEmailAppUsageUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getEmailAppUsageUserDetail",
        name: get_email_app_usage_user_detail_6_2ec,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getEmailAppUsageUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getEmailAppUsageUserDetail",
        name: get_email_app_usage_user_detail_54_6b,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getEmailAppUsageUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getEmailAppUsageVersionsUserCounts",
        name: get_email_app_usage_versions_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getEmailAppUsageVersionsUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getMailboxUsageDetail",
        name: get_mailbox_usage_detail,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getMailboxUsageDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getMailboxUsageMailboxCounts",
        name: get_mailbox_usage_mailbox_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getMailboxUsageMailboxCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getMailboxUsageQuotaStatusMailboxCounts",
        name: get_mailbox_usage_quota_status_mailbox_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getMailboxUsageQuotaStatusMailboxCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getMailboxUsageStorage",
        name: get_mailbox_usage_storage,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getMailboxUsageStorage(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOffice365ActivationCounts",
        name: get_office_365_activation_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOffice365ActivationCounts()",
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOffice365ActivationsUserCounts",
        name: get_office_365_activations_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOffice365ActivationsUserCounts()",
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOffice365ActivationsUserDetail",
        name: get_office_365_activations_user_detail,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOffice365ActivationsUserDetail()",
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOffice365ActiveUserCounts",
        name: get_office_365_active_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOffice365ActiveUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOffice365ActiveUserDetail",
        name: get_office_365_active_user_detail_d_389,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOffice365ActiveUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOffice365ActiveUserDetail",
        name: get_office_365_active_user_detail_6_8ad,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOffice365ActiveUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOffice365GroupsActivityCounts",
        name: get_office_365_groups_activity_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOffice365GroupsActivityCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOffice365GroupsActivityDetail",
        name: get_office_365_groups_activity_detail_8_1cc,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOffice365GroupsActivityDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOffice365GroupsActivityDetail",
        name: get_office_365_groups_activity_detail_3_8f_6,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOffice365GroupsActivityDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOffice365GroupsActivityFileCounts",
        name: get_office_365_groups_activity_file_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOffice365GroupsActivityFileCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOffice365GroupsActivityGroupCounts",
        name: get_office_365_groups_activity_group_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOffice365GroupsActivityGroupCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOffice365GroupsActivityStorage",
        name: get_office_365_groups_activity_storage,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOffice365GroupsActivityStorage(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOffice365ServicesUserCounts",
        name: get_office_365_services_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOffice365ServicesUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOneDriveActivityFileCounts",
        name: get_one_drive_activity_file_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOneDriveActivityFileCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOneDriveActivityUserCounts",
        name: get_one_drive_activity_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOneDriveActivityUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOneDriveActivityUserDetail",
        name: get_one_drive_activity_user_detail_0_5f_1,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOneDriveActivityUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOneDriveActivityUserDetail",
        name: get_one_drive_activity_user_detail_c_424,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOneDriveActivityUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOneDriveUsageAccountCounts",
        name: get_one_drive_usage_account_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOneDriveUsageAccountCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOneDriveUsageAccountDetail",
        name: get_one_drive_usage_account_detail_e_827,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOneDriveUsageAccountDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOneDriveUsageAccountDetail",
        name: get_one_drive_usage_account_detail_dd_7f,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOneDriveUsageAccountDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOneDriveUsageFileCounts",
        name: get_one_drive_usage_file_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOneDriveUsageFileCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getOneDriveUsageStorage",
        name: get_one_drive_usage_storage,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getOneDriveUsageStorage(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSharePointActivityFileCounts",
        name: get_share_point_activity_file_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSharePointActivityFileCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSharePointActivityPages",
        name: get_share_point_activity_pages,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSharePointActivityPages(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSharePointActivityUserCounts",
        name: get_share_point_activity_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSharePointActivityUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSharePointActivityUserDetail",
        name: get_share_point_activity_user_detail_f_3be,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSharePointActivityUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSharePointActivityUserDetail",
        name: get_share_point_activity_user_detail_b_778,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSharePointActivityUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSharePointSiteUsageDetail",
        name: get_share_point_site_usage_detail_d_2_7a,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSharePointSiteUsageDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSharePointSiteUsageDetail",
        name: get_share_point_site_usage_detail_20_4b,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSharePointSiteUsageDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSharePointSiteUsageFileCounts",
        name: get_share_point_site_usage_file_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSharePointSiteUsageFileCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSharePointSiteUsagePages",
        name: get_share_point_site_usage_pages,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSharePointSiteUsagePages(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSharePointSiteUsageSiteCounts",
        name: get_share_point_site_usage_site_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSharePointSiteUsageSiteCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSharePointSiteUsageStorage",
        name: get_share_point_site_usage_storage,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSharePointSiteUsageStorage(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSkypeForBusinessActivityCounts",
        name: get_skype_for_business_activity_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessActivityCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSkypeForBusinessActivityUserCounts",
        name: get_skype_for_business_activity_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessActivityUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSkypeForBusinessActivityUserDetail",
        name: get_skype_for_business_activity_user_detail_e_4c_9,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessActivityUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSkypeForBusinessActivityUserDetail",
        name: get_skype_for_business_activity_user_detail_74_4e,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessActivityUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSkypeForBusinessDeviceUsageDistributionUserCounts",
        name: get_skype_for_business_device_usage_distribution_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessDeviceUsageDistributionUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSkypeForBusinessDeviceUsageUserCounts",
        name: get_skype_for_business_device_usage_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessDeviceUsageUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSkypeForBusinessDeviceUsageUserDetail",
        name: get_skype_for_business_device_usage_user_detail_a_692,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessDeviceUsageUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSkypeForBusinessDeviceUsageUserDetail",
        name: get_skype_for_business_device_usage_user_detail_e_753,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessDeviceUsageUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSkypeForBusinessOrganizerActivityCounts",
        name: get_skype_for_business_organizer_activity_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessOrganizerActivityCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSkypeForBusinessOrganizerActivityMinuteCounts",
        name: get_skype_for_business_organizer_activity_minute_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessOrganizerActivityMinuteCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSkypeForBusinessOrganizerActivityUserCounts",
        name: get_skype_for_business_organizer_activity_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessOrganizerActivityUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSkypeForBusinessParticipantActivityCounts",
        name: get_skype_for_business_participant_activity_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessParticipantActivityCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSkypeForBusinessParticipantActivityMinuteCounts",
        name: get_skype_for_business_participant_activity_minute_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessParticipantActivityMinuteCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSkypeForBusinessParticipantActivityUserCounts",
        name: get_skype_for_business_participant_activity_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessParticipantActivityUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSkypeForBusinessPeerToPeerActivityCounts",
        name: get_skype_for_business_peer_to_peer_activity_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessPeerToPeerActivityCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSkypeForBusinessPeerToPeerActivityMinuteCounts",
        name: get_skype_for_business_peer_to_peer_activity_minute_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessPeerToPeerActivityMinuteCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getSkypeForBusinessPeerToPeerActivityUserCounts",
        name: get_skype_for_business_peer_to_peer_activity_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getSkypeForBusinessPeerToPeerActivityUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getTeamsDeviceUsageDistributionUserCounts",
        name: get_teams_device_usage_distribution_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getTeamsDeviceUsageDistributionUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getTeamsDeviceUsageUserCounts",
        name: get_teams_device_usage_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getTeamsDeviceUsageUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getTeamsDeviceUsageUserDetail",
        name: get_teams_device_usage_user_detail_7148,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getTeamsDeviceUsageUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getTeamsDeviceUsageUserDetail",
        name: get_teams_device_usage_user_detail_7565,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getTeamsDeviceUsageUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getTeamsUserActivityCounts",
        name: get_teams_user_activity_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getTeamsUserActivityCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getTeamsUserActivityUserCounts",
        name: get_teams_user_activity_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getTeamsUserActivityUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getTeamsUserActivityUserDetail",
        name: get_teams_user_activity_user_detail_a_3f_1,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getTeamsUserActivityUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getTeamsUserActivityUserDetail",
        name: get_teams_user_activity_user_detail_eb_13,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getTeamsUserActivityUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getYammerActivityCounts",
        name: get_yammer_activity_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getYammerActivityCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getYammerActivityUserCounts",
        name: get_yammer_activity_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getYammerActivityUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getYammerActivityUserDetail",
        name: get_yammer_activity_user_detail_ac_30,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getYammerActivityUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getYammerActivityUserDetail",
        name: get_yammer_activity_user_detail_1_5a_5,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getYammerActivityUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getYammerDeviceUsageDistributionUserCounts",
        name: get_yammer_device_usage_distribution_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getYammerDeviceUsageDistributionUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getYammerDeviceUsageUserCounts",
        name: get_yammer_device_usage_user_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getYammerDeviceUsageUserCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getYammerDeviceUsageUserDetail",
        name: get_yammer_device_usage_user_detail_d_0ac,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getYammerDeviceUsageUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getYammerDeviceUsageUserDetail",
        name: get_yammer_device_usage_user_detail_cfad,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getYammerDeviceUsageUserDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getYammerGroupsActivityCounts",
        name: get_yammer_groups_activity_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getYammerGroupsActivityCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getYammerGroupsActivityDetail",
        name: get_yammer_groups_activity_detail_da_9a,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getYammerGroupsActivityDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getYammerGroupsActivityDetail",
        name: get_yammer_groups_activity_detail_0d_7d,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getYammerGroupsActivityDetail(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function getYammerGroupsActivityGroupCounts",
        name: get_yammer_groups_activity_group_counts,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.getYammerGroupsActivityGroupCounts(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function managedDeviceEnrollmentFailureDetails",
        name: managed_device_enrollment_failure_details_02_7e,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.managedDeviceEnrollmentFailureDetails()",
        has_body: false
    });
    async_download!({
        doc: "Invoke function managedDeviceEnrollmentFailureDetails",
        name: managed_device_enrollment_failure_details_2b_3d,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.managedDeviceEnrollmentFailureDetails(skip={{id}},top={{id2}},filter='{{id3}}',skipToken='{{id4}}')",
        params: [ skip  top  filter  skip_token ],
        has_body: false
    });
    async_download!({
        doc: "Invoke function managedDeviceEnrollmentTopFailures",
        name: managed_device_enrollment_top_failures_4669,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.managedDeviceEnrollmentTopFailures()",
        has_body: false
    });
    async_download!({
        doc: "Invoke function managedDeviceEnrollmentTopFailures",
        name: managed_device_enrollment_top_failures_afd_1,
        response: AsyncDownload,
        path: "/reports/microsoft.graph.managedDeviceEnrollmentTopFailures(period='{{id}}')",
        params: [ period ],
        has_body: false
    });
}
