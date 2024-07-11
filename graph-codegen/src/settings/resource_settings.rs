use crate::api_types::{WriteConfiguration, WriteConfigurationBuilder};
use crate::settings::{ApiClientLink, ApiClientLinkSettings};
use from_as::*;
use graph_core::resource::ResourceIdentity;
use std::collections::BTreeMap;
use std::io::Write;

#[derive(Builder, Debug, Default, Clone, Eq, PartialEq, Serialize, AsFile, Hash)]
#[builder(
    pattern = "mutable",
    derive(Debug, Eq, PartialEq, Serialize),
    setter(into, strip_option),
    default
)]
pub struct ResourceSettings {
    pub path_name: String,
    pub ri: ResourceIdentity,
    pub imports: Vec<&'static str>,
    pub api_client_links: Vec<ApiClientLinkSettings>,
}

impl ResourceSettings {
    pub fn default(path_name: &str, ri: ResourceIdentity) -> ResourceSettings {
        ResourceSettings {
            path_name: path_name.into(),
            ri,
            imports: vec![],
            api_client_links: vec![],
        }
    }

    pub fn builder(path_name: &str, ri: ResourceIdentity) -> ResourceSettingsBuilder {
        let mut builder = ResourceSettingsBuilder::default();
        builder.ri(ri).path_name(path_name);
        builder
    }

