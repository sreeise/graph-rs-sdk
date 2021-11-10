// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

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
        path: "/reports/dailyPrintUsageByPrinter/{{id}}",
        params: [ print_usage_by_printer_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property dailyPrintUsageByPrinter in reports",
        name: update_daily_print_usage_by_printer,
        response: NoContent,
        path: "/reports/dailyPrintUsageByPrinter/{{id}}",
        params: [ print_usage_by_printer_id ],
        has_body: true
    });
    get!({
        doc: "Get dailyPrintUsageByPrinter from reports",
        name: get_daily_print_usage_by_printer,
        response: serde_json::Value,
        path: "/reports/dailyPrintUsageByPrinter/{{id}}",
        params: [ print_usage_by_printer_id ],
        has_body: false
    });
    post!({
        doc: "Create new navigation property to dailyPrintUsageByUser for reports",
        name: create_daily_print_usage_by_user,
        response: serde_json::Value,
        path: "/reports/dailyPrintUsageByUser",
        has_body: true
    });
    get!({
        doc: "Get dailyPrintUsageByUser from reports",
        name: list_daily_print_usage_by_user,
        response: serde_json::Value,
        path: "/reports/dailyPrintUsageByUser",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property dailyPrintUsageByUser for reports",
        name: delete_daily_print_usage_by_user,
        response: NoContent,
        path: "/reports/dailyPrintUsageByUser/{{id}}",
        params: [ print_usage_by_user_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property dailyPrintUsageByUser in reports",
        name: update_daily_print_usage_by_user,
        response: NoContent,
        path: "/reports/dailyPrintUsageByUser/{{id}}",
        params: [ print_usage_by_user_id ],
        has_body: true
    });
    get!({
        doc: "Get dailyPrintUsageByUser from reports",
        name: get_daily_print_usage_by_user,
        response: serde_json::Value,
        path: "/reports/dailyPrintUsageByUser/{{id}}",
        params: [ print_usage_by_user_id ],
        has_body: false
    });
    get!({
        doc: "Invoke function deviceConfigurationDeviceActivity",
        name: device_configuration_device_activity,
        response: serde_json::Value,
        path: "/reports/microsoft.graph.deviceConfigurationDeviceActivity()",
        has_body: false
    });
    get!({
        doc: "Invoke function deviceConfigurationUserActivity",
        name: device_configuration_user_activity,
        response: serde_json::Value,
        path: "/reports/microsoft.graph.deviceConfigurationUserActivity()",
        has_body: false
    });
    get!({
        doc: "Invoke function getEmailActivityCounts",
        name: get_email_activity_counts,
        response: serde_json::Value,
        path: "/reports/getEmailActivityCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getEmailActivityUserCounts",
        name: get_email_activity_user_counts,
        response: serde_json::Value,
        path: "/reports/getEmailActivityUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getEmailActivityUserDetail",
        name: get_email_activity_user_detail_by_date,
        response: serde_json::Value,
        path: "/reports/getEmailActivityUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    get!({
        doc: "Invoke function getEmailActivityUserDetail",
        name: get_email_activity_user_detail_by_period,
        response: serde_json::Value,
        path: "/reports/getEmailActivityUserDetail(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getEmailAppUsageAppsUserCounts",
        name: get_email_app_usage_apps_user_counts,
        response: serde_json::Value,
        path: "/reports/getEmailAppUsageAppsUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getEmailAppUsageUserCounts",
        name: get_email_app_usage_user_counts,
        response: serde_json::Value,
        path: "/reports/getEmailAppUsageUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getEmailAppUsageUserDetail",
        name: get_email_app_usage_user_detail_by_date,
        response: serde_json::Value,
        path: "/reports/getEmailAppUsageUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    get!({
        doc: "Invoke function getEmailAppUsageUserDetail",
        name: get_email_app_usage_user_detail_by_period,
        response: serde_json::Value,
        path: "/reports/getEmailAppUsageUserDetail(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getEmailAppUsageVersionsUserCounts",
        name: get_email_app_usage_versions_user_counts,
        response: serde_json::Value,
        path: "/reports/getEmailAppUsageVersionsUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getGroupArchivedPrintJobs",
        name: get_group_archived_print_jobs,
        response: serde_json::Value,
        path: "/reports/getGroupArchivedPrintJobs(groupId={{id}},startDateTime={{id1}},endDateTime={{id2}})",
        params: [ group_id  start_date_time  end_date_time ],
        has_body: false
    });
    get!({
        doc: "Invoke function getMailboxUsageDetail",
        name: get_mailbox_usage_detail,
        response: serde_json::Value,
        path: "/reports/getMailboxUsageDetail(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getMailboxUsageMailboxCounts",
        name: get_mailbox_usage_mailbox_counts,
        response: serde_json::Value,
        path: "/reports/getMailboxUsageMailboxCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getMailboxUsageQuotaStatusMailboxCounts",
        name: get_mailbox_usage_quota_status_mailbox_counts,
        response: serde_json::Value,
        path: "/reports/getMailboxUsageQuotaStatusMailboxCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getMailboxUsageStorage",
        name: get_mailbox_usage_storage,
        response: serde_json::Value,
        path: "/reports/getMailboxUsageStorage(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getOffice365ActivationCounts",
        name: get_office_365_activation_counts,
        response: serde_json::Value,
        path: "/reports/microsoft.graph.getOffice365ActivationCounts()",
        has_body: false
    });
    get!({
        doc: "Invoke function getOffice365ActivationsUserCounts",
        name: get_office_365_activations_user_counts,
        response: serde_json::Value,
        path: "/reports/microsoft.graph.getOffice365ActivationsUserCounts()",
        has_body: false
    });
    get!({
        doc: "Invoke function getOffice365ActivationsUserDetail",
        name: get_office_365_activations_user_detail,
        response: serde_json::Value,
        path: "/reports/microsoft.graph.getOffice365ActivationsUserDetail()",
        has_body: false
    });
    get!({
        doc: "Invoke function getOffice365ActiveUserCounts",
        name: get_office_365_active_user_counts,
        response: serde_json::Value,
        path: "/reports/getOffice365ActiveUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getOffice365ActiveUserDetail",
        name: get_office_365_active_user_detail_by_date,
        response: serde_json::Value,
        path: "/reports/getOffice365ActiveUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    get!({
        doc: "Invoke function getOffice365ActiveUserDetail",
        name: get_office_365_active_user_detail_by_period,
        response: serde_json::Value,
        path: "/reports/getOffice365ActiveUserDetail(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getOffice365GroupsActivityCounts",
        name: get_office_365_groups_activity_counts,
        response: serde_json::Value,
        path: "/reports/getOffice365GroupsActivityCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getOffice365GroupsActivityDetail",
        name: get_office_365_groups_activity_detail_by_date,
        response: serde_json::Value,
        path: "/reports/getOffice365GroupsActivityDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    get!({
        doc: "Invoke function getOffice365GroupsActivityDetail",
        name: get_office_365_groups_activity_detail_by_period,
        response: serde_json::Value,
        path: "/reports/getOffice365GroupsActivityDetail(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getOffice365GroupsActivityFileCounts",
        name: get_office_365_groups_activity_file_counts,
        response: serde_json::Value,
        path: "/reports/getOffice365GroupsActivityFileCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getOffice365GroupsActivityGroupCounts",
        name: get_office_365_groups_activity_group_counts,
        response: serde_json::Value,
        path: "/reports/getOffice365GroupsActivityGroupCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getOffice365GroupsActivityStorage",
        name: get_office_365_groups_activity_storage,
        response: serde_json::Value,
        path: "/reports/getOffice365GroupsActivityStorage(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getOffice365ServicesUserCounts",
        name: get_office_365_services_user_counts,
        response: serde_json::Value,
        path: "/reports/getOffice365ServicesUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getOneDriveActivityFileCounts",
        name: get_one_drive_activity_file_counts,
        response: serde_json::Value,
        path: "/reports/getOneDriveActivityFileCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getOneDriveActivityUserCounts",
        name: get_one_drive_activity_user_counts,
        response: serde_json::Value,
        path: "/reports/getOneDriveActivityUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getOneDriveActivityUserDetail",
        name: get_one_drive_activity_user_detail_by_date,
        response: serde_json::Value,
        path: "/reports/getOneDriveActivityUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    get!({
        doc: "Invoke function getOneDriveActivityUserDetail",
        name: get_one_drive_activity_user_detail_by_period,
        response: serde_json::Value,
        path: "/reports/getOneDriveActivityUserDetail(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getOneDriveUsageAccountCounts",
        name: get_one_drive_usage_account_counts,
        response: serde_json::Value,
        path: "/reports/getOneDriveUsageAccountCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getOneDriveUsageAccountDetail",
        name: get_one_drive_usage_account_detail_by_date,
        response: serde_json::Value,
        path: "/reports/getOneDriveUsageAccountDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    get!({
        doc: "Invoke function getOneDriveUsageAccountDetail",
        name: get_one_drive_usage_account_detail_by_period,
        response: serde_json::Value,
        path: "/reports/getOneDriveUsageAccountDetail(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getOneDriveUsageFileCounts",
        name: get_one_drive_usage_file_counts,
        response: serde_json::Value,
        path: "/reports/getOneDriveUsageFileCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getOneDriveUsageStorage",
        name: get_one_drive_usage_storage,
        response: serde_json::Value,
        path: "/reports/getOneDriveUsageStorage(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getPrinterArchivedPrintJobs",
        name: get_printer_archived_print_jobs,
        response: serde_json::Value,
        path: "/reports/getPrinterArchivedPrintJobs(printerId={{id}},startDateTime={{id1}},endDateTime={{id2}})",
        params: [ printer_id  start_date_time  end_date_time ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSharePointActivityFileCounts",
        name: get_share_point_activity_file_counts,
        response: serde_json::Value,
        path: "/reports/getSharePointActivityFileCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSharePointActivityPages",
        name: get_share_point_activity_pages,
        response: serde_json::Value,
        path: "/reports/getSharePointActivityPages(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSharePointActivityUserCounts",
        name: get_share_point_activity_user_counts,
        response: serde_json::Value,
        path: "/reports/getSharePointActivityUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSharePointActivityUserDetail",
        name: get_share_point_activity_user_detail_by_date,
        response: serde_json::Value,
        path: "/reports/getSharePointActivityUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSharePointActivityUserDetail",
        name: get_share_point_activity_user_detail_by_period,
        response: serde_json::Value,
        path: "/reports/getSharePointActivityUserDetail(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSharePointSiteUsageDetail",
        name: get_share_point_site_usage_detail_by_date,
        response: serde_json::Value,
        path: "/reports/getSharePointSiteUsageDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSharePointSiteUsageDetail",
        name: get_share_point_site_usage_detail_by_period,
        response: serde_json::Value,
        path: "/reports/getSharePointSiteUsageDetail(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSharePointSiteUsageFileCounts",
        name: get_share_point_site_usage_file_counts,
        response: serde_json::Value,
        path: "/reports/getSharePointSiteUsageFileCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSharePointSiteUsagePages",
        name: get_share_point_site_usage_pages,
        response: serde_json::Value,
        path: "/reports/getSharePointSiteUsagePages(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSharePointSiteUsageSiteCounts",
        name: get_share_point_site_usage_site_counts,
        response: serde_json::Value,
        path: "/reports/getSharePointSiteUsageSiteCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSharePointSiteUsageStorage",
        name: get_share_point_site_usage_storage,
        response: serde_json::Value,
        path: "/reports/getSharePointSiteUsageStorage(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSkypeForBusinessActivityCounts",
        name: get_skype_for_business_activity_counts,
        response: serde_json::Value,
        path: "/reports/getSkypeForBusinessActivityCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSkypeForBusinessActivityUserCounts",
        name: get_skype_for_business_activity_user_counts,
        response: serde_json::Value,
        path: "/reports/getSkypeForBusinessActivityUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSkypeForBusinessActivityUserDetail",
        name: get_skype_for_business_activity_user_detail_by_date,
        response: serde_json::Value,
        path: "/reports/getSkypeForBusinessActivityUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSkypeForBusinessActivityUserDetail",
        name: get_skype_for_business_activity_user_detail_by_period,
        response: serde_json::Value,
        path: "/reports/getSkypeForBusinessActivityUserDetail(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSkypeForBusinessDeviceUsageDistributionUserCounts",
        name: get_skype_for_business_device_usage_distribution_user_counts,
        response: serde_json::Value,
        path: "/reports/getSkypeForBusinessDeviceUsageDistributionUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSkypeForBusinessDeviceUsageUserCounts",
        name: get_skype_for_business_device_usage_user_counts,
        response: serde_json::Value,
        path: "/reports/getSkypeForBusinessDeviceUsageUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSkypeForBusinessDeviceUsageUserDetail",
        name: get_skype_for_business_device_usage_user_detail_by_date,
        response: serde_json::Value,
        path: "/reports/getSkypeForBusinessDeviceUsageUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSkypeForBusinessDeviceUsageUserDetail",
        name: get_skype_for_business_device_usage_user_detail_by_period,
        response: serde_json::Value,
        path: "/reports/getSkypeForBusinessDeviceUsageUserDetail(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSkypeForBusinessOrganizerActivityCounts",
        name: get_skype_for_business_organizer_activity_counts,
        response: serde_json::Value,
        path: "/reports/getSkypeForBusinessOrganizerActivityCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSkypeForBusinessOrganizerActivityMinuteCounts",
        name: get_skype_for_business_organizer_activity_minute_counts,
        response: serde_json::Value,
        path: "/reports/getSkypeForBusinessOrganizerActivityMinuteCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSkypeForBusinessOrganizerActivityUserCounts",
        name: get_skype_for_business_organizer_activity_user_counts,
        response: serde_json::Value,
        path: "/reports/getSkypeForBusinessOrganizerActivityUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSkypeForBusinessParticipantActivityCounts",
        name: get_skype_for_business_participant_activity_counts,
        response: serde_json::Value,
        path: "/reports/getSkypeForBusinessParticipantActivityCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSkypeForBusinessParticipantActivityMinuteCounts",
        name: get_skype_for_business_participant_activity_minute_counts,
        response: serde_json::Value,
        path: "/reports/getSkypeForBusinessParticipantActivityMinuteCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSkypeForBusinessParticipantActivityUserCounts",
        name: get_skype_for_business_participant_activity_user_counts,
        response: serde_json::Value,
        path: "/reports/getSkypeForBusinessParticipantActivityUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSkypeForBusinessPeerToPeerActivityCounts",
        name: get_skype_for_business_peer_to_peer_activity_counts,
        response: serde_json::Value,
        path: "/reports/getSkypeForBusinessPeerToPeerActivityCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSkypeForBusinessPeerToPeerActivityMinuteCounts",
        name: get_skype_for_business_peer_to_peer_activity_minute_counts,
        response: serde_json::Value,
        path: "/reports/getSkypeForBusinessPeerToPeerActivityMinuteCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getSkypeForBusinessPeerToPeerActivityUserCounts",
        name: get_skype_for_business_peer_to_peer_activity_user_counts,
        response: serde_json::Value,
        path: "/reports/getSkypeForBusinessPeerToPeerActivityUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getTeamsDeviceUsageDistributionUserCounts",
        name: get_teams_device_usage_distribution_user_counts,
        response: serde_json::Value,
        path: "/reports/getTeamsDeviceUsageDistributionUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getTeamsDeviceUsageUserCounts",
        name: get_teams_device_usage_user_counts,
        response: serde_json::Value,
        path: "/reports/getTeamsDeviceUsageUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getTeamsDeviceUsageUserDetail",
        name: get_teams_device_usage_user_detail_by_date,
        response: serde_json::Value,
        path: "/reports/getTeamsDeviceUsageUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    get!({
        doc: "Invoke function getTeamsDeviceUsageUserDetail",
        name: get_teams_device_usage_user_detail_by_period,
        response: serde_json::Value,
        path: "/reports/getTeamsDeviceUsageUserDetail(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getTeamsUserActivityCounts",
        name: get_teams_user_activity_counts,
        response: serde_json::Value,
        path: "/reports/getTeamsUserActivityCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getTeamsUserActivityUserCounts",
        name: get_teams_user_activity_user_counts,
        response: serde_json::Value,
        path: "/reports/getTeamsUserActivityUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getTeamsUserActivityUserDetail",
        name: get_teams_user_activity_user_detail_by_date,
        response: serde_json::Value,
        path: "/reports/getTeamsUserActivityUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    get!({
        doc: "Invoke function getTeamsUserActivityUserDetail",
        name: get_teams_user_activity_user_detail_by_period,
        response: serde_json::Value,
        path: "/reports/getTeamsUserActivityUserDetail(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getUserArchivedPrintJobs",
        name: get_user_archived_print_jobs,
        response: serde_json::Value,
        path: "/reports/getUserArchivedPrintJobs(userId={{id}},startDateTime={{id1}},endDateTime={{id2}})",
        params: [ user_id  start_date_time  end_date_time ],
        has_body: false
    });
    get!({
        doc: "Invoke function getYammerActivityCounts",
        name: get_yammer_activity_counts,
        response: serde_json::Value,
        path: "/reports/getYammerActivityCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getYammerActivityUserCounts",
        name: get_yammer_activity_user_counts,
        response: serde_json::Value,
        path: "/reports/getYammerActivityUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getYammerActivityUserDetail",
        name: get_yammer_activity_user_detail_by_date,
        response: serde_json::Value,
        path: "/reports/getYammerActivityUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    get!({
        doc: "Invoke function getYammerActivityUserDetail",
        name: get_yammer_activity_user_detail_by_period,
        response: serde_json::Value,
        path: "/reports/getYammerActivityUserDetail(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getYammerDeviceUsageDistributionUserCounts",
        name: get_yammer_device_usage_distribution_user_counts,
        response: serde_json::Value,
        path: "/reports/getYammerDeviceUsageDistributionUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getYammerDeviceUsageUserCounts",
        name: get_yammer_device_usage_user_counts,
        response: serde_json::Value,
        path: "/reports/getYammerDeviceUsageUserCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getYammerDeviceUsageUserDetail",
        name: get_yammer_device_usage_user_detail_by_date,
        response: serde_json::Value,
        path: "/reports/getYammerDeviceUsageUserDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    get!({
        doc: "Invoke function getYammerDeviceUsageUserDetail",
        name: get_yammer_device_usage_user_detail_by_period,
        response: serde_json::Value,
        path: "/reports/getYammerDeviceUsageUserDetail(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getYammerGroupsActivityCounts",
        name: get_yammer_groups_activity_counts,
        response: serde_json::Value,
        path: "/reports/getYammerGroupsActivityCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getYammerGroupsActivityDetail",
        name: get_yammer_groups_activity_detail_by_date,
        response: serde_json::Value,
        path: "/reports/getYammerGroupsActivityDetail(date={{id}})",
        params: [ date ],
        has_body: false
    });
    get!({
        doc: "Invoke function getYammerGroupsActivityDetail",
        name: get_yammer_groups_activity_detail_by_period,
        response: serde_json::Value,
        path: "/reports/getYammerGroupsActivityDetail(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function getYammerGroupsActivityGroupCounts",
        name: get_yammer_groups_activity_group_counts,
        response: serde_json::Value,
        path: "/reports/getYammerGroupsActivityGroupCounts(period={{id}})",
        params: [ period ],
        has_body: false
    });
    get!({
        doc: "Invoke function managedDeviceEnrollmentFailureDetails",
        name: managed_device_enrollment_failure_details,
        response: serde_json::Value,
        path: "/reports/microsoft.graph.managedDeviceEnrollmentFailureDetails()",
        has_body: false
    });
    get!({
        doc: "Invoke function managedDeviceEnrollmentTopFailures",
        name: managed_device_enrollment_top_failures,
        response: serde_json::Value,
        path: "/reports/microsoft.graph.managedDeviceEnrollmentTopFailures()",
        has_body: false
    });
    get!({
        doc: "Invoke function managedDeviceEnrollmentTopFailures",
        name: managed_device_enrollment_top_failures_by_period,
        response: serde_json::Value,
        path: "/reports/managedDeviceEnrollmentTopFailures(period={{id}})",
        params: [ period ],
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
    patch!({
        doc: "Update the navigation property monthlyPrintUsageByPrinter in reports",
        name: update_monthly_print_usage_by_printer,
        response: NoContent,
        path: "/reports/monthlyPrintUsageByPrinter/{{id}}",
        params: [ print_usage_by_printer_id ],
        has_body: true
    });
    delete!({
        doc: "Delete navigation property monthlyPrintUsageByPrinter for reports",
        name: delete_monthly_print_usage_by_printer,
        response: NoContent,
        path: "/reports/monthlyPrintUsageByPrinter/{{id}}",
        params: [ print_usage_by_printer_id ],
        has_body: false
    });
    get!({
        doc: "Get monthlyPrintUsageByPrinter from reports",
        name: get_monthly_print_usage_by_printer,
        response: serde_json::Value,
        path: "/reports/monthlyPrintUsageByPrinter/{{id}}",
        params: [ print_usage_by_printer_id ],
        has_body: false
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
    get!({
        doc: "Get monthlyPrintUsageByUser from reports",
        name: get_monthly_print_usage_by_user,
        response: serde_json::Value,
        path: "/reports/monthlyPrintUsageByUser/{{id}}",
        params: [ print_usage_by_user_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property monthlyPrintUsageByUser in reports",
        name: update_monthly_print_usage_by_user,
        response: NoContent,
        path: "/reports/monthlyPrintUsageByUser/{{id}}",
        params: [ print_usage_by_user_id ],
        has_body: true
    });
    delete!({
        doc: "Delete navigation property monthlyPrintUsageByUser for reports",
        name: delete_monthly_print_usage_by_user,
        response: NoContent,
        path: "/reports/monthlyPrintUsageByUser/{{id}}",
        params: [ print_usage_by_user_id ],
        has_body: false
    });
}
