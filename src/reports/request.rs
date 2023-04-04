// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(ReportsApiClient, ResourceIdentity::Reports);

impl ReportsApiClient {
    get!(
        doc: "Get reports",
        name: get_report_root,
        path: "/reports"
    );
    patch!(
        doc: "Update reports",
        name: update_report_root,
        path: "/reports",
        body: true
    );
    post!(
        doc: "Create new navigation property to dailyPrintUsageByPrinter for reports",
        name: create_daily_print_usage_by_printer,
        path: "/reports/dailyPrintUsageByPrinter",
        body: true
    );
    get!(
        doc: "List dailyPrintUsageByPrinter",
        name: list_daily_print_usage_by_printer,
        path: "/reports/dailyPrintUsageByPrinter"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_daily_print_usage_by_printer_count,
        path: "/reports/dailyPrintUsageByPrinter/$count"
    );
    delete!(
        doc: "Delete navigation property dailyPrintUsageByPrinter for reports",
        name: delete_daily_print_usage_by_printer,
        path: "/reports/dailyPrintUsageByPrinter/{{id}}",
        params: print_usage_by_printer_id
    );
    get!(
        doc: "Get dailyPrintUsageByPrinter from reports",
        name: get_daily_print_usage_by_printer,
        path: "/reports/dailyPrintUsageByPrinter/{{id}}",
        params: print_usage_by_printer_id
    );
    patch!(
        doc: "Update the navigation property dailyPrintUsageByPrinter in reports",
        name: update_daily_print_usage_by_printer,
        path: "/reports/dailyPrintUsageByPrinter/{{id}}",
        body: true,
        params: print_usage_by_printer_id
    );
    post!(
        doc: "Create new navigation property to dailyPrintUsageByUser for reports",
        name: create_daily_print_usage_by_user,
        path: "/reports/dailyPrintUsageByUser",
        body: true
    );
    get!(
        doc: "List dailyPrintUsageByUser",
        name: list_daily_print_usage_by_user,
        path: "/reports/dailyPrintUsageByUser"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_daily_print_usage_by_user_count,
        path: "/reports/dailyPrintUsageByUser/$count"
    );
    delete!(
        doc: "Delete navigation property dailyPrintUsageByUser for reports",
        name: delete_daily_print_usage_by_user,
        path: "/reports/dailyPrintUsageByUser/{{id}}",
        params: print_usage_by_user_id
    );
    get!(
        doc: "Get dailyPrintUsageByUser from reports",
        name: get_daily_print_usage_by_user,
        path: "/reports/dailyPrintUsageByUser/{{id}}",
        params: print_usage_by_user_id
    );
    patch!(
        doc: "Update the navigation property dailyPrintUsageByUser in reports",
        name: update_daily_print_usage_by_user,
        path: "/reports/dailyPrintUsageByUser/{{id}}",
        body: true,
        params: print_usage_by_user_id
    );
    get!(
        doc: "Invoke function deviceConfigurationDeviceActivity",
        name: device_configuration_device_activity,
        path: "/reports/microsoft.graph.deviceConfigurationDeviceActivity()"
    );
    get!(
        doc: "Invoke function deviceConfigurationUserActivity",
        name: device_configuration_user_activity,
        path: "/reports/microsoft.graph.deviceConfigurationUserActivity()"
    );
    get!(
        doc: "Invoke function getEmailActivityCounts",
        name: get_email_activity_counts_by_period,
        path: "/reports/microsoft.graph.getEmailActivityCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getEmailActivityUserCounts",
        name: get_email_activity_user_counts_by_period,
        path: "/reports/microsoft.graph.getEmailActivityUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getEmailActivityUserDetail",
        name: get_email_activity_user_detail_by_date,
        path: "/reports/microsoft.graph.getEmailActivityUserDetail(date={{id}})",
        params: date
    );
    get!(
        doc: "Invoke function getEmailActivityUserDetail",
        name: get_email_activity_user_detail_by_period,
        path: "/reports/microsoft.graph.getEmailActivityUserDetail(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getEmailAppUsageAppsUserCounts",
        name: get_email_app_usage_apps_user_counts_by_period,
        path: "/reports/microsoft.graph.getEmailAppUsageAppsUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getEmailAppUsageUserCounts",
        name: get_email_app_usage_user_counts_by_period,
        path: "/reports/microsoft.graph.getEmailAppUsageUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getEmailAppUsageUserDetail",
        name: get_email_app_usage_user_detail_by_date,
        path: "/reports/microsoft.graph.getEmailAppUsageUserDetail(date={{id}})",
        params: date
    );
    get!(
        doc: "Invoke function getEmailAppUsageUserDetail",
        name: get_email_app_usage_user_detail_by_period,
        path: "/reports/microsoft.graph.getEmailAppUsageUserDetail(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getEmailAppUsageVersionsUserCounts",
        name: get_email_app_usage_versions_user_counts_by_period,
        path: "/reports/microsoft.graph.getEmailAppUsageVersionsUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getGroupArchivedPrintJobs",
        name: get_group_archived_print_jobs,
        path: "/reports/microsoft.graph.getGroupArchivedPrintJobs(groupId='{{id}}',startDateTime={{id2}},endDateTime={{id3}})",
        params: group_id, start_date_time, end_date_time
    );
    get!(
        doc: "Invoke function getM365AppPlatformUserCounts",
        name: get_m365_app_platform_user_counts_by_period,
        path: "/reports/microsoft.graph.getM365AppPlatformUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getM365AppUserCounts",
        name: get_m365_app_user_counts_by_period,
        path: "/reports/microsoft.graph.getM365AppUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getM365AppUserDetail",
        name: get_m365_app_user_detail_by_date,
        path: "/reports/microsoft.graph.getM365AppUserDetail(date={{id}})",
        params: date
    );
    get!(
        doc: "Invoke function getM365AppUserDetail",
        name: get_m365_app_user_detail_by_period,
        path: "/reports/microsoft.graph.getM365AppUserDetail(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getMailboxUsageDetail",
        name: get_mailbox_usage_detail_by_period,
        path: "/reports/microsoft.graph.getMailboxUsageDetail(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getMailboxUsageMailboxCounts",
        name: get_mailbox_usage_mailbox_counts_by_period,
        path: "/reports/microsoft.graph.getMailboxUsageMailboxCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getMailboxUsageQuotaStatusMailboxCounts",
        name: get_mailbox_usage_quota_status_mailbox_counts_by_period,
        path: "/reports/microsoft.graph.getMailboxUsageQuotaStatusMailboxCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getMailboxUsageStorage",
        name: get_mailbox_usage_storage_by_period,
        path: "/reports/microsoft.graph.getMailboxUsageStorage(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getOffice365ActivationCounts",
        name: get_office_365_activation_counts,
        path: "/reports/microsoft.graph.getOffice365ActivationCounts()"
    );
    get!(
        doc: "Invoke function getOffice365ActivationsUserCounts",
        name: get_office_365_activations_user_counts,
        path: "/reports/microsoft.graph.getOffice365ActivationsUserCounts()"
    );
    get!(
        doc: "Invoke function getOffice365ActivationsUserDetail",
        name: get_office_365_activations_user_detail,
        path: "/reports/microsoft.graph.getOffice365ActivationsUserDetail()"
    );
    get!(
        doc: "Invoke function getOffice365ActiveUserCounts",
        name: get_office_365_active_user_counts_by_period,
        path: "/reports/microsoft.graph.getOffice365ActiveUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getOffice365ActiveUserDetail",
        name: get_office_365_active_user_detail_by_date,
        path: "/reports/microsoft.graph.getOffice365ActiveUserDetail(date={{id}})",
        params: date
    );
    get!(
        doc: "Invoke function getOffice365ActiveUserDetail",
        name: get_office_365_active_user_detail_by_period,
        path: "/reports/microsoft.graph.getOffice365ActiveUserDetail(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getOffice365GroupsActivityCounts",
        name: get_office_365_groups_activity_counts_by_period,
        path: "/reports/microsoft.graph.getOffice365GroupsActivityCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getOffice365GroupsActivityDetail",
        name: get_office_365_groups_activity_detail_by_date,
        path: "/reports/microsoft.graph.getOffice365GroupsActivityDetail(date={{id}})",
        params: date
    );
    get!(
        doc: "Invoke function getOffice365GroupsActivityDetail",
        name: get_office_365_groups_activity_detail_by_period,
        path: "/reports/microsoft.graph.getOffice365GroupsActivityDetail(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getOffice365GroupsActivityFileCounts",
        name: get_office_365_groups_activity_file_counts_by_period,
        path: "/reports/microsoft.graph.getOffice365GroupsActivityFileCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getOffice365GroupsActivityGroupCounts",
        name: get_office_365_groups_activity_group_counts_by_period,
        path: "/reports/microsoft.graph.getOffice365GroupsActivityGroupCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getOffice365GroupsActivityStorage",
        name: get_office_365_groups_activity_storage_by_period,
        path: "/reports/microsoft.graph.getOffice365GroupsActivityStorage(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getOffice365ServicesUserCounts",
        name: get_office_365_services_user_counts_by_period,
        path: "/reports/microsoft.graph.getOffice365ServicesUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getOneDriveActivityFileCounts",
        name: get_one_drive_activity_file_counts_by_period,
        path: "/reports/microsoft.graph.getOneDriveActivityFileCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getOneDriveActivityUserCounts",
        name: get_one_drive_activity_user_counts_by_period,
        path: "/reports/microsoft.graph.getOneDriveActivityUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getOneDriveActivityUserDetail",
        name: get_one_drive_activity_user_detail_by_date,
        path: "/reports/microsoft.graph.getOneDriveActivityUserDetail(date={{id}})",
        params: date
    );
    get!(
        doc: "Invoke function getOneDriveActivityUserDetail",
        name: get_one_drive_activity_user_detail_by_period,
        path: "/reports/microsoft.graph.getOneDriveActivityUserDetail(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getOneDriveUsageAccountCounts",
        name: get_one_drive_usage_account_counts_by_period,
        path: "/reports/microsoft.graph.getOneDriveUsageAccountCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getOneDriveUsageAccountDetail",
        name: get_one_drive_usage_account_detail_by_date,
        path: "/reports/microsoft.graph.getOneDriveUsageAccountDetail(date={{id}})",
        params: date
    );
    get!(
        doc: "Invoke function getOneDriveUsageAccountDetail",
        name: get_one_drive_usage_account_detail_by_period,
        path: "/reports/microsoft.graph.getOneDriveUsageAccountDetail(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getOneDriveUsageFileCounts",
        name: get_one_drive_usage_file_counts_by_period,
        path: "/reports/microsoft.graph.getOneDriveUsageFileCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getOneDriveUsageStorage",
        name: get_one_drive_usage_storage_by_period,
        path: "/reports/microsoft.graph.getOneDriveUsageStorage(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getPrinterArchivedPrintJobs",
        name: get_printer_archived_print_jobs,
        path: "/reports/microsoft.graph.getPrinterArchivedPrintJobs(printerId='{{id}}',startDateTime={{id2}},endDateTime={{id3}})",
        params: printer_id, start_date_time, end_date_time
    );
    get!(
        doc: "Invoke function getSharePointActivityFileCounts",
        name: get_share_point_activity_file_counts_by_period,
        path: "/reports/microsoft.graph.getSharePointActivityFileCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSharePointActivityPages",
        name: get_share_point_activity_pages_by_period,
        path: "/reports/microsoft.graph.getSharePointActivityPages(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSharePointActivityUserCounts",
        name: get_share_point_activity_user_counts_by_period,
        path: "/reports/microsoft.graph.getSharePointActivityUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSharePointActivityUserDetail",
        name: get_share_point_activity_user_detail_by_date,
        path: "/reports/microsoft.graph.getSharePointActivityUserDetail(date={{id}})",
        params: date
    );
    get!(
        doc: "Invoke function getSharePointActivityUserDetail",
        name: get_share_point_activity_user_detail_by_period,
        path: "/reports/microsoft.graph.getSharePointActivityUserDetail(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSharePointSiteUsageDetail",
        name: get_share_point_site_usage_detail_by_date,
        path: "/reports/microsoft.graph.getSharePointSiteUsageDetail(date={{id}})",
        params: date
    );
    get!(
        doc: "Invoke function getSharePointSiteUsageDetail",
        name: get_share_point_site_usage_detail_by_period,
        path: "/reports/microsoft.graph.getSharePointSiteUsageDetail(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSharePointSiteUsageFileCounts",
        name: get_share_point_site_usage_file_counts_by_period,
        path: "/reports/microsoft.graph.getSharePointSiteUsageFileCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSharePointSiteUsagePages",
        name: get_share_point_site_usage_pages_by_period,
        path: "/reports/microsoft.graph.getSharePointSiteUsagePages(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSharePointSiteUsageSiteCounts",
        name: get_share_point_site_usage_site_counts_by_period,
        path: "/reports/microsoft.graph.getSharePointSiteUsageSiteCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSharePointSiteUsageStorage",
        name: get_share_point_site_usage_storage_by_period,
        path: "/reports/microsoft.graph.getSharePointSiteUsageStorage(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSkypeForBusinessActivityCounts",
        name: get_skype_for_business_activity_counts_by_period,
        path: "/reports/microsoft.graph.getSkypeForBusinessActivityCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSkypeForBusinessActivityUserCounts",
        name: get_skype_for_business_activity_user_counts_by_period,
        path: "/reports/microsoft.graph.getSkypeForBusinessActivityUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSkypeForBusinessActivityUserDetail",
        name: get_skype_for_business_activity_user_detail_by_date,
        path: "/reports/microsoft.graph.getSkypeForBusinessActivityUserDetail(date={{id}})",
        params: date
    );
    get!(
        doc: "Invoke function getSkypeForBusinessActivityUserDetail",
        name: get_skype_for_business_activity_user_detail_by_period,
        path: "/reports/microsoft.graph.getSkypeForBusinessActivityUserDetail(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSkypeForBusinessDeviceUsageDistributionUserCounts",
        name: get_skype_for_business_device_usage_distribution_user_counts_by_period,
        path: "/reports/microsoft.graph.getSkypeForBusinessDeviceUsageDistributionUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSkypeForBusinessDeviceUsageUserCounts",
        name: get_skype_for_business_device_usage_user_counts_by_period,
        path: "/reports/microsoft.graph.getSkypeForBusinessDeviceUsageUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSkypeForBusinessDeviceUsageUserDetail",
        name: get_skype_for_business_device_usage_user_detail_by_date,
        path: "/reports/microsoft.graph.getSkypeForBusinessDeviceUsageUserDetail(date={{id}})",
        params: date
    );
    get!(
        doc: "Invoke function getSkypeForBusinessDeviceUsageUserDetail",
        name: get_skype_for_business_device_usage_user_detail_by_period,
        path: "/reports/microsoft.graph.getSkypeForBusinessDeviceUsageUserDetail(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSkypeForBusinessOrganizerActivityCounts",
        name: get_skype_for_business_organizer_activity_counts_by_period,
        path: "/reports/microsoft.graph.getSkypeForBusinessOrganizerActivityCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSkypeForBusinessOrganizerActivityMinuteCounts",
        name: get_skype_for_business_organizer_activity_minute_counts_by_period,
        path: "/reports/microsoft.graph.getSkypeForBusinessOrganizerActivityMinuteCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSkypeForBusinessOrganizerActivityUserCounts",
        name: get_skype_for_business_organizer_activity_user_counts_by_period,
        path: "/reports/microsoft.graph.getSkypeForBusinessOrganizerActivityUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSkypeForBusinessParticipantActivityCounts",
        name: get_skype_for_business_participant_activity_counts_by_period,
        path: "/reports/microsoft.graph.getSkypeForBusinessParticipantActivityCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSkypeForBusinessParticipantActivityMinuteCounts",
        name: get_skype_for_business_participant_activity_minute_counts_by_period,
        path: "/reports/microsoft.graph.getSkypeForBusinessParticipantActivityMinuteCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSkypeForBusinessParticipantActivityUserCounts",
        name: get_skype_for_business_participant_activity_user_counts_by_period,
        path: "/reports/microsoft.graph.getSkypeForBusinessParticipantActivityUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSkypeForBusinessPeerToPeerActivityCounts",
        name: get_skype_for_business_peer_to_peer_activity_counts_by_period,
        path: "/reports/microsoft.graph.getSkypeForBusinessPeerToPeerActivityCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSkypeForBusinessPeerToPeerActivityMinuteCounts",
        name: get_skype_for_business_peer_to_peer_activity_minute_counts_by_period,
        path: "/reports/microsoft.graph.getSkypeForBusinessPeerToPeerActivityMinuteCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getSkypeForBusinessPeerToPeerActivityUserCounts",
        name: get_skype_for_business_peer_to_peer_activity_user_counts,
        path: "/reports/microsoft.graph.getSkypeForBusinessPeerToPeerActivityUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getTeamsDeviceUsageDistributionUserCounts",
        name: get_teams_device_usage_distribution_user_counts_by_period,
        path: "/reports/microsoft.graph.getTeamsDeviceUsageDistributionUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getTeamsDeviceUsageUserCounts",
        name: get_teams_device_usage_user_counts_by_period,
        path: "/reports/microsoft.graph.getTeamsDeviceUsageUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getTeamsDeviceUsageUserDetail",
        name: get_teams_device_usage_user_detail_by_date,
        path: "/reports/microsoft.graph.getTeamsDeviceUsageUserDetail(date={{id}})",
        params: date
    );
    get!(
        doc: "Invoke function getTeamsDeviceUsageUserDetail",
        name: get_teams_device_usage_user_detail_by_period,
        path: "/reports/microsoft.graph.getTeamsDeviceUsageUserDetail(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getTeamsUserActivityCounts",
        name: get_teams_user_activity_counts_by_period,
        path: "/reports/microsoft.graph.getTeamsUserActivityCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getTeamsUserActivityUserCounts",
        name: get_teams_user_activity_user_counts_by_period,
        path: "/reports/microsoft.graph.getTeamsUserActivityUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getTeamsUserActivityUserDetail",
        name: get_teams_user_activity_user_detail_by_date,
        path: "/reports/microsoft.graph.getTeamsUserActivityUserDetail(date={{id}})",
        params: date
    );
    get!(
        doc: "Invoke function getTeamsUserActivityUserDetail",
        name: get_teams_user_activity_user_detail_by_period,
        path: "/reports/microsoft.graph.getTeamsUserActivityUserDetail(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getUserArchivedPrintJobs",
        name: get_user_archived_print_jobs,
        path: "/reports/microsoft.graph.getUserArchivedPrintJobs(userId='{{id}}',startDateTime={{id2}},endDateTime={{id3}})",
        params: user_id, start_date_time, end_date_time
    );
    get!(
        doc: "Invoke function getYammerActivityCounts",
        name: get_yammer_activity_counts_by_period,
        path: "/reports/microsoft.graph.getYammerActivityCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getYammerActivityUserCounts",
        name: get_yammer_activity_user_counts_by_period,
        path: "/reports/microsoft.graph.getYammerActivityUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getYammerActivityUserDetail",
        name: get_yammer_activity_user_detail_by_date,
        path: "/reports/microsoft.graph.getYammerActivityUserDetail(date={{id}})",
        params: date
    );
    get!(
        doc: "Invoke function getYammerActivityUserDetail",
        name: get_yammer_activity_user_detail_by_period,
        path: "/reports/microsoft.graph.getYammerActivityUserDetail(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getYammerDeviceUsageDistributionUserCounts",
        name: get_yammer_device_usage_distribution_user_counts_by_period,
        path: "/reports/microsoft.graph.getYammerDeviceUsageDistributionUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getYammerDeviceUsageUserCounts",
        name: get_yammer_device_usage_user_counts_by_period,
        path: "/reports/microsoft.graph.getYammerDeviceUsageUserCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getYammerDeviceUsageUserDetail",
        name: get_yammer_device_usage_user_detail_by_date,
        path: "/reports/microsoft.graph.getYammerDeviceUsageUserDetail(date={{id}})",
        params: date
    );
    get!(
        doc: "Invoke function getYammerDeviceUsageUserDetail",
        name: get_yammer_device_usage_user_detail_by_period,
        path: "/reports/microsoft.graph.getYammerDeviceUsageUserDetail(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getYammerGroupsActivityCounts",
        name: get_yammer_groups_activity_counts_by_period,
        path: "/reports/microsoft.graph.getYammerGroupsActivityCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getYammerGroupsActivityDetail",
        name: get_yammer_groups_activity_detail_by_date,
        path: "/reports/microsoft.graph.getYammerGroupsActivityDetail(date={{id}})",
        params: date
    );
    get!(
        doc: "Invoke function getYammerGroupsActivityDetail",
        name: get_yammer_groups_activity_detail_by_period,
        path: "/reports/microsoft.graph.getYammerGroupsActivityDetail(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function getYammerGroupsActivityGroupCounts",
        name: get_yammer_groups_activity_group_counts,
        path: "/reports/microsoft.graph.getYammerGroupsActivityGroupCounts(period='{{id}}')",
        params: period
    );
    get!(
        doc: "Invoke function managedDeviceEnrollmentFailureDetails",
        name: get_managed_device_enrollment_failure_details,
        path: "/reports/microsoft.graph.managedDeviceEnrollmentFailureDetails()"
    );
    get!(
        doc: "Invoke function managedDeviceEnrollmentFailureDetails",
        name: get_managed_device_enrollment_failure_details_by_query,
        path: "/reports/microsoft.graph.managedDeviceEnrollmentFailureDetails(skip={{id}},top={{id2}},filter='{{id3}}',skipToken='{{id4}}')",
        params: skip, top, filter, skip_token
    );
    get!(
        doc: "Invoke function managedDeviceEnrollmentTopFailures",
        name: get_managed_device_enrollment_top_failures,
        path: "/reports/microsoft.graph.managedDeviceEnrollmentTopFailures()"
    );
    get!(
        doc: "Invoke function managedDeviceEnrollmentTopFailures",
        name: get_managed_device_enrollment_top_failures_by_period,
        path: "/reports/microsoft.graph.managedDeviceEnrollmentTopFailures(period='{{id}}')",
        params: period
    );
    post!(
        doc: "Create new navigation property to monthlyPrintUsageByPrinter for reports",
        name: create_monthly_print_usage_by_printer,
        path: "/reports/monthlyPrintUsageByPrinter",
        body: true
    );
    get!(
        doc: "List monthlyPrintUsageByPrinter",
        name: list_monthly_print_usage_by_printer,
        path: "/reports/monthlyPrintUsageByPrinter"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_monthly_print_usage_by_printer_count,
        path: "/reports/monthlyPrintUsageByPrinter/$count"
    );
    delete!(
        doc: "Delete navigation property monthlyPrintUsageByPrinter for reports",
        name: delete_monthly_print_usage_by_printer,
        path: "/reports/monthlyPrintUsageByPrinter/{{id}}",
        params: print_usage_by_printer_id
    );
    get!(
        doc: "Get monthlyPrintUsageByPrinter from reports",
        name: get_monthly_print_usage_by_printer,
        path: "/reports/monthlyPrintUsageByPrinter/{{id}}",
        params: print_usage_by_printer_id
    );
    patch!(
        doc: "Update the navigation property monthlyPrintUsageByPrinter in reports",
        name: update_monthly_print_usage_by_printer,
        path: "/reports/monthlyPrintUsageByPrinter/{{id}}",
        body: true,
        params: print_usage_by_printer_id
    );
    post!(
        doc: "Create new navigation property to monthlyPrintUsageByUser for reports",
        name: create_monthly_print_usage_by_user,
        path: "/reports/monthlyPrintUsageByUser",
        body: true
    );
    get!(
        doc: "List monthlyPrintUsageByUser",
        name: list_monthly_print_usage_by_user,
        path: "/reports/monthlyPrintUsageByUser"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_monthly_print_usage_by_user_count,
        path: "/reports/monthlyPrintUsageByUser/$count"
    );
    delete!(
        doc: "Delete navigation property monthlyPrintUsageByUser for reports",
        name: delete_monthly_print_usage_by_user,
        path: "/reports/monthlyPrintUsageByUser/{{id}}",
        params: print_usage_by_user_id
    );
    get!(
        doc: "Get monthlyPrintUsageByUser from reports",
        name: get_monthly_print_usage_by_user,
        path: "/reports/monthlyPrintUsageByUser/{{id}}",
        params: print_usage_by_user_id
    );
    patch!(
        doc: "Update the navigation property monthlyPrintUsageByUser in reports",
        name: update_monthly_print_usage_by_user,
        path: "/reports/monthlyPrintUsageByUser/{{id}}",
        body: true,
        params: print_usage_by_user_id
    );
    delete!(
        doc: "Delete navigation property security for reports",
        name: delete_security,
        path: "/reports/security"
    );
    get!(
        doc: "Get security from reports",
        name: get_security,
        path: "/reports/security"
    );
    patch!(
        doc: "Update the navigation property security in reports",
        name: update_security,
        path: "/reports/security",
        body: true
    );
    get!(
        doc: "Invoke function getAttackSimulationRepeatOffenders",
        name: get_attack_simulation_repeat_offenders,
        path: "/reports/security/microsoft.graph.getAttackSimulationRepeatOffenders()"
    );
    get!(
        doc: "Invoke function getAttackSimulationSimulationUserCoverage",
        name: get_attack_simulation_simulation_user_coverage,
        path: "/reports/security/microsoft.graph.getAttackSimulationSimulationUserCoverage()"
    );
    get!(
        doc: "Invoke function getAttackSimulationTrainingUserCoverage",
        name: get_attack_simulation_training_user_coverage,
        path: "/reports/security/microsoft.graph.getAttackSimulationTrainingUserCoverage()"
    );
}