    pub fn new(path_name: &str, ri: ResourceIdentity) -> ResourceSettings {
        match ri {
			ResourceIdentity::Applications => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::service_principals::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("ApplicationsIdApiClient"), vec![
							ApiClientLink::Struct("owners", "ServicePrincipalsOwnersApiClient"),
							ApiClientLink::StructId("owner", "ServicePrincipalsOwnersIdApiClient"),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::ServicePrincipals => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::service_principals::*", "crate::users::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("ServicePrincipalsIdApiClient"), vec![
							ApiClientLink::Struct("owners", "ServicePrincipalsOwnersApiClient"),
							ApiClientLink::StructId("owner", "ServicePrincipalsOwnersIdApiClient"),
							ApiClientLink::Struct("transitive_member_of", "TransitiveMemberOfApiClient"),
							ApiClientLink::Struct("member_of", "MemberOfApiClient"),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::AuthenticationMethodsPolicy => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::authentication_method_configurations::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						None, vec![
							ApiClientLink::Struct("authentication_method_configurations", "AuthenticationMethodConfigurationsApiClient"),
							ApiClientLink::StructId("authentication_method_configuration", "AuthenticationMethodConfigurationsIdApiClient"),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::AccessPackages =>
				ResourceSettings {
					path_name: path_name.into(),
					ri,
					imports: vec!["crate::identity_governance::*"],
					api_client_links: vec![
						ApiClientLinkSettings(
							Some("AccessPackagesIdApiClient"),
							vec![
								ApiClientLink::Struct(
									"assignment_policies",
									"AssignmentPoliciesApiClient"
								),
								ApiClientLink::StructId(
									"assignment_policy",
									"AssignmentPoliciesIdApiClient"
								)
							]
						)
					],
				},
			ResourceIdentity::AccessReviews =>
				ResourceSettings {
					path_name: path_name.into(),
					ri,
					imports: vec!["crate::identity_governance::*"],
					api_client_links: vec![
						ApiClientLinkSettings(
							None,
							vec![
								ApiClientLink::Struct("definitions", "AccessReviewsDefinitionsApiClient"),
								ApiClientLink::StructId("definition", "AccessReviewsDefinitionsIdApiClient")
							]
						)
					],
				},
			ResourceIdentity::AccessReviewsDefinitions =>
				ResourceSettings {
					path_name: path_name.into(),
					ri,
					imports: vec!["crate::identity_governance::*"],
					api_client_links: vec![
						ApiClientLinkSettings(
							Some("AccessReviewsDefinitionsIdApiClient"),
							vec![
								ApiClientLink::Struct("instances", "AccessReviewsDefinitionsInstancesApiClient"),
								ApiClientLink::StructId("instance", "AccessReviewsDefinitionsInstancesIdApiClient")
							]
						)
					],
				},
			ResourceIdentity::AccessReviewsDefinitionsInstances =>
				ResourceSettings {
					path_name: path_name.into(),
					ri,
					imports: vec!["crate::identity_governance::*"],
					api_client_links: vec![
						ApiClientLinkSettings(
							Some("AccessReviewsDefinitionsInstancesIdApiClient"),
							vec![
								ApiClientLink::Struct("stages", "AccessReviewsDefinitionsInstancesStagesApiClient"),
								ApiClientLink::StructId("stage", "AccessReviewsDefinitionsInstancesStagesIdApiClient")
							]
						)
					],
				},

			ResourceIdentity::Calls =>
				ResourceSettings {
					path_name: path_name.into(),
					ri,
					imports: vec![],
					api_client_links: vec![],
				},
			ResourceIdentity::CallRecords =>
				ResourceSettings {
					path_name: path_name.to_string(),
					ri,
					imports: vec!["crate::communications::*"],
					api_client_links: vec![
						ApiClientLinkSettings(
							Some("CallRecordsIdApiClient"),
							vec![
								ApiClientLink::Struct("sessions", "CallRecordsSessionsApiClient"),
								ApiClientLink::StructId("session", "CallRecordsSessionsIdApiClient")
							]
						)
					],
				},
			ResourceIdentity::Chats => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::chats::*", "crate::teams::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
							Some("ChatsIdApiClient"),
							vec![
								ApiClientLink::Struct("messages", "ChatsMessagesApiClient"),
								ApiClientLink::StructId("message", "ChatsMessagesIdApiClient"),
								ApiClientLink::Struct("members", "TeamsMembersApiClient"),
								ApiClientLink::Struct("member", "TeamsMembersIdApiClient"),
							]
						)
				]).build().unwrap(),
			ResourceIdentity::ChatsMessages => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::chats::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
							Some("ChatsMessagesIdApiClient"),
							vec![
								ApiClientLink::Struct("replies", "ChatsMessagesRepliesApiClient"),
								ApiClientLink::StructId("reply", "ChatsMessagesRepliesIdApiClient"),
							]
						)
				]).build().unwrap(),
			ResourceIdentity::Channels => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::teams::*", "crate::chats::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
							Some("ChannelsIdApiClient"),
							vec![
								ApiClientLink::Struct("messages", "ChatsMessagesApiClient"),
								ApiClientLink::StructId("message", "ChatsMessagesIdApiClient"),
								ApiClientLink::Struct("shared_with_teams", "SharedWithTeamsApiClient"),
								ApiClientLink::StructId("shared_with_team", "SharedWithTeamsIdApiClient"),
								ApiClientLink::Struct("members", "TeamsMembersApiClient"),
								ApiClientLink::StructId("member", "TeamsMembersIdApiClient"),
							]
						)
				]).build().unwrap(),
			ResourceIdentity::Communications =>
				ResourceSettings {
					path_name: path_name.into(),
					ri,
					imports: vec![
						"crate::communications::{call_records::CallRecordsApiClient, call_records::CallRecordsIdApiClient, calls::CallsApiClient, calls::CallsIdApiClient}"
					],
					api_client_links: vec![
						ApiClientLinkSettings(
							None,
							vec![
								ApiClientLink::Struct("call_records", "CallRecordsApiClient"),
								ApiClientLink::StructId(
									"call_record",
									"CallRecordsIdApiClient"
								),
								ApiClientLink::Struct("calls", "CallsApiClient"),
								ApiClientLink::StructId("call", "CallsIdApiClient")
							]
						)
					],
				},
			ResourceIdentity::ConnectedOrganizations => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::identity_governance::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("ConnectedOrganizationsIdApiClient"), vec![
						ApiClientLink::Struct("external_sponsors", "ConnectedOrganizationsExternalSponsorsApiClient"),
						ApiClientLink::Struct("internal_sponsors", "ConnectedOrganizationsInternalSponsorsApiClient"),
					])
				])
				.build()
				.unwrap(),


			// Device App Management
			ResourceIdentity::ManagedEBooks =>
				ResourceSettings::builder(path_name, ri)
					.imports(vec!["crate::device_app_management::*"])
					.api_client_links(vec![
						ApiClientLinkSettings(
							None, 							vec![
								ApiClientLink::Struct("device_states", "ManagedEBooksDeviceStatesApiClient"),
								ApiClientLink::StructId("device_state", "ManagedEBooksDeviceStatesIdApiClient"),
								ApiClientLink::Struct("user_state_summary", "ManagedEBooksUserStateSummaryApiClient"),
								ApiClientLink::StructId("user_state_summary_id", "ManagedEBooksUserStateSummaryIdApiClient"),
							]
						),
					]).build().unwrap(),
			ResourceIdentity::ManagedAppRegistrations =>
				ResourceSettings::builder(path_name, ri)
					.imports(vec!["crate::device_app_management::*"])
					.api_client_links(vec![
						ApiClientLinkSettings(
							None, 							vec![
								ApiClientLink::Struct("intended_policies", "ManagedAppRegistrationsIntendedPoliciesApiClient"),
								ApiClientLink::StructId("intended_policies_id", "ManagedAppRegistrationsIntendedPoliciesIdApiClient"),
								ApiClientLink::Struct("applied_policies", "ManagedAppRegistrationsAppliedPoliciesApiClient"),
								ApiClientLink::StructId("applied_policies_id", "ManagedAppRegistrationsAppliedPoliciesIdApiClient"),
							]
						),
					]).build().unwrap(),
			ResourceIdentity::DeviceAppManagement =>
				ResourceSettings::builder(path_name, ri)
					.imports(vec!["crate::device_app_management::*"])
					.api_client_links(vec![
						ApiClientLinkSettings(
							None, vec![
								ApiClientLink::Struct("android_managed_app_protections", "AndroidManagedAppProtectionsApiClient"),
								ApiClientLink::StructId("android_managed_app_protection", "AndroidManagedAppProtectionsIdApiClient"),
								ApiClientLink::Struct("default_managed_app_protections", "DefaultManagedAppProtectionsApiClient"),
								ApiClientLink::StructId("default_managed_app_protection", "DefaultManagedAppProtectionsIdApiClient"),
								ApiClientLink::Struct("ios_managed_app_protections", "IosManagedAppProtectionsApiClient"),
								ApiClientLink::StructId("ios_managed_app_protection", "IosManagedAppProtectionsIdApiClient"),
								ApiClientLink::Struct("managed_app_registrations", "ManagedAppRegistrationsApiClient"),
								ApiClientLink::StructId("managed_app_registration", "ManagedAppRegistrationsIdApiClient"),
								ApiClientLink::Struct("managed_app_statuses", "ManagedAppStatusesApiClient"),
								ApiClientLink::StructId("managed_app_statuses_id", "ManagedAppStatusesIdApiClient"),
								ApiClientLink::Struct("mdm_windows_information_protection_policies", "MdmWindowsInformationProtectionPoliciesApiClient"),
								ApiClientLink::StructId("mdm_windows_information_protection_policy", "MdmWindowsInformationProtectionPoliciesIdApiClient"),
								ApiClientLink::Struct("managed_app_policies", "ManagedAppPoliciesApiClient"),
								ApiClientLink::StructId("managed_app_policies_id", "ManagedAppPoliciesIdApiClient"),
								ApiClientLink::Struct("mobile_app_categories", "MobileAppCategoriesApiClient"),
								ApiClientLink::StructId("mobile_app_categories_id", "MobileAppCategoriesIdApiClient"),
								ApiClientLink::Struct("managed_e_books", "ManagedEBooksApiClient"),
								ApiClientLink::StructId("managed_e_book", "ManagedEBooksIdApiClient"),
								ApiClientLink::Struct("mobile_app_configurations", "MobileAppConfigurationsApiClient"),
								ApiClientLink::StructId("mobile_app_configuration", "MobileAppConfigurationsIdApiClient"),
								ApiClientLink::Struct("mobile_apps", "MobileAppsApiClient"),
								ApiClientLink::StructId("mobile_app", "MobileAppsIdApiClient"),
								ApiClientLink::Struct("targeted_managed_app_configurations", "TargetedManagedAppConfigurationsApiClient"),
								ApiClientLink::StructId("targeted_managed_app_configuration", "TargetedManagedAppConfigurationsIdApiClient"),
								ApiClientLink::Struct("vpp_tokens", "VppTokensApiClient"),
								ApiClientLink::StructId("vpp_token", "VppTokensIdApiClient"),
								ApiClientLink::Struct("windows_information_protection_policies", "WindowsInformationProtectionPoliciesApiClient"),
								ApiClientLink::StructId("windows_information_protection_policies_id", "WindowsInformationProtectionPoliciesIdApiClient"),
							]
						),
					]).build().unwrap(),


			// Device Management
			ResourceIdentity::DeviceManagement =>
				ResourceSettings::builder(path_name, ri)
					.imports(vec!["crate::device_management::*"])
					.api_client_links(vec![
						ApiClientLinkSettings(
							None, vec![
								ApiClientLink::Struct("device_configurations", "DeviceConfigurationsApiClient"),
								ApiClientLink::StructId("device_configuration", "DeviceConfigurationsIdApiClient"),
								ApiClientLink::Struct("device_enrollment_configurations", "DeviceEnrollmentConfigurationsApiClient"),
								ApiClientLink::StructId(
									"device_enrollment_configuration",
									"DeviceEnrollmentConfigurationsIdApiClient"
								),
								ApiClientLink::Struct("managed_devices", "DeviceManagementManagedDevicesApiClient"),
								ApiClientLink::StructId("managed_device", "DeviceManagementManagedDevicesIdApiClient"),
								ApiClientLink::Struct("role_definitions", "RoleDefinitionsApiClient"),
								ApiClientLink::StructId("role_definition", "RoleDefinitionsIdApiClient"),
								ApiClientLink::Struct("terms_and_conditions", "TermsAndConditionsApiClient"),
								ApiClientLink::StructId("terms_and_condition", "TermsAndConditionsIdApiClient"),
								ApiClientLink::Struct("troubleshooting_events", "TroubleshootingEventsApiClient"),
								ApiClientLink::StructId("troubleshooting_event", "TroubleshootingEventsIdApiClient"),
								ApiClientLink::Struct("reports", "DeviceManagementReportsApiClient"),
								ApiClientLink::Struct("device_compliance_policy_setting_state_summaries", "DeviceCompliancePolicySettingStateSummariesApiClient"),
								ApiClientLink::StructId("device_compliance_policy_setting_state_summaries_id", "DeviceCompliancePolicySettingStateSummariesIdApiClient"),
								ApiClientLink::StructId("windows_autopilot_device_identities", "WindowsAutopilotDeviceIdentitiesApiClient"),
								ApiClientLink::StructId("windows_autopilot_device_identities_id", "WindowsAutopilotDeviceIdentitiesIdApiClient"),
							]
						),
					]).build().unwrap(),


			ResourceIdentity::Drives =>
				ResourceSettings {
					path_name: path_name.to_string(),
					ri,
					imports: vec![
						"crate::drives::*",
					],
					api_client_links: vec![
						// api_client_link_id!(item_by_path, DrivesItemsPathIdApiClient);
						ApiClientLinkSettings(Some("DrivesIdApiClient"), vec![
							ApiClientLink::Struct("list", "DrivesListApiClient"),
							ApiClientLink::Struct("items", "DrivesItemsApiClient"),
							ApiClientLink::StructId("item", "DrivesItemsIdApiClient"),
							ApiClientLink::StructId("item_by_path", "DrivesItemsPathIdApiClient"),
                            ApiClientLink::Struct("workbook", "WorkbookApiClient"),
                            ApiClientLink::Struct("worksheets", "WorksheetsApiClient"),
                            ApiClientLink::StructId("worksheet", "WorksheetsIdApiClient"),
							ApiClientLink::Struct("last_modified_by_user", "LastModifiedByUserApiClient"),
							ApiClientLink::Struct("created_by_user", "CreatedByUserApiClient"),
						])
					],
				},

			ResourceIdentity::DrivesList =>
				ResourceSettings {
					path_name: path_name.to_string(),
					ri,
					imports: vec![
						"crate::drives::{DrivesListContentTypesApiClient, DrivesListContentTypesIdApiClient, DrivesItemsApiClient, DrivesItemsIdApiClient, CreatedByUserApiClient, LastModifiedByUserApiClient}"
					],
					api_client_links: vec![
						ApiClientLinkSettings(Some("DrivesListApiClient"), vec![
							ApiClientLink::Struct("content_types", "DrivesListContentTypesApiClient"),
							ApiClientLink::StructId("content_type", "DrivesListContentTypesIdApiClient"),
							ApiClientLink::Struct("items", "DrivesItemsApiClient"),
							ApiClientLink::StructId("item", "DrivesItemsIdApiClient"),
							ApiClientLink::Struct("last_modified_by_user", "LastModifiedByUserApiClient"),
							ApiClientLink::Struct("created_by_user", "CreatedByUserApiClient"),
						])
					],
				},
			ResourceIdentity::DrivesItems => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::drives::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("DrivesItemsIdApiClient"), vec![
						ApiClientLink::Struct("last_modified_by_user", "LastModifiedByUserApiClient"),
						ApiClientLink::Struct("created_by_user", "CreatedByUserApiClient"),
						ApiClientLink::Struct("workbook", "WorkbookApiClient"),
						ApiClientLink::Struct("worksheets", "WorksheetsApiClient"),
						ApiClientLink::StructId("worksheet", "WorksheetsIdApiClient"),
					])
				]).build().unwrap(),
			ResourceIdentity::Workbook => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::drives::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("WorkbookApiClient"), vec![
						ApiClientLink::Struct("tables", "WorkbookTablesApiClient"),
						ApiClientLink::StructId("table", "WorkbookTablesIdApiClient"),
						ApiClientLink::Struct("worksheets", "WorksheetsApiClient"),
						ApiClientLink::StructId("worksheet", "WorksheetsIdApiClient"),
						ApiClientLink::Struct("functions", "WorkbookFunctionsApiClient"),
					]),
				])
				.build()
				.unwrap(),
			ResourceIdentity::WorkbookTables => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::drives::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("WorkbookTablesIdApiClient"), vec![
						ApiClientLink::Struct("columns", "WorkbookTablesColumnsApiClient"),
						ApiClientLink::StructId("column", "WorkbookTablesColumnsIdApiClient"),
						ApiClientLink::Struct("rows", "WorkbookTablesRowsApiClient"),
						ApiClientLink::StructId("row", "WorkbookTablesRowsIdApiClient"),
					])
				])
				.build().unwrap(),
			ResourceIdentity::Worksheets => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::drives::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("WorksheetsIdApiClient"), vec![
						ApiClientLink::Struct("tables", "WorkbookTablesApiClient"),
						ApiClientLink::StructId("table", "WorkbookTablesIdApiClient"),
						ApiClientLink::Struct("charts", "WorksheetsChartsApiClient"),
						ApiClientLink::StructId("chart", "WorksheetsChartsIdApiClient"),
					])
				]).build().unwrap(),
			ResourceIdentity::WorksheetsCharts => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::drives::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("WorksheetsChartsIdApiClient"), vec![
						ApiClientLink::Struct("axes", "WorksheetsChartsAxesApiClient"),
						ApiClientLink::Struct("legend", "WorksheetsChartsLegendApiClient"),
						ApiClientLink::Struct("series", "WorksheetsChartsSeriesApiClient"),
						ApiClientLink::Struct("formatting", "WorksheetsChartsFormatApiClient"),
						ApiClientLink::Struct("title", "WorksheetsChartsTitleApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::WorksheetsChartsAxes => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::drives::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("WorksheetsChartsAxesApiClient"), vec![
						ApiClientLink::Struct("category_axis", "WorksheetsChartsAxesApiClient"),
						ApiClientLink::Struct("series_axis", "WorksheetsChartsLegendApiClient"),
						ApiClientLink::Struct("value_axis", "WorksheetsChartsSeriesApiClient"),
						ApiClientLink::Struct("formatting", "WorksheetsChartsFormatApiClient"),
						ApiClientLink::Struct("data_labels", "WorksheetsChartsDataLabelsApiClient"),
						ApiClientLink::Struct("title", "WorksheetsChartsTitleApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::WorksheetsChartsAxesSeriesAxis => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::drives::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("WorksheetsChartsAxesSeriesAxisApiClient"), vec![
						ApiClientLink::Struct("formatting", "WorksheetsChartsFormatApiClient"),
						ApiClientLink::Struct("title", "WorksheetsChartsTitleApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::WorksheetsChartsAxesValueAxis => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::drives::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("WorksheetsChartsAxesValueAxisApiClient"), vec![
						ApiClientLink::Struct("formatting", "WorksheetsChartsFormatApiClient"),
						ApiClientLink::Struct("title", "WorksheetsChartsTitleApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::WorksheetsChartsAxesCategoryAxis => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::drives::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("WorksheetsChartsAxesCategoryAxisApiClient"), vec![
						ApiClientLink::Struct("formatting", "WorksheetsChartsFormatApiClient"),
						ApiClientLink::Struct("title", "WorksheetsChartsTitleApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::WorksheetsChartsTitle => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::drives::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("WorksheetsChartsTitleApiClient"), vec![
						ApiClientLink::Struct("formatting", "WorksheetsChartsFormatApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::WorksheetsChartsDataLabels => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::drives::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("WorksheetsChartsDataLabelsApiClient"), vec![
						ApiClientLink::Struct("formatting", "WorksheetsChartsFormatApiClient"),
					])
				])
				.build()
				.unwrap(),

			ResourceIdentity::Education => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::education::*"])
				.api_client_links(vec![
						ApiClientLinkSettings(None, vec![
							ApiClientLink::Struct("classes", "EducationClassesApiClient"),
							ApiClientLink::StructId("class", "EducationClassesIdApiClient"),
							ApiClientLink::Struct("schools", "EducationSchoolsApiClient"),
							ApiClientLink::StructId("school", "EducationSchoolsIdApiClient"),
							ApiClientLink::Struct("me", "EducationMeApiClient"),
							ApiClientLink::Struct("users", "EducationUsersApiClient"),
							ApiClientLink::StructId("user", "EducationUsersIdApiClient"),
						])
					]).build().unwrap(),
			ResourceIdentity::EducationAssignments => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::education::*"])
				.api_client_links(vec![
						ApiClientLinkSettings(Some("EducationAssignmentsIdApiClient"), vec![
							ApiClientLink::Struct("submissions", "EducationAssignmentsSubmissionsApiClient"),
							ApiClientLink::StructId("submission", "EducationAssignmentsSubmissionsIdApiClient"),
						])
					]).build().unwrap(),
			ResourceIdentity::EducationMe => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::education::*"])
				.api_client_links(vec![
						ApiClientLinkSettings(Some("EducationMeApiClient"), vec![
							ApiClientLink::Struct("assignments", "EducationAssignmentsApiClient"),
							ApiClientLink::StructId("assignment", "EducationAssignmentsIdApiClient"),
						])
					]).build().unwrap(),
			ResourceIdentity::EducationSchools => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::education::*"])
				.api_client_links(vec![
						ApiClientLinkSettings(Some("EducationSchoolsIdApiClient"), vec![
							ApiClientLink::Struct("assignments", "EducationAssignmentsApiClient"),
							ApiClientLink::StructId("assignment", "EducationAssignmentsIdApiClient"),
						])
					]).build().unwrap(),
			ResourceIdentity::EducationUsers => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::education::*"])
				.api_client_links(vec![
						ApiClientLinkSettings(Some("EducationUsersIdApiClient"), vec![
							ApiClientLink::Struct("assignments", "EducationAssignmentsApiClient"),
							ApiClientLink::StructId("assignment", "EducationAssignmentsIdApiClient"),
						])
					]).build().unwrap(),
			ResourceIdentity::EducationClasses => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::education::*"])
				.api_client_links(vec![
						ApiClientLinkSettings(Some("EducationClassesIdApiClient"), vec![
							ApiClientLink::Struct("assignments", "EducationAssignmentsApiClient"),
							ApiClientLink::StructId("assignment", "EducationAssignmentsIdApiClient"),
						])
					]).build().unwrap(),
			ResourceIdentity::EntitlementManagement =>
				ResourceSettings {
					path_name: path_name.to_string(),
					ri,
					imports: vec![
						"crate::identity_governance::*",
					],
					api_client_links: vec![
						ApiClientLinkSettings(
							None,
							vec![
								ApiClientLink::Struct("access_packages", "AccessPackagesApiClient"),
								ApiClientLink::StructId(
									"access_package",
									"AccessPackagesIdApiClient"
								),
								ApiClientLink::Struct(
									"access_package_assignment_approvals",
									"AccessPackageAssignmentApprovalsApiClient"
								),
								ApiClientLink::StructId(
									"access_package_assignment_approval",
									"AccessPackageAssignmentApprovalsIdApiClient",
								),
								ApiClientLink::Struct(
									"assignment_policies",
									"AssignmentPoliciesApiClient"
								),
								ApiClientLink::StructId(
									"assignment_policy",
									"AssignmentPoliciesIdApiClient"
								),
								ApiClientLink::Struct(
									"assignments",
									"EntitlementManagementAssignmentsApiClient"
								),
								ApiClientLink::StructId(
									"assignment",
									"EntitlementManagementAssignmentsIdApiClient"
								),
								ApiClientLink::Struct(
									"catalogs",
									"EntitlementManagementCatalogsApiClient",
								),
								ApiClientLink::StructId(
									"catalog",
									"EntitlementManagementCatalogsIdApiClient",
								),
								ApiClientLink::Struct(
									"assignment_requests",
									"AssignmentRequestsApiClient",
								),
								ApiClientLink::StructId(
									"assignment_request",
									"AssignmentRequestsIdApiClient",
								),
								ApiClientLink::Struct(
									"connected_organizations",
									"ConnectedOrganizationsApiClient",
								),
								ApiClientLink::StructId(
									"connected_organization",
									"ConnectedOrganizationsIdApiClient",
								)
							]
						)
					],
				},
			ResourceIdentity::EntitlementManagementCatalogs =>
				ResourceSettings {
					path_name: path_name.to_string(),
					ri,
					imports: vec!["crate::identity_governance::{AccessPackagesApiClient, AccessPackagesIdApiClient}"],
					api_client_links: vec![
						ApiClientLinkSettings(
							Some("EntitlementManagementCatalogsIdApiClient"),
							vec![
								ApiClientLink::Struct("access_packages", "AccessPackagesApiClient"),
								ApiClientLink::StructId(
									"access_package",
									"AccessPackagesIdApiClient"
								)
							]
						)
					],
				},
			ResourceIdentity::IdentityGovernance =>
				ResourceSettings {
					path_name: path_name.to_string(),
					ri,
					imports: vec![
						"crate::identity_governance::{AccessReviewsApiClient, AccessPackagesApiClient, AccessPackagesIdApiClient, EntitlementManagementApiClient}"
					],
					api_client_links: vec![
						ApiClientLinkSettings(
							None,
							vec![
								ApiClientLink::Struct("access_reviews", "AccessReviewsApiClient"),
								ApiClientLink::Struct("entitlement_management", "EntitlementManagementApiClient")
							]
						)
					],
				},
			ResourceIdentity::Events => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("EventsIdApiClient"),
						vec![
							ApiClientLink::Struct("instances", "EventsInstancesApiClient"),
							ApiClientLink::StructId("instance", "EventsInstancesIdApiClient"),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::CalendarView => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("CalendarViewIdApiClient"),
						vec![
							ApiClientLink::Struct("instances", "EventsInstancesApiClient"),
							ApiClientLink::StructId("instance", "EventsInstancesIdApiClient"),
							ApiClientLink::Struct("attachments", "UsersAttachmentsApiClient"),
							ApiClientLink::StructId("attachment", "UsersAttachmentsIdApiClient"),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::Calendars => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*", "crate::extended_properties::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("CalendarsIdApiClient"),
						vec![
							ApiClientLink::Struct("events", "EventsApiClient"),
							ApiClientLink::StructId("event", "EventsIdApiClient"),
							ApiClientLink::Struct("calendar_views", "CalendarViewApiClient"),
							ApiClientLink::StructId("calendar_view", "CalendarViewIdApiClient"),
							ApiClientLink::Struct("extended_properties", "ExtendedPropertiesApiClient")
						]
					),
				]).build().unwrap(),
			ResourceIdentity::DefaultCalendar => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*", "crate::extended_properties::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("DefaultCalendarApiClient"),
						vec![
							ApiClientLink::Struct("events", "EventsApiClient"),
							ApiClientLink::StructId("event", "EventsIdApiClient"),
							ApiClientLink::Struct("calendar_views", "CalendarViewApiClient"),
							ApiClientLink::StructId("calendar_view", "CalendarViewIdApiClient"),
							ApiClientLink::Struct("extended_properties", "ExtendedPropertiesApiClient"),
						]
					),
				]).build().unwrap(),

			ResourceIdentity::Directory => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::directory::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						None, vec![
							ApiClientLink::Struct("deleted_items", "DeletedItemsApiClient"),
							ApiClientLink::StructId("deleted_item", "DeletedItemsIdApiClient"),
							ApiClientLink::Struct("administrative_units", "AdministrativeUnitsApiClient"),
							ApiClientLink::StructId("administrative_unit", "AdministrativeUnitsIdApiClient"),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::AdministrativeUnits => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::directory::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("AdministrativeUnitsIdApiClient"), vec![
							ApiClientLink::Struct("members", "DirectoryMembersApiClient"),
							ApiClientLink::StructId("member", "DirectoryMembersIdApiClient"),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::DirectoryRoles => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::directory::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("DirectoryRolesIdApiClient"), vec![
							ApiClientLink::Struct("members", "DirectoryMembersApiClient"),
							ApiClientLink::StructId("member", "DirectoryMembersIdApiClient"),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::CalendarGroups => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("CalendarGroupsIdApiClient"),
						vec![
							ApiClientLink::Struct("calendars", "CalendarsApiClient"),
							ApiClientLink::StructId("calendar", "CalendarsIdApiClient"),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::Groups => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::groups::*", "crate::users::*", "crate::sites::*", "crate::planner::*",
							  "crate::group_lifecycle_policies::*", "crate::permission_grants::*"])
				.api_client_links(vec![
						ApiClientLinkSettings(
						Some("GroupsIdApiClient"),
						vec![
							ApiClientLink::Struct("group_lifecycle_policies", "GroupLifecyclePoliciesApiClient"),
							ApiClientLink::StructId("conversation", "ConversationsIdApiClient"),
							ApiClientLink::Struct("conversations", "ConversationsApiClient"),
							ApiClientLink::StructId("thread", "ThreadsIdApiClient"),
							ApiClientLink::Struct("threads", "ThreadsApiClient"),
							ApiClientLink::Struct("onenote", "OnenoteApiClient"),
							ApiClientLink::Struct("member_of", "MemberOfApiClient"),
							ApiClientLink::StructId("member_of_id", "MemberOfIdApiClient"),
							ApiClientLink::Struct("events", "EventsApiClient"),
							ApiClientLink::StructId("event", "EventsIdApiClient"),
							ApiClientLink::Struct("calendar_views", "CalendarViewApiClient"),
							ApiClientLink::StructId("calendar_view", "CalendarViewIdApiClient"),
							ApiClientLink::Struct("default_calendar", "DefaultCalendarApiClient"),
							ApiClientLink::Struct("planner", "PlannerApiClient"),
							ApiClientLink::Struct("sites", "SitesApiClient"),
							ApiClientLink::StructId("site", "SitesIdApiClient"),
							ApiClientLink::Struct("groups_team", "GroupsTeamApiClient"),
							ApiClientLink::Struct("transitive_members", "TransitiveMembersApiClient"),
							ApiClientLink::StructId("transitive_member", "TransitiveMembersIdApiClient"),
							ApiClientLink::Struct("members_with_license_errors", "MembersWithLicenseErrorsApiClient"),
							ApiClientLink::StructId("members_with_license_errors_id", "MembersWithLicenseErrorsIdApiClient"),
							ApiClientLink::Struct("owners", "GroupsOwnersApiClient"),
							ApiClientLink::StructId("owner", "GroupsOwnersIdApiClient"),
							ApiClientLink::Struct("permission_grants", "PermissionGrantsApiClient"),
							ApiClientLink::StructId("permission_grant", "PermissionGrantsIdApiClient"),
						]
					)
				]).build().unwrap(),
			ResourceIdentity::Conversations => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::groups::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("ConversationsIdApiClient"),
						vec![
							ApiClientLink::StructId("thread", "ThreadsIdApiClient"),
							ApiClientLink::Struct("threads", "ThreadsApiClient")
						]
					)
				]).build().unwrap(),
			ResourceIdentity::Threads => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::groups::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("ThreadsIdApiClient"),
						vec![
							ApiClientLink::StructId("post", "ThreadsPostsIdApiClient"),
							ApiClientLink::Struct("posts", "ThreadsPostsApiClient")
						]
					)
				]).build().unwrap(),
			ResourceIdentity::Reports =>
				ResourceSettings {
					path_name: path_name.to_string(),
					ri,
					imports: vec![],
					api_client_links: vec![],
				},
			ResourceIdentity::PrimaryChannel => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::teams::*", "crate::chats::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("PrimaryChannelApiClient"),
						vec![
							ApiClientLink::Struct("shared_with_teams", "SharedWithTeamsApiClient"),
							ApiClientLink::StructId("shared_with_team", "SharedWithTeamsIdApiClient"),
							ApiClientLink::Struct("messages", "ChatsMessagesApiClient"),
							ApiClientLink::StructId("message", "ChatsMessagesIdApiClient"),
							ApiClientLink::Struct("members", "TeamsMembersApiClient"),
							ApiClientLink::Struct("member", "TeamsMembersIdApiClient"),
						]
					)
				]).build().unwrap(),
			ResourceIdentity::Teams => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*", "crate::teams::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("TeamsIdApiClient"),
						vec![
							ApiClientLink::Struct("primary_channel", "PrimaryChannelApiClient"),
							ApiClientLink::Struct("channels", "ChannelsApiClient"),
							ApiClientLink::StructId("channel", "ChannelsIdApiClient"),
							ApiClientLink::Struct("tags", "TeamsTagsApiClient"),
							ApiClientLink::StructId("tag", "TeamsTagsIdApiClient"),
							ApiClientLink::Struct("schedule", "ScheduleApiClient"),
							ApiClientLink::Struct("members", "TeamsMembersApiClient"),
							ApiClientLink::Struct("member", "TeamsMembersIdApiClient"),
						]
					)
				]).build().unwrap(),
			ResourceIdentity::TeamsTags => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::teams::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("TeamsTagsIdApiClient"),
						vec![
							ApiClientLink::Struct("members", "TeamsMembersApiClient"),
							ApiClientLink::Struct("member", "TeamsMembersIdApiClient"),
						]
					)
				]).build().unwrap(),
			ResourceIdentity::GroupsTeam => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*", "crate::teams::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("TeamApiClient"),
						vec![
							ApiClientLink::Struct("primary_channel", "PrimaryChannelApiClient"),
							ApiClientLink::Struct("channels", "ChannelsApiClient"),
							ApiClientLink::StructId("channel", "ChannelsIdApiClient"),
							ApiClientLink::Struct("tags", "TeamsTagsApiClient"),
							ApiClientLink::StructId("tag", "TeamsTagsIdApiClient"),
							ApiClientLink::Struct("schedule", "ScheduleApiClient"),
							ApiClientLink::Struct("members", "TeamsMembersApiClient"),
							ApiClientLink::StructId("member", "TeamsMembersIdApiClient"),
						]
					)
				]).build().unwrap(),
			ResourceIdentity::Todo => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						None,
						vec![
							ApiClientLink::Struct("lists", "TodoListsApiClient"),
							ApiClientLink::StructId("list", "TodoListsIdApiClient"),
						]
					)
				]).build().unwrap(),
			ResourceIdentity::TodoLists => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("TodoListsIdApiClient"),
						vec![
							ApiClientLink::Struct("tasks", "TodoListsTasksApiClient"),
							ApiClientLink::StructId("task", "TodoListsTasksIdApiClient"),
						]
					)
				]).build().unwrap(),
			ResourceIdentity::TodoListsTasks => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("TodoListsIdApiClient"),
						vec![
							ApiClientLink::Struct("tasks", "TodoListsTasksApiClient"),
							ApiClientLink::StructId("task", "TodoListsTasksIdApiClient"),
						]
					)
				]).build().unwrap(),
			ResourceIdentity::JoinedTeams => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*", "crate::teams::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("JoinedTeamsIdApiClient"),
						vec![
							ApiClientLink::Struct("primary_channel", "PrimaryChannelApiClient"),
							ApiClientLink::Struct("channels", "ChannelsApiClient"),
							ApiClientLink::StructId("channel", "ChannelsIdApiClient"),
							ApiClientLink::Struct("tags", "TeamsTagsApiClient"),
							ApiClientLink::StructId("tag", "TeamsTagsIdApiClient"),
							ApiClientLink::Struct("schedule", "ScheduleApiClient"),
							ApiClientLink::Struct("members", "TeamsMembersApiClient"),
							ApiClientLink::Struct("member", "TeamsMembersIdApiClient"),
						]
					)
				]).build().unwrap(),
			ResourceIdentity::MailFolders => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*", "crate::extended_properties::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("MailFoldersIdApiClient"), vec![
						ApiClientLink::Struct("child_folders", "ChildFoldersApiClient"),
						ApiClientLink::StructId("child_folder", "ChildFoldersIdApiClient"),
						ApiClientLink::Struct("extended_properties", "ExtendedPropertiesApiClient"),
						ApiClientLink::Struct("messages", "UsersMessagesApiClient"),
						ApiClientLink::StructId("messages_id", "UsersMessagesIdApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::ChildFolders => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*", "crate::extended_properties::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("ChildFoldersIdApiClient"), vec![
						ApiClientLink::Struct("messages", "UsersMessagesApiClient"),
						ApiClientLink::StructId("messages_id", "UsersMessagesIdApiClient"),
						ApiClientLink::Struct("extended_properties", "ExtendedPropertiesApiClient"),
						ApiClientLink::StructId("child_folder", "ChildFoldersIdApiClient"),
					])
				]).build()
				.unwrap(),
			ResourceIdentity::ContactFolders => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*", "crate::extended_properties::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("ContactFoldersIdApiClient"), vec![
						ApiClientLink::Struct("contacts", "ContactsApiClient"),
						ApiClientLink::StructId("contact", "ContactsIdApiClient"),
						ApiClientLink::Struct("child_folders", "ChildFoldersApiClient"),
						ApiClientLink::StructId("child_folder", "ChildFoldersIdApiClient"),
						ApiClientLink::Struct("extended_properties", "ExtendedPropertiesApiClient"),
					])
				]).build()
				.unwrap(),
			ResourceIdentity::Planner => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::planner::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("PlannerApiClient"), vec![
						ApiClientLink::Struct("tasks", "PlannerTasksApiClient"),
						ApiClientLink::StructId("task", "PlannerTasksIdApiClient"),
						ApiClientLink::Struct("buckets", "BucketsApiClient"),
						ApiClientLink::StructId("bucket", "BucketsIdApiClient"),
						ApiClientLink::Struct("plans", "PlansApiClient"),
						ApiClientLink::StructId("plan", "PlansIdApiClient"),
					])
				]).build()
				.unwrap(),
			ResourceIdentity::Buckets => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::planner::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("BucketsIdApiClient"), vec![
						ApiClientLink::Struct("tasks", "PlannerTasksApiClient"),
						ApiClientLink::StructId("task", "PlannerTasksIdApiClient"),
					])
				]).build()
				.unwrap(),
			ResourceIdentity::Plans => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::planner::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("PlansIdApiClient"), vec![
						ApiClientLink::Struct("tasks", "PlannerTasksApiClient"),
						ApiClientLink::StructId("task", "PlannerTasksIdApiClient"),
						ApiClientLink::Struct("plans", "PlansApiClient"),
						ApiClientLink::StructId("plan", "PlansIdApiClient"),
					])
				]).build()
				.unwrap(),
			ResourceIdentity::Onenote => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("OnenoteApiClient"), vec![
						ApiClientLink::Struct("pages", "OnenotePagesApiClient"),
						ApiClientLink::StructId("page", "OnenotePagesIdApiClient"),
						ApiClientLink::Struct("sections", "OnenoteSectionsApiClient"),
						ApiClientLink::StructId("section", "OnenoteSectionsIdApiClient"),
						ApiClientLink::Struct("section_groups", "OnenoteSectionGroupsApiClient"),
						ApiClientLink::StructId("section_group", "OnenoteSectionGroupsIdApiClient"),
						ApiClientLink::Struct("notebooks", "OnenoteNotebooksApiClient"),
						ApiClientLink::StructId("notebook", "OnenoteNotebooksIdApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::OnenoteNotebooks => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("OnenoteNotebooksIdApiClient"), vec![
						ApiClientLink::Struct("sections", "OnenoteSectionsApiClient"),
						ApiClientLink::StructId("section", "OnenoteSectionsIdApiClient"),
						ApiClientLink::Struct("section_groups", "OnenoteSectionGroupsApiClient"),
						ApiClientLink::StructId("section_group", "OnenoteSectionGroupsIdApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::OnenoteSections => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("OnenoteSectionsIdApiClient"), vec![
						ApiClientLink::Struct("sections", "OnenoteSectionsApiClient"),
						ApiClientLink::StructId("section", "OnenoteSectionsIdApiClient"),
						ApiClientLink::Struct("pages", "OnenotePagesApiClient"),
						ApiClientLink::StructId("page", "OnenotePagesIdApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::OnenoteSectionGroups => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("OnenoteSectionGroupsIdApiClient"), vec![
						ApiClientLink::Struct("sections", "OnenoteSectionsApiClient"),
						ApiClientLink::StructId("section", "OnenoteSectionsIdApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::OnenotePages => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*"])
				.build()
				.unwrap(),
			ResourceIdentity::Teamwork => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::teamwork::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("TeamworkApiClient"), vec![
						ApiClientLink::Struct("deleted_teams", "DeletedTeamsApiClient"),
						ApiClientLink::StructId("deleted_team", "DeletedTeamsIdApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::DeletedTeams => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("DeletedTeamsIdApiClient"), vec![
						ApiClientLink::Struct("channels", "ChannelsApiClient"),
						ApiClientLink::StructId("channel", "ChannelsIdApiClient"),
					])
				])
				.build()
				.unwrap(),


			// Sites
			ResourceIdentity::SitesItems => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::sites::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("SitesItemsIdApiClient"), vec![
						ApiClientLink::Struct("versions", "SitesItemsVersionsApiClient"),
						ApiClientLink::StructId("version", "SitesItemsVersionsIdApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::SitesLists => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::sites::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("SitesListsIdApiClient"), vec![
						ApiClientLink::Struct("content_types", "SitesContentTypesApiClient"),
						ApiClientLink::StructId("content_type", "SitesContentTypesIdApiClient"),
						ApiClientLink::Struct("items", "SitesItemsApiClient"),
						ApiClientLink::StructId("item", "SitesItemsIdApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::TermStores => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::sites::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("TermStoresIdApiClient"), vec![
						ApiClientLink::Struct("sets", "TermStoreSetsApiClient"),
						ApiClientLink::StructId("set", "TermStoreSetsIdApiClient"),
						ApiClientLink::Struct("groups", "TermStoreGroupsApiClient"),
						ApiClientLink::StructId("group", "TermStoreGroupsIdApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::TermStore => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::sites::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("TermStoreApiClient"), vec![
						ApiClientLink::Struct("sets", "TermStoreSetsApiClient"),
						ApiClientLink::StructId("set", "TermStoreSetsIdApiClient"),
						ApiClientLink::Struct("groups", "TermStoreGroupsApiClient"),
						ApiClientLink::StructId("group", "TermStoreGroupsIdApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::TermStoreSets => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::sites::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("TermStoreSetsIdApiClient"), vec![
						ApiClientLink::Struct("children", "TermStoreSetsChildrenApiClient"),
						ApiClientLink::StructId("children_id", "TermStoreSetsChildrenIdApiClient"),
						ApiClientLink::Struct("parent_group", "TermStoreSetsParentGroupApiClient"),
						ApiClientLink::Struct("terms", "TermStoreSetsTermsApiClient"),
						ApiClientLink::StructId("term", "TermStoreSetsTermsIdApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::TermStoreSetsParentGroup => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::sites::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("TermStoreSetsParentGroupApiClient"), vec![
						ApiClientLink::Struct("sets", "TermStoreSetsApiClient"),
						ApiClientLink::StructId("set", "TermStoreSetsIdApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::TermStoreGroups => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::sites::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("TTermStoreGroupsIdApiClient"), vec![
						ApiClientLink::Struct("sets", "TermStoreSetsApiClient"),
						ApiClientLink::StructId("set", "TermStoreSetsIdApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::Sites => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*", "crate::sites::*", "crate::default_drive::*", "crate::drives::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("SitesIdApiClient"), vec![
						ApiClientLink::Struct("lists", "SitesListsApiClient"),
						ApiClientLink::StructId("list", "SitesListsIdApiClient"),
						ApiClientLink::Struct("onenote", "OnenoteApiClient"),
						ApiClientLink::Struct("drive", "DefaultDriveApiClient"),
						ApiClientLink::Struct("term_store", "TermStoreApiClient"),
						ApiClientLink::Struct("term_stores", "TermStoresApiClient"),
						ApiClientLink::StructId("term_stores_id", "TermStoresIdApiClient"),
						ApiClientLink::Struct("content_types", "SitesContentTypesApiClient"),
						ApiClientLink::StructId("content_type", "SitesContentTypesIdApiClient"),
						ApiClientLink::Struct("items", "SitesItemsApiClient"),
					])
				])
				.build()
				.unwrap(),



			ResourceIdentity::UsersMessages => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("UsersMessagesIdApiClient"), vec![
						ApiClientLink::Struct("attachments", "UsersAttachmentsApiClient"),
						ApiClientLink::StructId("attachment", "UsersAttachmentsIdApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::Users => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*", "crate::chats::*", "crate::agreement_acceptances::*",
							  "crate::planner::*", "crate::oauth2_permission_grants::*", "crate::teams::*"])
				.api_client_links(get_users_api_client_links(ri))
				.build()
				.unwrap(),
			ResourceIdentity::Me => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*", "crate::chats::*", "crate::agreement_acceptances::*",
							  "crate::planner::*", "crate::oauth2_permission_grants::*", "crate::teams::*"])
				.api_client_links(get_users_api_client_links(ri))
				.build()
				.unwrap(),
			ResourceIdentity::Solutions => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::solutions::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("SolutionsApiClient"), 
					vec![
						ApiClientLink::Struct("booking_businesses", "BookingBusinessesApiClient"),
						ApiClientLink::StructId("booking_business", "BookingBusinessesIdApiClient"),
						ApiClientLink::Struct("virtual_events", "VirtualEventsApiClient"),
					]
					)
				])
				.build()
				.unwrap(),
			ResourceIdentity::BookingBusinesses => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::solutions::*", "crate::users::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("BookingBusinessesIdApiClient"), 
					vec![
						ApiClientLink::Struct("appointments", "AppointmentsApiClient"),
						ApiClientLink::StructId("appointment", "AppointmentsIdApiClient"),
						ApiClientLink::Struct("services", "ServicesApiClient"),
						ApiClientLink::StructId("service", "ServicesIdApiClient"),
						ApiClientLink::Struct("custom_questions", "CustomQuestionsApiClient"),
						ApiClientLink::StructId("custom_question", "CustomQuestionsIdApiClient"),
						ApiClientLink::Struct("customers", "CustomersApiClient"),
						ApiClientLink::StructId("customer", "CustomersIdApiClient"),
						ApiClientLink::Struct("staff_members", "StaffMembersApiClient"),
						ApiClientLink::StructId("staff_member", "StaffMembersIdApiClient"),
						ApiClientLink::Struct("calendar_views", "CalendarViewApiClient"),
						ApiClientLink::StructId("calendar_view", "CalendarViewIdApiClient"),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::VirtualEvents => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::solutions::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("VirtualEventsApiClient"),
										  vec![
											  ApiClientLink::Struct("events", "VirtualEventsEventsApiClient"),
											  ApiClientLink::Struct("webinars", "VirtualEventsWebinarsApiClient"),
											  ApiClientLink::StructId("event", "VirtualEventsEventsIdApiClient"),
											  ApiClientLink::StructId("webinar", "VirtualEventsWebinarsIdApiClient"),
										  ]
					)
				])
				.build()
				.unwrap(),
			ResourceIdentity::VirtualEventsEvents => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::solutions::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("VirtualEventsEventsIdApiClient"),
										  vec![
											  ApiClientLink::Struct("sessions", "VirtualEventsSessionsApiClient"),
											  ApiClientLink::StructId("session", "VirtualEventsSessionsIdApiClient"),
										  ]
					)
				])
				.build()
				.unwrap(),
			ResourceIdentity::VirtualEventsWebinars => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::solutions::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("VirtualEventsWebinarsIdApiClient"),
										  vec![
											  ApiClientLink::Struct("sessions", "VirtualEventsSessionsApiClient"),
											  ApiClientLink::StructId("session", "VirtualEventsSessionsIdApiClient"),
										  ]
					)
				])
				.build()
				.unwrap(),
			ResourceIdentity::VirtualEventsSessions => ResourceSettings::builder(path_name, ri)
				.build()
				.unwrap(),
			ResourceIdentity::Devices => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::TransitiveMemberOfApiClient", "crate::users::MemberOfApiClient", "crate::users::TransitiveMemberOfIdApiClient", "crate::users::MemberOfIdApiClient", "crate::devices::*"])
				.api_client_links(vec![ApiClientLinkSettings(Some("DevicesIdApiClient"), vec![
					ApiClientLink::StructId("registered_user", "DevicesRegisteredUsersIdApiClient"),
					ApiClientLink::Struct("registered_users", "DevicesRegisteredUsersApiClient"),
					ApiClientLink::StructId("registered_owner", "DevicesRegisteredOwnersIdApiClient"),
					ApiClientLink::Struct("registered_owners", "DevicesRegisteredOwnersApiClient"),
					ApiClientLink::StructId("transitive_member_of", "TransitiveMemberOfIdApiClient"),
					ApiClientLink::StructId("member_of", "MemberOfIdApiClient"),
					ApiClientLink::Struct("transitive_members_of", "TransitiveMemberOfApiClient"),
					ApiClientLink::Struct("members_of", "MemberOfApiClient"),
				]
				)
				])
				.build().unwrap(),
			_ => ResourceSettings::default(path_name, ri),

		}
    }
}

#[derive(Default, Debug, Clone, Serialize, AsFile, Eq, PartialEq, Hash)]
pub struct ResourceSettingsMap(pub BTreeMap<ResourceIdentity, ResourceSettings>);

pub fn get_me_filter() -> Vec<&'static str> {
    vec![
        "activities",
        "appRoleAssignments",
        "authentication",
        "calendar",
        "calendarGroups",
        "calendarView",
        "calendars",
        "chats",
        "contactFolders",
        "contacts",
        "createdObjects",
        "deviceManagementTroubleshootingEvents",
        "directReports",
        "childFolders",
        //"drives",
        "events",
        "extensions",
        "followedSites",
        "inferenceClassification",
        "insights",
        "joinedTeams",
        "licenseDetails",
        "mailFolders",
        "managedAppRegistrations",
        "managedDevices",
        "memberOf",
        "messages",
        "oauth2PermissionGrants",
        "onenote",
        "onlineMeetings",
        "outlook",
        "ownedDevices",
        "ownedObjects",
        //"people",
        "photos",
        "presence",
        "registeredDevices",
        "scopedRoleMemberOf",
        "teamwork",
        "todo",
        "threads",
        "transitiveMemberOf",
        // Second and third level custom
        "tasks",
        "buckets",
        "plans",
    ]
}

pub fn get_me_resource_identity() -> Vec<ResourceIdentity> {
    vec![
        ResourceIdentity::Activities,
        ResourceIdentity::AppRoleAssignments,
        ResourceIdentity::Authentication,
        ResourceIdentity::DefaultCalendar,
        ResourceIdentity::CalendarGroups,
        ResourceIdentity::CalendarView,
        ResourceIdentity::Calendars,
        ResourceIdentity::Channels,
        ResourceIdentity::ChildFolders,
        ResourceIdentity::ContactFolders,
        ResourceIdentity::Contacts,
        ResourceIdentity::CreatedObjects,
        ResourceIdentity::TroubleshootingEvents,
        ResourceIdentity::DirectReports,
        //ResourceIdentity::Drives,
        ResourceIdentity::Events,
        ResourceIdentity::Extensions,
        ResourceIdentity::FollowedSites,
        ResourceIdentity::InferenceClassification,
        ResourceIdentity::Insights,
        ResourceIdentity::EventsInstances,
        ResourceIdentity::JoinedTeams,
        ResourceIdentity::LicenseDetails,
        ResourceIdentity::MailFolders,
        ResourceIdentity::ManagedAppRegistrations,
        ResourceIdentity::UsersManagedDevices,
        ResourceIdentity::MemberOf,
        ResourceIdentity::UsersMessages,
        ResourceIdentity::Oauth2PermissionGrants,
        ResourceIdentity::Onenote,
        ResourceIdentity::OnenoteSections,
        ResourceIdentity::OnenoteSectionGroups,
        ResourceIdentity::OnenoteNotebooks,
        ResourceIdentity::Onenote,
        ResourceIdentity::OnlineMeetings,
        ResourceIdentity::Outlook,
        ResourceIdentity::OwnedDevices,
        ResourceIdentity::OwnedObjects,
        //ResourceIdentity::People,
        ResourceIdentity::Photos,
        ResourceIdentity::Presence,
        ResourceIdentity::RegisteredDevices,
        ResourceIdentity::ScopedRoleMemberOf,
        ResourceIdentity::Teamwork,
        ResourceIdentity::Todo,
        ResourceIdentity::TodoLists,
        ResourceIdentity::TodoListsTasks,
        ResourceIdentity::TransitiveMemberOf,
    ]
}

pub fn get_groups_filter() -> Vec<&'static str> {
    vec![
        "activities",
        //"agreementAcceptances",
        "appRoleAssignments",
        "authentication",
        "calendar",
        "calendarGroups",
        "calendarView",
        "calendars",
        "chats",
        "contactFolders",
        "contacts",
        "createdObjects",
        "deviceManagementTroubleshootingEvents",
        "directReports",
        //"drives",
        "events",
        "extensions",
        "followedSites",
        "inferenceClassification",
        "insights",
        "joinedTeams",
        "licenseDetails",
        "mailFolders",
        "managedAppRegistrations",
        "managedDevices",
        "memberOf",
        "messages",
        "oauth2PermissionGrants",
        "onenote",
        "onlineMeetings",
        "outlook",
        "ownedDevices",
        "ownedObjects",
        //"people",
        "photos",
        "planner",
        "presence",
        "registeredDevices",
        "scopedRoleMemberOf",
        "teamwork",
        "todo",
        "transitiveMemberOf",
        // Below are groups specific the rest are users and groups
        "owners",
        "transitiveMembers",
        "membersWithLicenseErrors",
        "permissionGrants",
        "threads",
        "conversations",
        "sites",
        "team",
        "groupLifecyclePolicies",
    ]
}

fn get_me_child_filters(resource_identity: ResourceIdentity) -> Vec<&'static str> {
    match resource_identity {
        ResourceIdentity::Calendars => vec![
            "events/{{id}}",
            "calendarGroup",
            "instances",
            "calendarView",
            "multiValueExtendedProperties",
            "singleValueExtendedProperties",
        ],
        ResourceIdentity::DefaultCalendar => vec![
        /*
                    "calendars",
                    "events/{{id}}",
                    "calendarGroup",
                    "instances",
                    "calendarView",
                    "multiValueExtendedProperties",
                    "singleValueExtendedProperties",
         */
                ],
        ResourceIdentity::CalendarView => vec![
            "/calendars/{calendar-id}",
            "calendarGroup",
            "instances",
            "/calendar/",
            "events",
            "multiValueExtendedProperties",
            "singleValueExtendedProperties",
        ],
        ResourceIdentity::CalendarGroups => vec![
            "instances",
            "/calendars/{calendar-id}",
            "/calendar/",
            "events",
            "multiValueExtendedProperties",
            "singleValueExtendedProperties",
            "calendarView",
        ],
        // Channels doesUserHaveAccess is part of a broken path
        ResourceIdentity::Channels => vec![
            "messages",
            "sharedWithTeams",
            "doesUserHaveAccess",
            "members",
        ],
        ResourceIdentity::JoinedTeams => {
            vec!["primaryChannel", "channels", "tags", "schedule"]
        }
        ResourceIdentity::Events => vec!["instances"],
        ResourceIdentity::MailFolders => vec!["childFolders", "messages"],
        ResourceIdentity::Outlook => vec!["supportedTimeZones()"],
        ResourceIdentity::Todo => vec!["lists"],
        ResourceIdentity::TodoLists => vec!["tasks"],
        ResourceIdentity::Users => vec!["exportDeviceAndAppManagementData()"],
        ResourceIdentity::Planner => vec!["plans", "tasks"],

        _ => vec![
            //"/multiValueExtendedProperties",
           // "singleValueExtendedProperties",
        ],
    }
}

fn get_users_api_client_links(resource_identity: ResourceIdentity) -> Vec<ApiClientLinkSettings> {
    let name = {
        if resource_identity.eq(&ResourceIdentity::Users) {
            Some("UsersIdApiClient")
        } else {
            Some("MeApiClient")
        }
    };

    vec![ApiClientLinkSettings(
        name,
        vec![
            ApiClientLink::Struct("chats", "ChatsApiClient"),
            ApiClientLink::StructId("chat", "ChatsIdApiClient"),
            // Users and Me
            ApiClientLink::Struct("agreement_acceptances", "AgreementAcceptancesApiClient"),
            ApiClientLink::StructId("agreement_acceptance", "AgreementAcceptancesIdApiClient"),
            ApiClientLink::Struct("default_calendar", "DefaultCalendarApiClient"),
            ApiClientLink::Struct("calendars", "CalendarsApiClient"),
            ApiClientLink::StructId("calendar", "CalendarsIdApiClient"),
            ApiClientLink::Struct("calendar_groups", "CalendarGroupsApiClient"),
            ApiClientLink::StructId("calendar_group", "CalendarGroupsIdApiClient"),
            ApiClientLink::Struct("calendar_views", "CalendarViewApiClient"),
            ApiClientLink::StructId("calendar_view", "CalendarViewIdApiClient"),
            ApiClientLink::Struct("messages", "UsersMessagesApiClient"),
            ApiClientLink::StructId("message", "UsersMessagesIdApiClient"),
            //ApiClientLink::Struct("agreement_acceptances", "AgreementAcceptancesApiClient"),
            ApiClientLink::Struct("app_role_assignments", "AppRoleAssignmentsApiClient"),
            ApiClientLink::StructId("app_role_assignment", "AppRoleAssignmentsIdApiClient"),
            ApiClientLink::Struct("authentication", "AuthenticationApiClient"),
            ApiClientLink::Struct("channels", "ChannelsApiClient"),
            ApiClientLink::StructId("channel", "ChannelsIdApiClient"),
            ApiClientLink::Struct("contact_folders", "ContactFoldersApiClient"),
            ApiClientLink::StructId("contact_folder", "ContactFoldersIdApiClient"),
            ApiClientLink::Struct("contacts", "ContactsApiClient"),
            ApiClientLink::StructId("contact", "ContactsIdApiClient"),
            ApiClientLink::Struct("joined_teams", "JoinedTeamsApiClient"),
            ApiClientLink::StructId("joined_team", "JoinedTeamsIdApiClient"),
            ApiClientLink::Struct("insights", "InsightsApiClient"),
            ApiClientLink::Struct("license_details", "LicenseDetailsApiClient"),
            ApiClientLink::StructId("license_detail", "LicenseDetailsIdApiClient"),
            ApiClientLink::Struct("followed_sites", "FollowedSitesApiClient"),
            ApiClientLink::Struct(
                "managed_app_registrations",
                "ManagedAppRegistrationsApiClient",
            ),
            ApiClientLink::StructId(
                "managed_app_registration",
                "ManagedAppRegistrationsIdApiClient",
            ),
            ApiClientLink::Struct("scoped_role_member_of", "ScopedRoleMemberOfApiClient"),
            ApiClientLink::StructId("scoped_role_member_of_id", "ScopedRoleMemberOfIdApiClient"),
            ApiClientLink::Struct("teamwork", "TeamworkApiClient"),
            ApiClientLink::Struct(
                "inference_classification",
                "InferenceClassificationApiClient",
            ),
            ApiClientLink::Struct("mail_folders", "MailFoldersApiClient"),
            ApiClientLink::StructId("mail_folder", "MailFoldersIdApiClient"),
            ApiClientLink::Struct("activities", "ActivitiesApiClient"),
            ApiClientLink::StructId("activity", "ActivitiesIdApiClient"),
            ApiClientLink::Struct(
                "device_management_troubleshooting_events",
                "DeviceManagementTroubleshootingEventsApiClient",
            ),
            ApiClientLink::StructId(
                "device_management_troubleshooting_event",
                "DeviceManagementTroubleshootingEventsIdApiClient",
            ),
            ApiClientLink::Struct("extensions", "ExtensionsApiClient"),
            ApiClientLink::StructId("extension", "ExtensionsIdApiClient"),
            ApiClientLink::Struct("todo", "TodoApiClient"),
            ApiClientLink::Struct("created_objects", "CreatedObjectsApiClient"),
            ApiClientLink::StructId("created_object", "CreatedObjectsIdApiClient"),
            ApiClientLink::Struct("transitive_member_of", "TransitiveMemberOfApiClient"),
            ApiClientLink::StructId("transitive_member_of_id", "TransitiveMemberOfIdApiClient"),
            ApiClientLink::Struct("direct_reports", "DirectReportsApiClient"),
            ApiClientLink::StructId("direct_report", "DirectReportsIdApiClient"),
            ApiClientLink::Struct("managed_devices", "ManagedDevicesApiClient"),
            ApiClientLink::StructId("managed_device", "ManagedDevicesIdApiClient"),
            ApiClientLink::Struct("events", "EventsApiClient"),
            ApiClientLink::StructId("event", "EventsIdApiClient"),
            ApiClientLink::Struct("online_meetings", "OnlineMeetingsApiClient"),
            ApiClientLink::StructId("online_meeting", "OnlineMeetingsIdApiClient"),
            ApiClientLink::Struct("photos", "PhotosApiClient"),
            ApiClientLink::StructId("photo", "PhotosIdApiClient"),
            ApiClientLink::Struct("outlook", "OutlookApiClient"),
            ApiClientLink::Struct("presence", "PresenceApiClient"),
            ApiClientLink::Struct("planner", "PlannerApiClient"),
            ApiClientLink::Struct("registered_devices", "RegisteredDevicesApiClient"),
            ApiClientLink::StructId("registered_device", "RegisteredDevicesIdApiClient"),
            ApiClientLink::Struct("owned_devices", "OwnedDevicesApiClient"),
            ApiClientLink::StructId("owned_device", "OwnedDevicesIdApiClient"),
            ApiClientLink::Struct("owned_objects", "OwnedObjectsApiClient"),
            ApiClientLink::StructId("owned_object", "OwnedObjectsIdApiClient"),
            ApiClientLink::Struct("member_of", "MemberOfApiClient"),
            ApiClientLink::StructId("member_of_id", "MemberOfIdApiClient"),
            ApiClientLink::Struct("onenote", "OnenoteApiClient"),
            ApiClientLink::Struct("schedule", "ScheduleApiClient"),
            ApiClientLink::Struct("settings", "SettingsApiClient"),
            ApiClientLink::Struct(
                "oauth2_permission_grants",
                "Oauth2PermissionGrantsApiClient",
            ),
            ApiClientLink::StructId(
                "oauth2_permission_grant",
                "Oauth2PermissionGrantsIdApiClient",
            ),
            ApiClientLink::Struct("mailbox_settings", "MailboxSettingsApiClient"),
            ApiClientLink::Struct("drive", "DefaultDriveApiClient"),
        ],
    )]
}

pub fn map_shared_write_config(ris: Vec<ResourceIdentity>) -> Vec<WriteConfiguration> {
    ris.iter()
        .map(|ri| match ri {
            ResourceIdentity::ChatsMessages => {
                WriteConfiguration::second_level_builder(ResourceIdentity::Users, *ri)
                    .trim_path_start("/users/{user-id}/chats/{chat-id}")
                    .filter_path(get_me_child_filters(*ri))
                    .build()
                    .unwrap()
            }
            ResourceIdentity::ChatsMessagesReplies => {
                WriteConfiguration::second_level_builder(ResourceIdentity::Users, *ri)
                    .trim_path_start("/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}")
                    .filter_path(get_me_child_filters(*ri))
                    .build()
                    .unwrap()
            }

            ResourceIdentity::Channels => {
                WriteConfiguration::second_level_builder(ResourceIdentity::Users, *ri)
                    .trim_path_start("/users/{user-id}/joinedTeams/{team-id}")
                    .filter_path(get_me_child_filters(*ri))
                    .imports(vec!["crate::teams::*", "crate::chats::*"])
                    .api_client_links(vec![ApiClientLinkSettings(
                        Some("ChannelsIdApiClient"),
                        vec![
                            ApiClientLink::Struct("messages", "ChatsMessagesApiClient"),
                            ApiClientLink::StructId("message", "ChatsMessagesIdApiClient"),
                            ApiClientLink::Struct("shared_with_teams", "SharedWithTeamsApiClient"),
                            ApiClientLink::StructId(
                                "shared_with_team",
                                "SharedWithTeamsIdApiClient",
                            ),
                            ApiClientLink::Struct("members", "TeamsMembersApiClient"),
                            ApiClientLink::StructId("member", "TeamsMembersIdApiClient"),
                        ],
                    )])
                    .build()
                    .unwrap()
            }
            ResourceIdentity::TodoLists => {
                WriteConfiguration::second_level_builder(ResourceIdentity::Users, *ri)
                    .trim_path_start("/users/{user-id}/todo")
                    .filter_path(get_me_child_filters(*ri))
                    .build()
                    .unwrap()
            }
            ResourceIdentity::TodoListsTasks => {
                WriteConfiguration::second_level_builder(ResourceIdentity::Users, *ri)
                    .trim_path_start("/users/{user-id}/todo/lists/{todoTaskList-id}")
                    .filter_path(vec!["attachments"])
                    .build()
                    .unwrap()
            }

            ResourceIdentity::EventsInstances => {
                WriteConfiguration::second_level_builder(ResourceIdentity::Users, *ri)
                    .filter_path(vec!["attachments"])
                    .trim_path_start("/users/{user-id}/events/{event-id}")
                    .filter_path(get_me_child_filters(*ri))
                    .build()
                    .unwrap()
            }
            ResourceIdentity::DefaultCalendar => get_write_configuration(*ri),
            ResourceIdentity::Onenote => get_write_configuration(*ri),
            ResourceIdentity::OnenoteSections => get_write_configuration(*ri),
            ResourceIdentity::OnenoteSectionGroups => get_write_configuration(*ri),
            ResourceIdentity::OnenoteNotebooks => get_write_configuration(*ri),
            ResourceIdentity::OnenotePages => get_write_configuration(*ri),

            ResourceIdentity::ContactFolders => {
                WriteConfiguration::second_level_builder(ResourceIdentity::Users, *ri)
                    .filter_path(vec![
                        "contacts",
                        "childFolders",
                        "multiValueExtendedProperties",
                        "singleValueExtendedProperties",
                    ])
                    .trim_path_start("/users/{user-id}")
                    .build()
                    .unwrap()
            }
            _ => WriteConfiguration::second_level_builder(ResourceIdentity::Users, *ri)
                .trim_path_start("/users/{user-id}")
                .filter_path(get_me_child_filters(*ri))
                .build()
                .unwrap(),
        })
        .collect()
}

pub fn get_me_children_write_config() -> Vec<WriteConfiguration> {
    map_shared_write_config(get_me_resource_identity())
}

pub fn map_shared_write_config_sites(ris: Vec<ResourceIdentity>) -> Vec<WriteConfiguration> {
    ris.iter()
        .map(|ri| {
            WriteConfiguration::second_level_builder(ResourceIdentity::Sites, *ri)
                .trim_path_start("/sites/{site-id}")
                .build()
                .unwrap()
        })
        .collect()
}

pub fn map_write_config(ris: Vec<ResourceIdentity>) -> Vec<WriteConfiguration> {
    ris.iter().map(|ri| get_write_configuration(*ri)).collect()
}

pub fn get_write_configuration(resource_identity: ResourceIdentity) -> WriteConfiguration {
    match resource_identity {
		ResourceIdentity::AuthenticationMethodsPolicy => WriteConfiguration::builder(resource_identity)
			.imports(vec!["crate::authentication_method_configurations::*"])
			.api_client_links(vec![
				ApiClientLinkSettings(
					None, vec![
						ApiClientLink::Struct("authentication_method_configurations", "AuthenticationMethodConfigurationsApiClient"),
						ApiClientLink::StructId("authentication_method_configuration", "AuthenticationMethodConfigurationsIdApiClient"),
					]
				),
			])
			.build().unwrap(),
		// Teamwork
		ResourceIdentity::Teamwork => WriteConfiguration::builder(resource_identity)
			.filter_path(vec!["channels", "deletedTeams"])
			.children(map_write_config(vec![ResourceIdentity::DeletedTeams]))
			.build()
			.unwrap(),
		ResourceIdentity::DeletedTeams => WriteConfiguration::second_level_builder(ResourceIdentity::Teamwork, resource_identity)
			.trim_path_start("/teamwork")
			.filter_path(vec!["channels"])
			.build()
			.unwrap(),


		// Planner
		ResourceIdentity::PlannerTasks => WriteConfiguration::second_level_builder(ResourceIdentity::Planner, resource_identity)
			.trim_path_start("/planner")
			.build()
			.unwrap(),
		ResourceIdentity::Plans => WriteConfiguration::second_level_builder(ResourceIdentity::Planner, resource_identity)
			.filter_path(vec!["tasks", "buckets"])
			.trim_path_start("/planner")
			.build()
			.unwrap(),
		ResourceIdentity::Buckets => WriteConfiguration::second_level_builder(ResourceIdentity::Planner, resource_identity)
			.filter_path(vec!["tasks"])
			.trim_path_start("/planner")
			.build()
			.unwrap(),
		ResourceIdentity::Planner => WriteConfiguration::builder(resource_identity)
			.filter_path(vec!["plans", "tasks", "buckets"])
			.children(map_write_config(vec![ResourceIdentity::PlannerTasks, ResourceIdentity::Plans, ResourceIdentity::Buckets]))
			.build()
			.unwrap(),


		ResourceIdentity::Agreements => WriteConfiguration::builder(resource_identity)
			.filter_path(vec!["file", "files"])
			.build()
			.unwrap(),
		ResourceIdentity::Applications => WriteConfiguration::builder(resource_identity)
			.imports(vec!["crate::service_principals::*", "crate::users::*"])
			.api_client_links(vec![
				ApiClientLinkSettings(
					Some("ServicePrincipalsIdApiClient"), vec![
						ApiClientLink::Struct("owners", "ServicePrincipalsOwnersApiClient"),
						ApiClientLink::StructId("owner", "ServicePrincipalsOwnersIdApiClient"),
						ApiClientLink::Struct("transitive_member_of", "TransitiveMemberOfApiClient"),
						ApiClientLink::Struct("member_of", "MemberOfApiClient"),
					]
				),
			])
			.filter_path(vec!["owners"])
			.build()
			.unwrap(),
		ResourceIdentity::Communications => WriteConfiguration::builder(resource_identity)
			.children(map_write_config(vec![ResourceIdentity::Calls, ResourceIdentity::CallRecords, ResourceIdentity::CallRecordsSessions]))
			.imports(vec!["crate::communications::{call_records::CallRecordsApiClient, call_records::CallRecordsIdApiClient, calls::CallsApiClient, calls::CallsIdApiClient}"])
			.api_client_links(vec![
				ApiClientLinkSettings(
					None,
					vec![
						ApiClientLink::Struct("call_records", "CallRecordsApiClient"),
						ApiClientLink::StructId(
							"call_record",
							"CallRecordsIdApiClient"
						),
						ApiClientLink::Struct("calls", "CallsApiClient"),
						ApiClientLink::StructId("call", "CallsIdApiClient")
					]
				)
			])
			.build()
			.unwrap(),
		ResourceIdentity::Calls =>  WriteConfiguration::second_level_builder(ResourceIdentity::Communications, resource_identity)
			.trim_path_start("/communications")
			.build()
			.unwrap(),
		ResourceIdentity::CallRecords =>  WriteConfiguration::second_level_builder(ResourceIdentity::Communications, resource_identity)
			.imports(vec!["crate::communications::*"])
			.trim_path_start("/communications")
			.api_client_links(vec![
				ApiClientLinkSettings(
					Some("CallRecordsIdApiClient"),
					vec![
						ApiClientLink::Struct("sessions", "CallRecordsSessionsApiClient"),
						ApiClientLink::StructId("session", "CallRecordsSessionsIdApiClient")
					]
				)
			])
			.build()
			.unwrap(),
		ResourceIdentity::CallRecordsSessions =>  WriteConfiguration::second_level_builder(ResourceIdentity::Communications, resource_identity)
			.trim_path_start("/communications/callRecords/{callRecord-id}")
			.build()
			.unwrap(),


		ResourceIdentity::CertificateBasedAuthConfiguration => WriteConfiguration::from(resource_identity),


		ResourceIdentity::Chats => WriteConfiguration::builder(resource_identity)
			.filter_path(vec!["messages", "members", "tabs"])
			.children(map_write_config(vec![ResourceIdentity::ChatsMessages, ResourceIdentity::ChatsMessagesReplies]))
			.imports(vec!["crate::chats::*", "crate::teams::*"])
			.api_client_links(vec![
				ApiClientLinkSettings(
					Some("ChatsIdApiClient"),
					vec![
						ApiClientLink::Struct("messages", "ChatsMessagesApiClient"),
						ApiClientLink::StructId("message", "ChatsMessagesIdApiClient"),
						ApiClientLink::Struct("members", "TeamsMembersApiClient"),
						ApiClientLink::Struct("member", "TeamsMembersIdApiClient"),
					]
				)
			])
			.build()
			.unwrap(),
		ResourceIdentity::ChatsMessages => WriteConfiguration::second_level_builder(ResourceIdentity::Chats, resource_identity)
			.trim_path_start("/chats/{chat-id}")
			.filter_path(vec!["replies"])
			.imports(vec!["crate::chats::*"])
			.api_client_links(vec![
				ApiClientLinkSettings(
					Some("ChatsMessagesIdApiClient"),
					vec![
						ApiClientLink::Struct("replies", "ChatsMessagesRepliesApiClient"),
						ApiClientLink::StructId("reply", "ChatsMessagesRepliesIdApiClient"),
					]
				)
			])
			.build()
			.unwrap(),
		ResourceIdentity::ChatsMessagesReplies => WriteConfiguration::second_level_builder(ResourceIdentity::Chats, resource_identity)
			.trim_path_start("/chats/{chat-id}/messages/{chatMessage-id}")
			.build()
			.unwrap(),


		ResourceIdentity::ConnectedOrganizations => WriteConfiguration::second_level_builder(ResourceIdentity::IdentityGovernance, resource_identity)
			.trim_path_start("/identityGovernance/entitlementManagement")
			.filter_path(vec!["internalSponsors", "externalSponsors"])
			.imports(vec!["crate::identity_governance::*"])
			.api_client_links(vec![
				ApiClientLinkSettings(Some("ConnectedOrganizationsIdApiClient"), vec![
					ApiClientLink::Struct("external_sponsors", "ConnectedOrganizationsExternalSponsorsApiClient"),
					ApiClientLink::Struct("internal_sponsors", "ConnectedOrganizationsInternalSponsorsApiClient"),
				])
			])
			.build()
			.unwrap(),
		ResourceIdentity::ConnectedOrganizationsInternalSponsors => WriteConfiguration::second_level_builder(ResourceIdentity::IdentityGovernance, resource_identity)
			.trim_path_start("/identityGovernance/entitlementManagement/connectedOrganizations/{connectedOrganization-id}")
			.build()
			.unwrap(),
		ResourceIdentity::ConnectedOrganizationsExternalSponsors => WriteConfiguration::second_level_builder(ResourceIdentity::IdentityGovernance, resource_identity)
			.trim_path_start("/identityGovernance/entitlementManagement/connectedOrganizations/{connectedOrganization-id}")
			.build()
			.unwrap(),


		// Device App Management
		ResourceIdentity::AndroidManagedAppProtections |
		ResourceIdentity::DefaultManagedAppProtections |
		ResourceIdentity::IosManagedAppProtections |
		ResourceIdentity::ManagedAppPolicies |
		ResourceIdentity::ManagedAppStatuses |
		ResourceIdentity::MdmWindowsInformationProtectionPolicies |
		ResourceIdentity::MobileAppCategories |
		ResourceIdentity::MobileAppConfigurations |
		ResourceIdentity::MobileApps |
		ResourceIdentity::TargetedManagedAppConfigurations |
		ResourceIdentity::VppTokens |
		ResourceIdentity::WindowsInformationProtectionPolicies  => WriteConfiguration::second_level_builder(ResourceIdentity::DeviceAppManagement, resource_identity)
			.trim_path_start("/deviceAppManagement")
			.build()
			.unwrap(),
		ResourceIdentity::ManagedEBooks => WriteConfiguration::second_level_builder(ResourceIdentity::DeviceAppManagement, resource_identity)
			.trim_path_start("/deviceAppManagement")
			.filter_path(vec!["userStateSummary", "deviceStates"])
			.imports(vec!["crate::device_app_management::*"])
			.api_client_links(vec![
				ApiClientLinkSettings(
					None, 							vec![
						ApiClientLink::Struct("device_states", "ManagedEBooksDeviceStatesApiClient"),
						ApiClientLink::StructId("device_state", "ManagedEBooksDeviceStatesIdApiClient"),
						ApiClientLink::Struct("user_state_summary", "ManagedEBooksUserStateSummaryApiClient"),
						ApiClientLink::StructId("user_state_summary_id", "ManagedEBooksUserStateSummaryIdApiClient"),
					]
				),
			])
			.build()
			.unwrap(),
		ResourceIdentity::ManagedEBooksDeviceStates |
		ResourceIdentity::ManagedEBooksUserStateSummary  => WriteConfiguration::second_level_builder(ResourceIdentity::DeviceAppManagement, resource_identity)
			.trim_path_start("/deviceAppManagement/managedEBooks/{managedEBook-id}")
			.build()
			.unwrap(),
		ResourceIdentity::ManagedAppRegistrations => WriteConfiguration::second_level_builder(ResourceIdentity::DeviceAppManagement, resource_identity)
			.trim_path_start("/deviceAppManagement")
			.filter_path(vec!["appliedPolicies", "intendedPolicies"])
			.imports(vec!["crate::device_app_management::*"])
			.api_client_links(vec![
				ApiClientLinkSettings(
					None, 							vec![
						ApiClientLink::Struct("intended_policies", "ManagedAppRegistrationsIntendedPoliciesApiClient"),
						ApiClientLink::StructId("intended_policies_id", "ManagedAppRegistrationsIntendedPoliciesIdApiClient"),
						ApiClientLink::Struct("applied_policies", "ManagedAppRegistrationsAppliedPoliciesApiClient"),
						ApiClientLink::StructId("applied_policies_id", "ManagedAppRegistrationsAppliedPoliciesIdApiClient"),
					]
				),
			])
			.build()
			.unwrap(),
		ResourceIdentity::ManagedAppRegistrationsIntendedPolicies |
		ResourceIdentity::ManagedAppRegistrationsAppliedPolicies  => WriteConfiguration::second_level_builder(ResourceIdentity::DeviceAppManagement, resource_identity)
			.trim_path_start("/deviceAppManagement/managedAppRegistrations/{managedAppRegistration-id}")
			.build()
			.unwrap(),
		ResourceIdentity::DeviceAppManagement => WriteConfiguration::builder(resource_identity)
			.filter_path(vec![
				"androidManagedAppProtections","defaultManagedAppProtections","iosManagedAppProtections","managedAppPolicies",
				"managedAppRegistrations","managedAppStatuses","managedEBooks","mdmWindowsInformationProtectionPolicies","mobileAppCategories",
				"mobileAppConfigurations","mobileApps","targetedManagedAppConfigurations","vppTokens","windowsInformationProtectionPolicies"
			])
			.children(map_write_config(vec![
				ResourceIdentity::AndroidManagedAppProtections, ResourceIdentity::DefaultManagedAppProtections,
				ResourceIdentity::IosManagedAppProtections, ResourceIdentity::ManagedAppPolicies,
				ResourceIdentity::ManagedAppRegistrations, ResourceIdentity::ManagedAppStatuses,
				ResourceIdentity::ManagedEBooks, ResourceIdentity::MdmWindowsInformationProtectionPolicies,
				ResourceIdentity::MobileAppCategories, ResourceIdentity::MobileAppConfigurations,
				ResourceIdentity::MobileApps, ResourceIdentity::TargetedManagedAppConfigurations,
				ResourceIdentity::VppTokens, ResourceIdentity::WindowsInformationProtectionPolicies,
				ResourceIdentity::ManagedEBooksDeviceStates, ResourceIdentity::ManagedEBooksUserStateSummary,
				ResourceIdentity::ManagedAppRegistrationsAppliedPolicies, ResourceIdentity::ManagedAppRegistrationsIntendedPolicies
			]))
			.imports(vec!["crate::device_app_management::*"])
			.api_client_links(vec![
				ApiClientLinkSettings(
					None, vec![
						ApiClientLink::Struct("android_managed_app_protections", "AndroidManagedAppProtectionsApiClient"),
						ApiClientLink::StructId("android_managed_app_protection", "AndroidManagedAppProtectionsIdApiClient"),
						ApiClientLink::Struct("default_managed_app_protections", "DefaultManagedAppProtectionsApiClient"),
						ApiClientLink::StructId("default_managed_app_protection", "DefaultManagedAppProtectionsIdApiClient"),
						ApiClientLink::Struct("ios_managed_app_protections", "IosManagedAppProtectionsApiClient"),
						ApiClientLink::StructId("ios_managed_app_protection", "IosManagedAppProtectionsIdApiClient"),
						ApiClientLink::Struct("managed_app_registrations", "ManagedAppRegistrationsApiClient"),
						ApiClientLink::StructId("managed_app_registration", "ManagedAppRegistrationsIdApiClient"),
						ApiClientLink::Struct("managed_app_statuses", "ManagedAppStatusesApiClient"),
						ApiClientLink::StructId("managed_app_statuses_id", "ManagedAppStatusesIdApiClient"),
						ApiClientLink::Struct("mdm_windows_information_protection_policies", "MdmWindowsInformationProtectionPoliciesApiClient"),
						ApiClientLink::StructId("mdm_windows_information_protection_policy", "MdmWindowsInformationProtectionPoliciesIdApiClient"),
						ApiClientLink::Struct("managed_app_policies", "ManagedAppPoliciesApiClient"),
						ApiClientLink::StructId("managed_app_policies_id", "ManagedAppPoliciesIdApiClient"),
						ApiClientLink::Struct("mobile_app_categories", "MobileAppCategoriesApiClient"),
						ApiClientLink::StructId("mobile_app_categories_id", "MobileAppCategoriesIdApiClient"),
						ApiClientLink::Struct("managed_e_books", "ManagedEBooksApiClient"),
						ApiClientLink::StructId("managed_e_book", "ManagedEBooksIdApiClient"),
						ApiClientLink::Struct("mobile_app_configurations", "MobileAppConfigurationsApiClient"),
						ApiClientLink::StructId("mobile_app_configuration", "MobileAppConfigurationsIdApiClient"),
						ApiClientLink::Struct("mobile_apps", "MobileAppsApiClient"),
						ApiClientLink::StructId("mobile_app", "MobileAppsIdApiClient"),
						ApiClientLink::Struct("targeted_managed_app_configurations", "TargetedManagedAppConfigurationsApiClient"),
						ApiClientLink::StructId("targeted_managed_app_configuration", "TargetedManagedAppConfigurationsIdApiClient"),
						ApiClientLink::Struct("vpp_tokens", "VppTokensApiClient"),
						ApiClientLink::StructId("vpp_token", "VppTokensIdApiClient"),
						ApiClientLink::Struct("windows_information_protection_policies", "WindowsInformationProtectionPoliciesApiClient"),
						ApiClientLink::StructId("windows_information_protection_policies_id", "WindowsInformationProtectionPoliciesIdApiClient"),
					]
				),
			])
			.build()
			.unwrap(),

		// Device Management
		ResourceIdentity::DeviceConfigurations |
		ResourceIdentity::DeviceEnrollmentConfigurations |
		ResourceIdentity::DeviceManagementManagedDevices |
		ResourceIdentity::RoleDefinitions |
		ResourceIdentity::TermsAndConditions |
		ResourceIdentity::TroubleshootingEvents |
		ResourceIdentity::DeviceCompliancePolicySettingStateSummaries |
		ResourceIdentity::WindowsAutopilotDeviceIdentities |
		ResourceIdentity::DeviceManagementReports => WriteConfiguration::second_level_builder(ResourceIdentity::DeviceManagement, resource_identity)
			.trim_path_start("/deviceManagement")
			.build()
			.unwrap(),
		ResourceIdentity::DeviceManagement =>
			WriteConfiguration::builder(resource_identity)
				.filter_path(vec![
					"deviceEnrollmentConfigurations", "deviceConfigurations", "deviceCompliancePolicies",
					"termsAndConditions", "managedDevices", "roleDefinitions", "troubleshootingEvents",
					"reports", "deviceCompliancePolicySettingStateSummaries", "windowsAutopilotDeviceIdentities"
				])
				.children(map_write_config(vec![
					ResourceIdentity::DeviceConfigurations,
					ResourceIdentity::DeviceEnrollmentConfigurations,
					ResourceIdentity::DeviceManagementManagedDevices,
					ResourceIdentity::RoleDefinitions,
					ResourceIdentity::TermsAndConditions,
					ResourceIdentity::TroubleshootingEvents,
					ResourceIdentity::DeviceManagementReports,
					ResourceIdentity::DeviceCompliancePolicySettingStateSummaries,
					ResourceIdentity::WindowsAutopilotDeviceIdentities,
				]))
				.imports(vec!["crate::device_management::*"])
				.api_client_links(vec![
					ApiClientLinkSettings(
						None, vec![
							ApiClientLink::Struct("device_configurations", "DeviceConfigurationsApiClient"),
							ApiClientLink::StructId("device_configuration", "DeviceConfigurationsIdApiClient"),
							ApiClientLink::Struct("device_enrollment_configurations", "DeviceEnrollmentConfigurationsApiClient"),
							ApiClientLink::StructId(
								"device_enrollment_configuration",
								"DeviceEnrollmentConfigurationsIdApiClient"
							),
							ApiClientLink::Struct("managed_devices", "DeviceManagementManagedDevicesApiClient"),
							ApiClientLink::StructId("managed_device", "DeviceManagementManagedDevicesIdApiClient"),
							ApiClientLink::Struct("role_definitions", "RoleDefinitionsApiClient"),
							ApiClientLink::StructId("role_definition", "RoleDefinitionsIdApiClient"),
							ApiClientLink::Struct("terms_and_conditions", "TermsAndConditionsApiClient"),
							ApiClientLink::StructId("terms_and_condition", "TermsAndConditionsIdApiClient"),
							ApiClientLink::Struct("troubleshooting_events", "TroubleshootingEventsApiClient"),
							ApiClientLink::StructId("troubleshooting_event", "TroubleshootingEventsIdApiClient"),
							ApiClientLink::Struct("reports", "DeviceManagementReportsApiClient"),
							ApiClientLink::Struct("device_compliance_policy_setting_state_summaries", "DeviceCompliancePolicySettingStateSummariesApiClient"),
							ApiClientLink::StructId("device_compliance_policy_setting_state_summaries_id", "DeviceCompliancePolicySettingStateSummariesIdApiClient"),
							ApiClientLink::StructId("windows_autopilot_device_identities", "WindowsAutopilotDeviceIdentitiesApiClient"),
							ApiClientLink::StructId("windows_autopilot_device_identities_id", "WindowsAutopilotDeviceIdentitiesIdApiClient"),
						]
					),
				])
				.build()
				.unwrap(),


		// Directory
		ResourceIdentity::DeletedItems | ResourceIdentity::AdministrativeUnits => WriteConfiguration::second_level_builder(ResourceIdentity::Directory, resource_identity)
			.trim_path_start("/directory")
			.filter_path(vec!["members"])
			.build()
			.unwrap(),
		ResourceIdentity::DirectoryMembers => WriteConfiguration::second_level_builder(ResourceIdentity::Directory, resource_identity)
			.trim_path_start("/directory/administrativeUnits/{administrativeUnit-id}")
			.build()
			.unwrap(),
		ResourceIdentity::Directory =>
			WriteConfiguration::builder(resource_identity)
				.filter_path(vec![
					"microsoft.graph.additionalAccess()", "directoryObjects", "directoryRoleTemplates",
					"directoryRoles", "administrativeUnits", "deletedItems"
				])
				.children(vec![
					get_write_configuration(ResourceIdentity::DeletedItems),
					get_write_configuration(ResourceIdentity::AdministrativeUnits),
					get_write_configuration(ResourceIdentity::DirectoryMembers),
				])
				.build()
				.unwrap(),


		// Directory Roles
		ResourceIdentity::DirectoryRoles => WriteConfiguration::builder(resource_identity)
			.filter_path(vec!["members"])
			.build()
			.unwrap(),


		ResourceIdentity::DomainDnsRecords => WriteConfiguration::from(resource_identity),

		ResourceIdentity::Devices => WriteConfiguration::builder(resource_identity)
			.filter_path(vec!["registeredOwners", "registeredUsers", "memberOf", "transitiveMemberOf"])
			.imports(vec!["crate::users::TransitiveMemberOfApiClient", "crate::users::MemberOfApiClient", "crate::users::TransitiveMemberOfIdApiClient", "crate::users::MemberOfIdApiClient", "crate::devices::*"])
			.children(vec![
				get_write_configuration(ResourceIdentity::DevicesRegisteredOwners),
				get_write_configuration(ResourceIdentity::DevicesRegisteredUsers),
			])
			.api_client_links(vec![ApiClientLinkSettings(Some("DevicesIdApiClient"), vec![
					ApiClientLink::StructId("registered_user", "DevicesRegisteredUsersIdApiClient"),
					ApiClientLink::Struct("registered_users", "DevicesRegisteredUsersApiClient"),
					ApiClientLink::StructId("registered_owner", "DevicesRegisteredOwnersIdApiClient"),
					ApiClientLink::Struct("registered_owners", "DevicesRegisteredOwnersApiClient"),
					ApiClientLink::StructId("transitive_member_of", "TransitiveMemberOfIdApiClient"),
					ApiClientLink::StructId("member_of", "MemberOfIdApiClient"),
					ApiClientLink::Struct("transitive_members_of", "TransitiveMemberOfApiClient"),
					ApiClientLink::Struct("members_of", "MemberOfApiClient"),
					]
				)
			])
			.build()
			.unwrap(),

		ResourceIdentity::DevicesRegisteredUsers | ResourceIdentity::DevicesRegisteredOwners => WriteConfiguration::second_level_builder(ResourceIdentity::Devices, resource_identity)
			.trim_path_start("/devices/{device-id}")
			.build()
			.unwrap(),

		// Identity Governance
		ResourceIdentity::EntitlementManagement => WriteConfiguration::second_level_builder(ResourceIdentity::Directory, resource_identity)
			.trim_path_start("/entitlementManagement")
			.filter_path(vec![
				"assignments", "accessPackages", "assignmentPolicies", "assignmentRequests",
				"catalogs", "$count", "accessPackageAssignmentApprovals", "connectedOrganizations"
			])
			.build()
			.unwrap(),
		ResourceIdentity::EntitlementManagementAssignments |
		ResourceIdentity::EntitlementManagementCatalogs => WriteConfiguration::second_level_builder(ResourceIdentity::Directory, resource_identity)
			.trim_path_start("/identityGovernance/entitlementManagement")
			.filter_path(vec!["additionalAccess()", "accessPackages/"])
			.build()
			.unwrap(),

		ResourceIdentity::AccessPackages => WriteConfiguration::second_level_builder(ResourceIdentity::IdentityGovernance, resource_identity)
			.imports(vec!["crate::identity_governance::*"])
			.trim_path_start("/identityGovernance/entitlementManagement")
			.api_client_links(vec![
				ApiClientLinkSettings(
					Some("AccessPackagesIdApiClient"),
					vec![
						ApiClientLink::Struct(
							"assignment_policies",
							"AssignmentPoliciesApiClient"
						),
						ApiClientLink::StructId(
							"assignment_policy",
							"AssignmentPoliciesIdApiClient"
						)
					]
				)
			])
			.build()
			.unwrap(),
		ResourceIdentity::AccessPackageAssignmentApprovals |
		ResourceIdentity::AssignmentPolicies |
		ResourceIdentity::AssignmentRequests => WriteConfiguration::second_level_builder(ResourceIdentity::IdentityGovernance, resource_identity)
			.trim_path_start("/identityGovernance/entitlementManagement")
			.build().unwrap(),
		ResourceIdentity::AppConsent => WriteConfiguration::second_level_builder(ResourceIdentity::IdentityGovernance, resource_identity)
			.imports(vec!["crate::identity_governance::*"])
			.filter_path(vec!["definitions", "/multiValueExtendedProperties", "singleValueExtendedProperties"])
			.trim_path_start("/identityGovernance".to_string())
			.build().unwrap(),
		ResourceIdentity::AccessReviews => WriteConfiguration::second_level_builder(ResourceIdentity::IdentityGovernance, resource_identity)
			.imports(vec!["crate::identity_governance::*"])
			.api_client_links( vec![
				ApiClientLinkSettings(
					None,
					vec![
						ApiClientLink::Struct("definitions", "AccessReviewsDefinitionsApiClient"),
						ApiClientLink::StructId("definition", "AccessReviewsDefinitionsIdApiClient")
					]
				)
			])
			.filter_path(vec!["definitions", "/multiValueExtendedProperties", "singleValueExtendedProperties"])
			.trim_path_start("/identityGovernance".to_string())
			.build().unwrap(),
		ResourceIdentity::AccessReviewsDefinitions => WriteConfiguration::second_level_builder(ResourceIdentity::IdentityGovernance, resource_identity)
			.imports(vec!["crate::identity_governance::*"])
			.trim_path_start("/identityGovernance/accessReviews".to_string())
			.filter_path(vec!["instance"])
			.api_client_links( vec![
				ApiClientLinkSettings(
					Some("AccessReviewsDefinitionsIdApiClient"),
					vec![
						ApiClientLink::Struct("instances", "AccessReviewsDefinitionsInstancesApiClient"),
						ApiClientLink::StructId("instance", "AccessReviewsDefinitionsInstancesIdApiClient")
					]
				)
			])
			.build().unwrap(),
		ResourceIdentity::AccessReviewsDefinitionsInstances => WriteConfiguration::second_level_builder(ResourceIdentity::IdentityGovernance, resource_identity)
			.imports(vec!["crate::identity_governance::*"])
			.trim_path_start("/identityGovernance/accessReviews/definitions/{accessReviewScheduleDefinition-id}")
			.api_client_links(vec![
				ApiClientLinkSettings(
					Some("AccessReviewsDefinitionsInstancesIdApiClient"),
					vec![
						ApiClientLink::Struct("stages", "AccessReviewsDefinitionsInstancesStagesApiClient"),
						ApiClientLink::StructId("stage", "AccessReviewsDefinitionsInstancesStagesIdApiClient")
					]
				)
			])
			.build().unwrap(),
		ResourceIdentity::AccessReviewsDefinitionsInstancesStages => WriteConfiguration::second_level_builder(ResourceIdentity::IdentityGovernance, resource_identity)
			.trim_path_start("/identityGovernance/accessReviews/definitions/{accessReviewScheduleDefinition-id}/instances/{accessReviewInstance-id}")
			.build().unwrap(),

		ResourceIdentity::IdentityGovernance =>
			WriteConfiguration::builder(resource_identity)
				.imports(vec!["crate::identity_governance::{AccessReviewsApiClient, AccessPackagesApiClient, AccessPackagesIdApiClient, EntitlementManagementApiClient}"])
				.filter_path(vec!["entitlementManagement", "accessReviews", "appConsent"])
				.api_client_links( vec![
					ApiClientLinkSettings(
						None,
						vec![
							ApiClientLink::Struct("access_reviews", "AccessReviewsApiClient"),
							ApiClientLink::Struct("entitlement_management", "EntitlementManagementApiClient")
						]
					)
				])
				.children(
					vec![
						get_write_configuration(ResourceIdentity::AccessPackageAssignmentApprovals),
						get_write_configuration(ResourceIdentity::AccessReviewsDefinitions),
						get_write_configuration(ResourceIdentity::AccessReviewsDefinitionsInstances),
						get_write_configuration(ResourceIdentity::AccessReviewsDefinitionsInstancesStages),
						get_write_configuration(ResourceIdentity::AccessReviews),
						get_write_configuration(ResourceIdentity::AccessPackages),
						get_write_configuration(ResourceIdentity::AppConsent),
						get_write_configuration(ResourceIdentity::AssignmentPolicies),
						get_write_configuration(ResourceIdentity::AssignmentRequests),
						get_write_configuration(ResourceIdentity::ConnectedOrganizations),
						get_write_configuration(ResourceIdentity::ConnectedOrganizationsExternalSponsors),
						get_write_configuration(ResourceIdentity::ConnectedOrganizationsInternalSponsors),
						get_write_configuration(ResourceIdentity::EntitlementManagement),
						get_write_configuration(ResourceIdentity::EntitlementManagementAssignments),
						get_write_configuration(ResourceIdentity::EntitlementManagementCatalogs)
					]
				)
				.build()
				.unwrap(),


		// Sites
		ResourceIdentity::Sites => WriteConfiguration::builder(resource_identity)
			.children(map_write_config(vec![
				ResourceIdentity::SitesLists, ResourceIdentity::SitesItems, ResourceIdentity::SitesItemsVersions, ResourceIdentity::SitesContentTypes,
				ResourceIdentity::TermStores, ResourceIdentity::TermStore, ResourceIdentity::TermStoreGroups,
				ResourceIdentity::TermStoreSets, ResourceIdentity::TermStoreSetsChildren, ResourceIdentity::TermStoreSetsTerms,
				ResourceIdentity::TermStoreSetsParentGroup
			]))
			.filter_path(vec!["termStores", "termStore", "lists",
							  "getActivitiesByInterval()", "onenote", "contentTypes"])
			.build()
			.unwrap(),
		ResourceIdentity::SitesLists => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.filter_path(vec!["items", "contentTypes", "getActivitiesByInterval()", "versions"])
			.trim_path_start("/sites/{site-id}")
			.build()
			.unwrap(),
		ResourceIdentity::SitesContentTypes => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.trim_path_start("/sites/{site-id}")
			.build()
			.unwrap(),
		ResourceIdentity::SitesItems => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.filter_path(vec!["versions", "getActivitiesByInterval()"])
			.trim_path_start("/sites/{site-id}/lists/{list-id}")
			.build()
			.unwrap(),
		ResourceIdentity::SitesItemsVersions => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.trim_path_start("/sites/{site-id}/lists/{list-id}/items/{listItem-id}")
			.build()
			.unwrap(),
		ResourceIdentity::TermStore => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.filter_path(vec!["termStores", "sets", "groups"])
			.trim_path_start("/sites/{site-id}")
			.build()
			.unwrap(),
		ResourceIdentity::TermStoreGroups => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.filter_path(vec!["sets"])
			.trim_path_start("/sites/{site-id}/termStore")
			.build()
			.unwrap(),
		ResourceIdentity::TermStoreSets => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.filter_path(vec!["children", "parentGroup", "terms", "relations"])
			.trim_path_start("/sites/{site-id}/termStore")
			.build()
			.unwrap(),
		ResourceIdentity::TermStoreSetsChildren => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.filter_path(vec!["children/{term-id1}", "relations/{relation-id}"])
			.trim_path_start("/sites/{site-id}/termStore/sets/{set-id}")
			.build()
			.unwrap(),
		ResourceIdentity::TermStoreSetsRelations => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.trim_path_start("/sites/{site-id}/termStore/sets/{set-id}")
			.build()
			.unwrap(),
		ResourceIdentity::TermStoreSetsTerms => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.filter_path(vec!["children", "relations"])
			.trim_path_start("/sites/{site-id}/termStore/sets/{set-id}")
			.build()
			.unwrap(),
		ResourceIdentity::TermStoreSetsParentGroup => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.filter_path(vec!["sets/{set-id1}/children", "sets/{set-id1}/terms", "sets/{set-id1}/relations"])
			.trim_path_start("/sites/{site-id}/termStore/sets/{set-id}")
			.build()
			.unwrap(),
		ResourceIdentity::TermStores => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.filter_path(vec!["sets", "groups"])
			.trim_path_start("/sites/{site-id}")
			.build()
			.unwrap(),


		// Users and Me
		ResourceIdentity::Me => WriteConfiguration::builder(resource_identity)
			.filter_path(get_me_filter())
			.build()
			.unwrap(),
		ResourceIdentity::Users => WriteConfiguration::builder(resource_identity)
			.filter_path(get_me_filter())
			.children(get_me_children_write_config())
			.build()
			.unwrap(),


		ResourceIdentity::Reports =>
			WriteConfigurationBuilder::default()
				.modifier_name("reports")
				.path("/reports")
				.build()
				.unwrap(),


		// Teams
		ResourceIdentity::Teams =>
			WriteConfiguration::builder(resource_identity)
				.children(vec![
					get_write_configuration(ResourceIdentity::PrimaryChannel),
					get_write_configuration(ResourceIdentity::SharedWithTeams),
					get_write_configuration(ResourceIdentity::TeamsTags),
					get_write_configuration(ResourceIdentity::Schedule),
					get_write_configuration(ResourceIdentity::TeamsMembers),
				]
				)
				.build()
				.unwrap(),
		ResourceIdentity::Schedule => WriteConfiguration::second_level_builder(ResourceIdentity::Teams, resource_identity)
			.trim_path_start("/teams/{team-id}".to_string())
			.build()
			.unwrap(),
		ResourceIdentity::TeamsMembers => WriteConfiguration::second_level_builder(ResourceIdentity::Teams, resource_identity)
			.path("/members")
			.trim_path_start("/teams/{team-id}".to_string())
			.build()
			.unwrap(),
		ResourceIdentity::PrimaryChannel => WriteConfiguration::second_level_builder(ResourceIdentity::Teams, resource_identity)
			.path("/primaryChannel".to_string())
			.trim_path_start("/teams/{team-id}".to_string())
			.filter_path(vec!["sharedWithTeams", "messages", "members"])
			.build()
			.unwrap(),
		ResourceIdentity::SharedWithTeams => WriteConfiguration::second_level_builder(ResourceIdentity::Teams, resource_identity)
			.path("/sharedWithTeams".to_string())
			.trim_path_start("/teams/{team-id}/primaryChannel".to_string())
			.build()
			.unwrap(),
		ResourceIdentity::TeamsTags => WriteConfiguration::second_level_builder(ResourceIdentity::Teams, resource_identity)
			.path("/tags".to_string())
			.trim_path_start("/teams/{team-id}".to_string())
			.filter_path(vec!["members"])
			.build()
			.unwrap(),


		// Calendars
		ResourceIdentity::Calendars => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}")
			.path("/calendars/")
			.filter_path(vec![
				"calendarGroup",
				"instances",
				"calendarView",
				"events",
				"multiValueExtendedProperties",
				"singleValueExtendedProperties",
				"/attachments/",
			])
			.replace_operation_map(resource_identity.exact_camel_case())
			.build().unwrap(),
		ResourceIdentity::DefaultCalendar => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}")
			.filter_path(vec![
				"calendarGroup", "instances", "calendarView", "events", "multiValueExtendedProperties",
				"singleValueExtendedProperties", "calendars", "attachments",
			])
			.build().unwrap(),
		ResourceIdentity::CalendarView => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}")
			.filter_path(vec![
				"calendarGroup", "instances", "/calendars/", "/calendar/", "events",
				"multiValueExtendedProperties", "singleValueExtendedProperties", "/attachments/",
			])
			.build().unwrap(),
		ResourceIdentity::CalendarGroups => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}")
			.filter_path(vec![
				"calendarView", "instances", "/calendars/", "/calendar/", "events",
				"multiValueExtendedProperties", "singleValueExtendedProperties", "/attachments/",
			])
			.build().unwrap(),

		ResourceIdentity::CreatedByUser => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.trim_path_start("/drives/{drive-id}")
			.build()
			.unwrap(),
		ResourceIdentity::LastModifiedByUser => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.trim_path_start("/drives/{drive-id}")
			.build()
			.unwrap(),

		ResourceIdentity::Drives => WriteConfiguration::builder(resource_identity)
			.filter_path(vec!["items", "list", "createdByUser", "lastModifiedByUser"])
			.imports(vec!["crate::drives::*"])
			.children(map_write_config(vec![
				ResourceIdentity::DrivesList, ResourceIdentity::DrivesItems, ResourceIdentity::DrivesListContentTypes,
				ResourceIdentity::Workbook, ResourceIdentity::WorkbookFunctions, ResourceIdentity::WorkbookTables,
				ResourceIdentity::WorkbookTablesColumns, ResourceIdentity::WorkbookTablesRows,
				ResourceIdentity::Worksheets, ResourceIdentity::CreatedByUser, ResourceIdentity::LastModifiedByUser,
				ResourceIdentity::WorksheetsCharts, ResourceIdentity::WorksheetsChartsAxes,
				ResourceIdentity::WorksheetsChartsAxesCategoryAxis, ResourceIdentity::WorksheetsChartsAxesSeriesAxis,
				ResourceIdentity::WorksheetsChartsAxesValueAxis, ResourceIdentity::WorksheetsChartsLegend,
				ResourceIdentity::WorksheetsChartsSeries, ResourceIdentity::WorksheetsChartsTitle,
				ResourceIdentity::WorksheetsChartsFormat, ResourceIdentity::WorksheetsChartsDataLabels
			]))
			.api_client_links(vec![
				// api_client_link_id!(item_by_path, DrivesItemsPathIdApiClient);
				ApiClientLinkSettings(Some("DrivesIdApiClient"), vec![
					ApiClientLink::Struct("list", "DrivesListApiClient"),
					ApiClientLink::Struct("items", "DrivesItemsApiClient"),
					ApiClientLink::StructId("item", "DrivesItemsIdApiClient"),
					ApiClientLink::StructId("item_by_path", "DrivesItemsPathIdApiClient"),
					ApiClientLink::Struct("workbook", "WorkbookApiClient"),
					ApiClientLink::Struct("worksheets", "WorksheetsApiClient"),
					ApiClientLink::StructId("worksheet", "WorksheetsIdApiClient"),
					ApiClientLink::Struct("last_modified_by_user", "LastModifiedByUserApiClient"),
					ApiClientLink::Struct("created_by_user", "CreatedByUserApiClient"),
				])
			])
			.custom_modules(vec!["manual_request", "drives_items_path"])
			.build()
			.unwrap(),
		ResourceIdentity::DrivesList => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.trim_path_start("/drives/{drive-id}")
			.filter_path(vec!["items", "contentTypes", "createdByUser", "lastModifiedByUser"])
			.build()
			.unwrap(),
		ResourceIdentity::DrivesListContentTypes => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.trim_path_start("/drives/{drive-id}/list")
			.build()
			.unwrap(),
		ResourceIdentity::DrivesItems => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.trim_path_start("/drives/{drive-id}")
			.filter_path(vec!["workbook", "getActivitiesByInterval()", "createdByUser", "lastModifiedByUser"])
			.imports(vec!["crate::drives::*"])
			.api_client_links(vec![
				ApiClientLinkSettings(Some("DrivesItemsIdApiClient"), vec![
					ApiClientLink::Struct("last_modified_by_user", "LastModifiedByUserApiClient"),
					ApiClientLink::Struct("created_by_user", "CreatedByUserApiClient"),
					ApiClientLink::Struct("workbook", "WorkbookApiClient"),
					ApiClientLink::Struct("worksheets", "WorksheetsApiClient"),
					ApiClientLink::StructId("worksheet", "WorksheetsIdApiClient"),
				])
			])
			.build()
			.unwrap(),
		ResourceIdentity::Workbook => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.trim_path_start("/drives/{drive-id}/items/{driveItem-id}")
			.filter_path(vec!["tables", "worksheets", "functions", "names", "comments"])
			.api_client_links(vec![
				ApiClientLinkSettings(Some("WorkbookApiClient"), vec![
					ApiClientLink::Struct("tables", "WorkbookTablesApiClient"),
					ApiClientLink::StructId("table", "WorkbookTablesIdApiClient"),
					ApiClientLink::Struct("functions", "WorkbookFunctionsApiClient"),
				])
			])
			.build()
			.unwrap(),
		ResourceIdentity::WorkbookFunctions => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.trim_path_start("/drives/{drive-id}/items/{driveItem-id}/workbook")
			.build()
			.unwrap(),
		ResourceIdentity::WorkbookTables => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.trim_path_start("/drives/{drive-id}/items/{driveItem-id}/workbook")
			.filter_path(vec!["columns", "rows"])
			.api_client_links(vec![
				ApiClientLinkSettings(Some("WorkbookTablesIdApiClient"), vec![
					ApiClientLink::Struct("columns", "WorkbookTablesColumnsApiClient"),
					ApiClientLink::StructId("column", "WorkbookTablesColumnsIdApiClient"),
					ApiClientLink::Struct("rows", "WorkbookTablesRowsApiClient"),
					ApiClientLink::StructId("row", "WorkbookTablesRowsIdApiClient"),
				])
			])
			.build()
			.unwrap(),
		ResourceIdentity::WorkbookTablesColumns => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.trim_path_start("/drives/{drive-id}/items/{driveItem-id}/workbook/tables/{workbookTable-id}")
			.build()
			.unwrap(),
		ResourceIdentity::WorkbookTablesRows => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.trim_path_start("/drives/{drive-id}/items/{driveItem-id}/workbook/tables/{workbookTable-id}")
			.build()
			.unwrap(),
		ResourceIdentity::Worksheets => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.trim_path_start("/drives/{drive-id}/items/{driveItem-id}/workbook")
			.filter_path(vec!["tables", "charts", "names", "pivotTables"])
			.imports(vec!["crate::drives::*"])
			.api_client_links(vec![
				ApiClientLinkSettings(Some("WorksheetsIdApiClient"), vec![
					ApiClientLink::Struct("charts", "WorksheetsChartsApiClient"),
					ApiClientLink::StructId("chart", "WorksheetsChartsIdApiClient"),
				])
			])
			.build()
			.unwrap(),
		ResourceIdentity::WorksheetsCharts => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.trim_path_start("/drives/{drive-id}/items/{driveItem-id}/workbook/worksheets/{workbookWorksheet-id}")
			.filter_path(vec!["axes", "legend", "series", "format", "dataLabels", "title"])
			.imports(vec!["crate::drives::*"])
			.api_client_links(vec![
				ApiClientLinkSettings(Some("WorksheetsChartsIdApiClient"), vec![
					ApiClientLink::Struct("axes", "WorksheetsChartsAxesApiClient"),
					ApiClientLink::Struct("legend", "WorksheetsChartsLegendApiClient"),
					ApiClientLink::Struct("series", "WorksheetsChartsSeriesApiClient"),
					ApiClientLink::Struct("data_labels", "WorksheetsChartsDataLabelsApiClient"),
					ApiClientLink::Struct("formatting", "WorksheetsChartsFormatApiClient"),
					ApiClientLink::Struct("title", "WorksheetsChartsTitleApiClient"),
				])
			])
			.build()
			.unwrap(),
		ResourceIdentity::WorksheetsChartsAxes => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.trim_path_start("/drives/{drive-id}/items/{driveItem-id}/workbook/worksheets/{workbookWorksheet-id}/charts/{workbookChart-id}")
			.filter_path(vec!["categoryAxis", "seriesAxis", "valueAxis", "format", "dataLabels", "title"])
			.imports(vec!["crate::drives::*"])
			.api_client_links(vec![
				ApiClientLinkSettings(Some("WorksheetsChartsAxesApiClient"), vec![
					ApiClientLink::Struct("category_axis", "WorksheetsChartsAxesApiClient"),
					ApiClientLink::Struct("series_axis", "WorksheetsChartsLegendApiClient"),
					ApiClientLink::Struct("value_axis", "WorksheetsChartsSeriesApiClient"),
					ApiClientLink::Struct("formatting", "WorksheetsChartsFormatApiClient"),
					ApiClientLink::Struct("data_labels", "WorksheetsChartsDataLabelsApiClient"),
				])
			])
			.build()
			.unwrap(),
		ResourceIdentity::WorksheetsChartsAxesCategoryAxis | ResourceIdentity::WorksheetsChartsAxesSeriesAxis
		| ResourceIdentity::WorksheetsChartsAxesValueAxis => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.trim_path_start("/drives/{drive-id}/items/{driveItem-id}/workbook/worksheets/{workbookWorksheet-id}/charts/{workbookChart-id}/axes")
			.filter_path(vec!["format", "title"])
			.imports(vec!["crate::drives::*"])
			.api_client_links(vec![
				ApiClientLinkSettings(None, vec![
					ApiClientLink::Struct("formatting", "WorksheetsChartsFormatApiClient"),
					ApiClientLink::Struct("title", "WorksheetsChartsTitleApiClient"),
				])
			])
			.build()
			.unwrap(),
		ResourceIdentity::WorksheetsChartsLegend | ResourceIdentity::WorksheetsChartsSeries
		| ResourceIdentity::WorksheetsChartsFormat => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.filter_path(vec!["format"])
			.trim_path_start("/drives/{drive-id}/items/{driveItem-id}/workbook/worksheets/{workbookWorksheet-id}/charts/{workbookChart-id}")
			.build()
			.unwrap(),
		ResourceIdentity::WorksheetsChartsDataLabels => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.trim_path_start("/drives/{drive-id}/items/{driveItem-id}/workbook/worksheets/{workbookWorksheet-id}/charts/{workbookChart-id}")
			.filter_path(vec!["format"])
			.build()
			.unwrap(),
		ResourceIdentity::WorksheetsChartsTitle => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.trim_path_start("/drives/{drive-id}/items/{driveItem-id}/workbook/worksheets/{workbookWorksheet-id}/charts/{workbookChart-id}")
			.filter_path(vec!["format"])
			.build()
			.unwrap(),




		// Service Principals
		ResourceIdentity::ServicePrincipals => WriteConfiguration::builder(resource_identity)
			.filter_path(vec!["owners", "transitiveMemberOf", "oauth2PermissionGrants", "memberOf", "createdObjects", "ownedObjects"])
			.children(vec![get_write_configuration(ResourceIdentity::ServicePrincipalsOwners)])
			.build()
			.unwrap(),
		ResourceIdentity::ServicePrincipalsOwners => WriteConfiguration::second_level_builder(ResourceIdentity::ServicePrincipals, resource_identity)
			.trim_path_start("/servicePrincipals/{servicePrincipal-id}")
			.build().unwrap(),


		// Groups
		ResourceIdentity::Groups => WriteConfiguration::builder(resource_identity)
			.filter_path(get_groups_filter())
			.children(map_write_config(vec![
				ResourceIdentity::Conversations, ResourceIdentity::Threads, ResourceIdentity::ThreadsPosts,
				ResourceIdentity::GroupsOwners, ResourceIdentity::GroupsTeam, ResourceIdentity::TransitiveMembers,
				ResourceIdentity::MembersWithLicenseErrors
			]))
			.build()
			.unwrap(),
		ResourceIdentity::TransitiveMembers => WriteConfiguration::second_level_builder(ResourceIdentity::Groups, resource_identity)
			.trim_path_start("/groups/{group-id}")
			.build().unwrap(),
		ResourceIdentity::MembersWithLicenseErrors => WriteConfiguration::second_level_builder(ResourceIdentity::Groups, resource_identity)
			.trim_path_start("/groups/{group-id}")
			.build().unwrap(),
		ResourceIdentity::GroupsTeam => WriteConfiguration::second_level_builder(ResourceIdentity::Groups, resource_identity)
			.trim_path_start("/groups/{group-id}")
			.filter_path(vec!["primaryChannel", "channels", "tags", "schedule", "members"])
			.build().unwrap(),
		ResourceIdentity::Threads =>  WriteConfiguration::second_level_builder(ResourceIdentity::Groups, resource_identity)
			.trim_path_start("/groups/{group-id}")
			.filter_path(vec![
				"posts",
				"multiValueExtendedProperties",
				"singleValueExtendedProperties",
			])
			.build().unwrap(),
		ResourceIdentity::GroupsOwners =>  WriteConfiguration::second_level_builder(ResourceIdentity::Groups, resource_identity)
			.trim_path_start("/groups/{group-id}")
			.modifier_name("groupsOwners")
			.build().unwrap(),
		ResourceIdentity::Conversations => WriteConfiguration::second_level_builder(ResourceIdentity::Groups, resource_identity)
			.trim_path_start("/groups/{group-id}")
			.filter_path(vec![
				"conversations/{{RID}}/threads",
				"posts",
				"threads",
				"multiValueExtendedProperties",
				"singleValueExtendedProperties",
			])
			.build().unwrap(),
		ResourceIdentity::ThreadsPosts => WriteConfiguration::second_level_builder(ResourceIdentity::Groups, resource_identity
		).filter_path(vec!["inReplyTo", "/multiValueExtendedProperties", "singleValueExtendedProperties"])
			.trim_path_start("/groups/{group-id}/threads/{conversationThread-id}")
			.build().unwrap(),


		ResourceIdentity::Identity => WriteConfiguration::builder(resource_identity)
			.replace_operation_map(resource_identity.exact_camel_case())
			.filter_path(vec!["identityProviders", "identityGovernance", "identityProtection"])
			.build()
			.unwrap(),


		// Education
		ResourceIdentity::EducationClasses |
		ResourceIdentity::EducationSchools |
		ResourceIdentity::EducationMe |
		ResourceIdentity::EducationUsers => WriteConfiguration::second_level_builder(ResourceIdentity::Education, resource_identity)
			.trim_path_start("/education")
			.filter_path(vec!["assignments"])
			.build()
			.unwrap(),
		ResourceIdentity::EducationAssignments => WriteConfiguration::second_level_builder(ResourceIdentity::Education, resource_identity)
			.trim_path_start("/education/me")
			.filter_path(vec!["submissions"])
			.build()
			.unwrap(),
		ResourceIdentity::EducationAssignmentsSubmissions => WriteConfiguration::second_level_builder(ResourceIdentity::Education, resource_identity)
			.trim_path_start("/education/me/assignments/{educationAssignment-id}")
			.build()
			.unwrap(),
		ResourceIdentity::Education => WriteConfiguration::builder(resource_identity)
			.filter_path(vec!["me", "classes", "schools", "users"])
			.children(vec![
				get_write_configuration(ResourceIdentity::EducationMe),
				get_write_configuration(ResourceIdentity::EducationUsers),
				get_write_configuration(ResourceIdentity::EducationClasses),
				get_write_configuration(ResourceIdentity::EducationSchools),
				get_write_configuration(ResourceIdentity::EducationAssignments),
				get_write_configuration(ResourceIdentity::EducationAssignmentsSubmissions)
			])
			.build()
			.unwrap(),


		// Onenote
		ResourceIdentity::Onenote => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}")
			.filter_path(vec!["sectionGroups", "sections", "notebooks", "pages"])
			.build()
			.unwrap(),
		ResourceIdentity::OnenoteSections => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}/onenote")
			.filter_path(vec!["pages"])
			.build()
			.unwrap(),
		ResourceIdentity::OnenoteSectionGroups => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}/onenote")
			.filter_path(vec!["sections", "pages"])
			.build()
			.unwrap(),
		ResourceIdentity::OnenoteNotebooks => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}/onenote")
			.filter_path(vec!["sectionGroups", "sections", "pages"])
			.build()
			.unwrap(),
		ResourceIdentity::OnenotePages => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}/onenote")
			.build()
			.unwrap(),


		// Users
		ResourceIdentity::EventsInstances=> WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.filter_path(vec!["attachments"])
			.trim_path_start("/users/{user-id}/events/{event-id}")
			.build()
			.unwrap(),
		ResourceIdentity::Events => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.filter_path(vec!["attachments", "instances"])
			.trim_path_start("/users/{user-id}")
			.build()
			.unwrap(),
		ResourceIdentity::UsersMessages => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.filter_path(vec!["attachments"])
			.trim_path_start("/users/{user-id}")
			.build()
			.unwrap(),
		ResourceIdentity::UsersAttachments => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}")
			.build()
			.unwrap(),
		ResourceIdentity::ChildFolders => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.filter_path(vec!["contacts", "messages", "singleValueExtendedProperties", "multiValueExtendedProperties"])
			.trim_path_start("/users/{user-id}/mailFolders/{mailFolder-id}")
			.build()
			.unwrap(),
		ResourceIdentity::ContactFolders => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.filter_path(vec!["childFolders", "contacts", "singleValueExtendedProperties", "multiValueExtendedProperties"])
			.trim_path_start("/users/{user-id}")
			.build()
			.unwrap(),
		ResourceIdentity::MailFolders => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.filter_path(vec!["childFolders", "messages", "singleValueExtendedProperties", "multiValueExtendedProperties"])
			.trim_path_start("/users/{user-id}")
			.build()
			.unwrap(),
		ResourceIdentity::Solutions => WriteConfiguration::builder(resource_identity)
			.filter_path(vec!["bookingBusinesses", "virtualEvents", "bookingCurrencies"])
			.children(vec![
				get_write_configuration(ResourceIdentity::BookingBusinesses),
				get_write_configuration(ResourceIdentity::Appointments),
				get_write_configuration(ResourceIdentity::Services),
				get_write_configuration(ResourceIdentity::CustomQuestions),
				get_write_configuration(ResourceIdentity::Customers),
				get_write_configuration(ResourceIdentity::StaffMembers),
				get_write_configuration(ResourceIdentity::VirtualEvents),
				get_write_configuration(ResourceIdentity::VirtualEventsEvents),
				get_write_configuration(ResourceIdentity::VirtualEventsWebinars),
				get_write_configuration(ResourceIdentity::VirtualEventsSessions),
			])
			.build()
			.unwrap(),
		ResourceIdentity::VirtualEvents => WriteConfiguration::second_level_builder(ResourceIdentity::Solutions, resource_identity)
			.trim_path_start("/solutions")
			.filter_path(vec!["sessions", "webinars", "events"])
			.build().unwrap(),
		ResourceIdentity::VirtualEventsEvents => WriteConfiguration::second_level_builder(ResourceIdentity::Solutions, resource_identity)
			.trim_path_start("/solutions/virtualEvents")
			.filter_path(vec!["sessions", "webinars"])
			.build().unwrap(),
		ResourceIdentity::VirtualEventsWebinars => WriteConfiguration::second_level_builder(ResourceIdentity::Solutions, resource_identity)
			.trim_path_start("/solutions/virtualEvents")
			.filter_path(vec!["sessions", "events"])
			.build().unwrap(),
		ResourceIdentity::VirtualEventsSessions => WriteConfiguration::second_level_builder(ResourceIdentity::Solutions, resource_identity)
			.trim_path_start("/solutions/virtualEvents/events/{virtualEvent-id}")
			.build().unwrap(),
		ResourceIdentity::BookingBusinesses => WriteConfiguration::second_level_builder(ResourceIdentity::Solutions, resource_identity)
			.trim_path_start("/solutions")
			.filter_path(vec!["appointments", "calendarView", "customQuestions", "customers", "services", "staffMembers"])
			.build()
			.unwrap(),
		ResourceIdentity::Appointments => WriteConfiguration::second_level_builder(ResourceIdentity::Solutions, resource_identity)
			.trim_path_start("/solutions/bookingBusinesses/{bookingBusiness-id}")
			.build()
			.unwrap(),
		ResourceIdentity::Services => WriteConfiguration::second_level_builder(ResourceIdentity::Solutions, resource_identity)
			.trim_path_start("/solutions/bookingBusinesses/{bookingBusiness-id}")
			.build()
			.unwrap(),
		ResourceIdentity::CustomQuestions => WriteConfiguration::second_level_builder(ResourceIdentity::Solutions, resource_identity)
			.trim_path_start("/solutions/bookingBusinesses/{bookingBusiness-id}")
			.build()
			.unwrap(),
		ResourceIdentity::Customers=> WriteConfiguration::second_level_builder(ResourceIdentity::Solutions, resource_identity)
			.trim_path_start("/solutions/bookingBusinesses/{bookingBusiness-id}")
			.build()
			.unwrap(),
		ResourceIdentity::StaffMembers => WriteConfiguration::second_level_builder(ResourceIdentity::Solutions, resource_identity)
			.trim_path_start("/solutions/bookingBusinesses/{bookingBusiness-id}")
			.build()
			.unwrap(),
		_ => WriteConfiguration::builder(resource_identity)
			.build()
			.unwrap(),
	}
}
