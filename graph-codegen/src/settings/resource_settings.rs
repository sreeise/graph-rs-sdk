use crate::api_types::{
    MethodMacro, ModFile, ModWriteConfiguration, WriteConfiguration, WriteConfigurationBuilder,
};
use crate::macros::OpenApiParser;
use crate::openapi::OpenApi;
use crate::settings::{ApiClientLink, ApiClientLinkSettings};
use from_as::*;
use graph_core::resource::ResourceIdentity;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::io::{Read, Write};

fn get_users_api_client_links(resource_identity: ResourceIdentity) -> Vec<ApiClientLinkSettings> {
    let name = {
        if resource_identity.eq(&ResourceIdentity::Users) {
            Some("UsersIdApiClient".into())
        } else {
            Some("MeApiClient".into())
        }
    };

    vec![ApiClientLinkSettings(
        name,
        vec![
            ApiClientLink::Struct("chats".into(), "ChatsApiClient".into()),
            ApiClientLink::StructId("chat".into(), "ChatsIdApiClient".into()),
            // Users and Me
            ApiClientLink::Struct(
                "agreement_acceptances".into(),
                "AgreementAcceptancesApiClient".into(),
            ),
            ApiClientLink::StructId(
                "agreement_acceptance".into(),
                "AgreementAcceptancesIdApiClient".into(),
            ),
            ApiClientLink::Struct("default_calendar".into(), "DefaultCalendarApiClient".into()),
            ApiClientLink::Struct("calendars".into(), "CalendarsApiClient".into()),
            ApiClientLink::StructId("calendar".into(), "CalendarsIdApiClient".into()),
            ApiClientLink::Struct("calendar_groups".into(), "CalendarGroupsApiClient".into()),
            ApiClientLink::StructId("calendar_group".into(), "CalendarGroupsIdApiClient".into()),
            ApiClientLink::Struct("calendar_views".into(), "CalendarViewApiClient".into()),
            ApiClientLink::StructId("calendar_view".into(), "CalendarViewIdApiClient".into()),
            ApiClientLink::Struct("messages".into(), "UsersMessagesApiClient".into()),
            ApiClientLink::StructId("message".into(), "UsersMessagesIdApiClient".into()),
            //ApiClientLink::Struct("agreement_acceptances".into(), "AgreementAcceptancesApiClient".into()),
            ApiClientLink::Struct(
                "app_role_assignments".into(),
                "AppRoleAssignmentsApiClient".into(),
            ),
            ApiClientLink::StructId(
                "app_role_assignment".into(),
                "AppRoleAssignmentsIdApiClient".into(),
            ),
            ApiClientLink::Struct("authentication".into(), "AuthenticationApiClient".into()),
            ApiClientLink::Struct("channels".into(), "ChannelsApiClient".into()),
            ApiClientLink::StructId("channel".into(), "ChannelsIdApiClient".into()),
            ApiClientLink::Struct("contact_folders".into(), "ContactFoldersApiClient".into()),
            ApiClientLink::StructId("contact_folder".into(), "ContactFoldersIdApiClient".into()),
            ApiClientLink::Struct("contacts".into(), "ContactsApiClient".into()),
            ApiClientLink::StructId("contact".into(), "ContactsIdApiClient".into()),
            ApiClientLink::Struct("joined_teams".into(), "JoinedTeamsApiClient".into()),
            ApiClientLink::StructId("joined_team".into(), "JoinedTeamsIdApiClient".into()),
            ApiClientLink::Struct("insights".into(), "InsightsApiClient".into()),
            ApiClientLink::Struct("license_details".into(), "LicenseDetailsApiClient".into()),
            ApiClientLink::StructId("license_detail".into(), "LicenseDetailsIdApiClient".into()),
            ApiClientLink::Struct("followed_sites".into(), "FollowedSitesApiClient".into()),
            ApiClientLink::Struct(
                "managed_app_registrations".into(),
                "ManagedAppRegistrationsApiClient".into(),
            ),
            ApiClientLink::StructId(
                "managed_app_registration".into(),
                "ManagedAppRegistrationsIdApiClient".into(),
            ),
            ApiClientLink::Struct(
                "scoped_role_member_of".into(),
                "ScopedRoleMemberOfApiClient".into(),
            ),
            ApiClientLink::StructId(
                "scoped_role_member_of_id".into(),
                "ScopedRoleMemberOfIdApiClient".into(),
            ),
            ApiClientLink::Struct("teamwork".into(), "TeamworkApiClient".into()),
            ApiClientLink::Struct(
                "inference_classification".into(),
                "InferenceClassificationApiClient".into(),
            ),
            ApiClientLink::Struct("mail_folders".into(), "MailFoldersApiClient".into()),
            ApiClientLink::StructId("mail_folder".into(), "MailFoldersIdApiClient".into()),
            ApiClientLink::Struct("activities".into(), "ActivitiesApiClient".into()),
            ApiClientLink::StructId("activity".into(), "ActivitiesIdApiClient".into()),
            ApiClientLink::Struct(
                "device_management_troubleshooting_events".into(),
                "DeviceManagementTroubleshootingEventsApiClient".into(),
            ),
            ApiClientLink::StructId(
                "device_management_troubleshooting_event".into(),
                "DeviceManagementTroubleshootingEventsIdApiClient".into(),
            ),
            ApiClientLink::Struct("extensions".into(), "ExtensionsApiClient".into()),
            ApiClientLink::StructId("extension".into(), "ExtensionsIdApiClient".into()),
            ApiClientLink::Struct("todo".into(), "TodoApiClient".into()),
            ApiClientLink::Struct("created_objects".into(), "CreatedObjectsApiClient".into()),
            ApiClientLink::StructId("created_object".into(), "CreatedObjectsIdApiClient".into()),
            ApiClientLink::Struct(
                "transitive_member_of".into(),
                "TransitiveMemberOfApiClient".into(),
            ),
            ApiClientLink::StructId(
                "transitive_member_of_id".into(),
                "TransitiveMemberOfIdApiClient".into(),
            ),
            ApiClientLink::Struct("direct_reports".into(), "DirectReportsApiClient".into()),
            ApiClientLink::StructId("direct_report".into(), "DirectReportsIdApiClient".into()),
            ApiClientLink::Struct("managed_devices".into(), "ManagedDevicesApiClient".into()),
            ApiClientLink::StructId("managed_device".into(), "ManagedDevicesIdApiClient".into()),
            ApiClientLink::Struct("events".into(), "EventsApiClient".into()),
            ApiClientLink::StructId("event".into(), "EventsIdApiClient".into()),
            ApiClientLink::Struct("online_meetings".into(), "OnlineMeetingsApiClient".into()),
            ApiClientLink::StructId("online_meeting".into(), "OnlineMeetingsIdApiClient".into()),
            ApiClientLink::Struct("photos".into(), "PhotosApiClient".into()),
            ApiClientLink::StructId("photo".into(), "PhotosIdApiClient".into()),
            ApiClientLink::Struct("outlook".into(), "OutlookApiClient".into()),
            ApiClientLink::Struct("presence".into(), "PresenceApiClient".into()),
            ApiClientLink::Struct("planner".into(), "PlannerApiClient".into()),
            ApiClientLink::Struct(
                "registered_devices".into(),
                "RegisteredDevicesApiClient".into(),
            ),
            ApiClientLink::StructId(
                "registered_device".into(),
                "RegisteredDevicesIdApiClient".into(),
            ),
            ApiClientLink::Struct("owned_devices".into(), "OwnedDevicesApiClient".into()),
            ApiClientLink::StructId("owned_device".into(), "OwnedDevicesIdApiClient".into()),
            ApiClientLink::Struct("owned_objects".into(), "OwnedObjectsApiClient".into()),
            ApiClientLink::StructId("owned_object".into(), "OwnedObjectsIdApiClient".into()),
            ApiClientLink::Struct("member_of".into(), "MemberOfApiClient".into()),
            ApiClientLink::StructId("member_of_id".into(), "MemberOfIdApiClient".into()),
            ApiClientLink::Struct("onenote".into(), "OnenoteApiClient".into()),
            ApiClientLink::Struct("schedule".into(), "ScheduleApiClient".into()),
            ApiClientLink::Struct("settings".into(), "SettingsApiClient".into()),
            ApiClientLink::Struct(
                "oauth2_permission_grants".into(),
                "Oauth2PermissionGrantsApiClient".into(),
            ),
            ApiClientLink::StructId(
                "oauth2_permission_grant".into(),
                "Oauth2PermissionGrantsIdApiClient".into(),
            ),
        ],
    )]
}

#[derive(
    Builder, Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, FromFile, AsFile, Hash,
)]
#[builder(
    pattern = "mutable",
    derive(Debug, Eq, PartialEq, Serialize, Deserialize),
    setter(into, strip_option),
    default
)]
pub struct ResourceSettings {
    pub path_name: String,
    pub ri: ResourceIdentity,
    pub imports: Vec<String>,
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
				.imports(vec!["crate::service_principals::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("ApplicationsIdApiClient".into()), vec![
							ApiClientLink::Struct("owners".into(), "ServicePrincipalsOwnersApiClient".into()),
							ApiClientLink::StructId("owner".into(), "ServicePrincipalsOwnersIdApiClient".into()),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::ServicePrincipals => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::service_principals::*".into(), "crate::users::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("ServicePrincipalsIdApiClient".into()), vec![
							ApiClientLink::Struct("owners".into(), "ServicePrincipalsOwnersApiClient".into()),
							ApiClientLink::StructId("owner".into(), "ServicePrincipalsOwnersIdApiClient".into()),
							ApiClientLink::Struct("transitive_member_of".into(), "TransitiveMemberOfApiClient".into()),
							ApiClientLink::Struct("member_of".into(), "MemberOfApiClient".into()),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::AuthenticationMethodsPolicy => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::authentication_method_configurations::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						None, vec![
							ApiClientLink::Struct("authentication_method_configurations".into(), "AuthenticationMethodConfigurationsApiClient".into()),
							ApiClientLink::StructId("authentication_method_configuration".into(), "AuthenticationMethodConfigurationsIdApiClient".into()),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::AccessPackages =>
				ResourceSettings {
					path_name: path_name.into(),
					ri,
					imports: vec!["crate::identity_governance::{AssignmentPoliciesApiClient, AssignmentPoliciesIdApiClient}".into()],
					api_client_links: vec![
						ApiClientLinkSettings(
							Some("AccessPackagesIdApiClient".into()),
							vec![
								ApiClientLink::Resource(
									"assignment_policies".into(),
									"AssignmentPoliciesApiClient".into(),
									ResourceIdentity::AssignmentPolicies
								),
								ApiClientLink::ResourceId(
									"assignment_policy".into(),
									"AssignmentPoliciesIdApiClient".into(),
									ResourceIdentity::AssignmentPolicies
								)
							]
						)
					],
				},
			ResourceIdentity::AccessReviews =>
				ResourceSettings {
					path_name: path_name.into(),
					ri,
					imports: vec!["crate::identity_governance::{AccessReviewsDefinitionsApiClient, AccessReviewsDefinitionsIdApiClient}".into()],
					api_client_links: vec![
						ApiClientLinkSettings(
							None,
							vec![
								ApiClientLink::Struct(
									"definitions".into(),
									"AccessReviewsDefinitionsApiClient".into(),
								),
								ApiClientLink::StructId(
									"definition".into(),
									"AccessReviewsDefinitionsIdApiClient".into(),
								)
							]
						)
					],
				},
			ResourceIdentity::AccessReviewsDefinitions =>
				ResourceSettings {
					path_name: path_name.into(),
					ri,
					imports: vec![
						"crate::identity_governance::{AccessReviewsDefinitionsInstancesApiClient, AccessReviewsDefinitionsInstancesIdApiClient}".into()
					],
					api_client_links: vec![
						ApiClientLinkSettings(
							Some("AccessReviewsDefinitionsIdApiClient".to_string()),
							vec![
								ApiClientLink::Resource(
									"instances".into(),
									"AccessReviewsDefinitionsInstancesApiClient".into(),
									ResourceIdentity::AccessReviewsDefinitionsInstances
								),
								ApiClientLink::ResourceId(
									"instance".into(),
									"AccessReviewsDefinitionsInstancesIdApiClient".into(),
									ResourceIdentity::AccessReviewsDefinitionsInstances
								)
							]
						)
					],
				},
			ResourceIdentity::AccessReviewsDefinitionsInstances =>
				ResourceSettings {
					path_name: path_name.into(),
					ri,
					imports: vec![
						"crate::identity_governance::{AccessReviewsDefinitionsInstancesStagesApiClient, AccessReviewsDefinitionsInstancesStagesIdApiClient}".into()
					],
					api_client_links: vec![
						ApiClientLinkSettings(
							Some("AccessReviewsDefinitionsInstancesIdApiClient".to_string()),
							vec![
								ApiClientLink::Resource(
									"stages".into(),
									"AccessReviewsDefinitionsInstancesStagesApiClient".into(),
									ResourceIdentity::AccessReviewsDefinitionsInstancesStages
								),
								ApiClientLink::ResourceId(
									"stage".into(),
									"AccessReviewsDefinitionsInstancesStagesIdApiClient".into(),
									ResourceIdentity::AccessReviewsDefinitionsInstancesStages
								)
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
					imports: vec!["crate::communications::*".into()],
					api_client_links: vec![
						ApiClientLinkSettings(
							Some("CallRecordsIdApiClient".into()),
							vec![
								ApiClientLink::Resource("sessions".into(), "CallRecordsSessionsApiClient".into(), ResourceIdentity::CallRecordsSessions),
								ApiClientLink::ResourceId("session".into(), "CallRecordsSessionsIdApiClient".into(), ResourceIdentity::CallRecordsSessions)
							]
						)
					],
				},
			ResourceIdentity::Chats => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::chats::*".into(), "crate::teams::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
							Some("ChatsIdApiClient".into()),
							vec![
								ApiClientLink::Struct("messages".into(), "ChatsMessagesApiClient".into()),
								ApiClientLink::StructId("message".into(), "ChatsMessagesIdApiClient".into()),
								ApiClientLink::Struct("members".into(), "TeamsMembersApiClient".into()),
								ApiClientLink::Struct("member".into(), "TeamsMembersIdApiClient".into()),
							]
						)
				]).build().unwrap(),
			ResourceIdentity::ChatsMessages => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::chats::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
							Some("ChatsMessagesIdApiClient".into()),
							vec![
								ApiClientLink::Struct("replies".into(), "ChatsMessagesRepliesApiClient".into()),
								ApiClientLink::StructId("reply".into(), "ChatsMessagesRepliesIdApiClient".into()),
							]
						)
				]).build().unwrap(),
			ResourceIdentity::Channels => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::teams::*".into(), "crate::chats::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
							Some("ChannelsIdApiClient".into()),
							vec![
								ApiClientLink::Struct("messages".into(), "ChatsMessagesApiClient".into()),
								ApiClientLink::StructId("message".into(), "ChatsMessagesIdApiClient".into()),
								ApiClientLink::Struct("shared_with_teams".into(), "SharedWithTeamsApiClient".into()),
								ApiClientLink::StructId("shared_with_team".into(), "SharedWithTeamsIdApiClient".into()),
								ApiClientLink::Struct("members".into(), "TeamsMembersApiClient".into()),
								ApiClientLink::StructId("member".into(), "TeamsMembersIdApiClient".into()),
							]
						)
				]).build().unwrap(),
			ResourceIdentity::Communications =>
				ResourceSettings {
					path_name: path_name.into(),
					ri,
					imports: vec![
						"crate::communications::{call_records::CallRecordsApiClient, call_records::CallRecordsIdApiClient, calls::CallsApiClient, calls::CallsIdApiClient}".to_string()
					],
					api_client_links: vec![
						ApiClientLinkSettings(
							None,
							vec![
								ApiClientLink::Resource("call_records".into(), "CallRecordsApiClient".into(), ResourceIdentity::CallRecords),
								ApiClientLink::ResourceId(
									"call_record".into(),
									"CallRecordsIdApiClient".into(),
									ResourceIdentity::CallRecords
								),
								ApiClientLink::Resource("calls".into(), "CallsApiClient".into(), ResourceIdentity::Calls),
								ApiClientLink::ResourceId("call".into(), "CallsIdApiClient".into(), ResourceIdentity::Calls)
							]
						)
					],
				},
			ResourceIdentity::ConnectedOrganizations => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::identity_governance::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("ConnectedOrganizationsIdApiClient".into()), vec![
						ApiClientLink::Struct("external_sponsors".into(), "ConnectedOrganizationsExternalSponsorsApiClient".into()),
						ApiClientLink::Struct("internal_sponsors".into(), "ConnectedOrganizationsInternalSponsorsApiClient".into()),
					])
				])
				.build()
				.unwrap(),


			// Device App Management
			ResourceIdentity::ManagedEBooks =>
				ResourceSettings::builder(path_name, ri)
					.imports(vec!["crate::device_app_management::*".into()])
					.api_client_links(vec![
						ApiClientLinkSettings(
							None, 							vec![
								ApiClientLink::Struct("device_states".into(), "ManagedEBooksDeviceStatesApiClient".into()),
								ApiClientLink::StructId("device_state".into(), "ManagedEBooksDeviceStatesIdApiClient".into()),
								ApiClientLink::Struct("user_state_summary".into(), "ManagedEBooksUserStateSummaryApiClient".into()),
								ApiClientLink::StructId("user_state_summary_id".into(), "ManagedEBooksUserStateSummaryIdApiClient".into()),
							]
						),
					]).build().unwrap(),
			ResourceIdentity::ManagedAppRegistrations =>
				ResourceSettings::builder(path_name, ri)
					.imports(vec!["crate::device_app_management::*".into()])
					.api_client_links(vec![
						ApiClientLinkSettings(
							None, 							vec![
								ApiClientLink::Struct("intended_policies".into(), "ManagedAppRegistrationsIntendedPoliciesApiClient".into()),
								ApiClientLink::StructId("intended_policies_id".into(), "ManagedAppRegistrationsIntendedPoliciesIdApiClient".into()),
								ApiClientLink::Struct("applied_policies".into(), "ManagedAppRegistrationsAppliedPoliciesApiClient".into()),
								ApiClientLink::StructId("applied_policies_id".into(), "ManagedAppRegistrationsAppliedPoliciesIdApiClient".into()),
							]
						),
					]).build().unwrap(),
			ResourceIdentity::DeviceAppManagement =>
				ResourceSettings::builder(path_name, ri)
					.imports(vec!["crate::device_app_management::*".into()])
					.api_client_links(vec![
						ApiClientLinkSettings(
							None, vec![
								ApiClientLink::Struct("android_managed_app_protections".into(), "AndroidManagedAppProtectionsApiClient".into()),
								ApiClientLink::StructId("android_managed_app_protection".into(), "AndroidManagedAppProtectionsIdApiClient".into()),
								ApiClientLink::Struct("default_managed_app_protections".into(), "DefaultManagedAppProtectionsApiClient".into()),
								ApiClientLink::StructId("default_managed_app_protection".into(), "DefaultManagedAppProtectionsIdApiClient".into()),
								ApiClientLink::Struct("ios_managed_app_protections".into(), "IosManagedAppProtectionsApiClient".into()),
								ApiClientLink::StructId("ios_managed_app_protection".into(), "IosManagedAppProtectionsIdApiClient".into()),
								ApiClientLink::Struct("managed_app_registrations".into(), "ManagedAppRegistrationsApiClient".into()),
								ApiClientLink::StructId("managed_app_registration".into(), "ManagedAppRegistrationsIdApiClient".into()),
								ApiClientLink::Struct("managed_app_statuses".into(), "ManagedAppStatusesApiClient".into()),
								ApiClientLink::StructId("managed_app_statuses_id".into(), "ManagedAppStatusesIdApiClient".into()),
								ApiClientLink::Struct("mdm_windows_information_protection_policies".into(), "MdmWindowsInformationProtectionPoliciesApiClient".into()),
								ApiClientLink::StructId("mdm_windows_information_protection_policy".into(), "MdmWindowsInformationProtectionPoliciesIdApiClient".into()),
								ApiClientLink::Struct("managed_app_policies".into(), "ManagedAppPoliciesApiClient".into()),
								ApiClientLink::StructId("managed_app_policies_id".into(), "ManagedAppPoliciesIdApiClient".into()),
								ApiClientLink::Struct("mobile_app_categories".into(), "MobileAppCategoriesApiClient".into()),
								ApiClientLink::StructId("mobile_app_categories_id".into(), "MobileAppCategoriesIdApiClient".into()),
								ApiClientLink::Struct("managed_e_books".into(), "ManagedEBooksApiClient".into()),
								ApiClientLink::StructId("managed_e_book".into(), "ManagedEBooksIdApiClient".into()),
								ApiClientLink::Struct("mobile_app_configurations".into(), "MobileAppConfigurationsApiClient".into()),
								ApiClientLink::StructId("mobile_app_configuration".into(), "MobileAppConfigurationsIdApiClient".into()),
								ApiClientLink::Struct("mobile_apps".into(), "MobileAppsApiClient".into()),
								ApiClientLink::StructId("mobile_app".into(), "MobileAppsIdApiClient".into()),
								ApiClientLink::Struct("targeted_managed_app_configurations".into(), "TargetedManagedAppConfigurationsApiClient".into()),
								ApiClientLink::StructId("targeted_managed_app_configuration".into(), "TargetedManagedAppConfigurationsIdApiClient".into()),
								ApiClientLink::Struct("vpp_tokens".into(), "VppTokensApiClient".into()),
								ApiClientLink::StructId("vpp_token".into(), "VppTokensIdApiClient".into()),
								ApiClientLink::Struct("windows_information_protection_policies".into(), "WindowsInformationProtectionPoliciesApiClient".into()),
								ApiClientLink::StructId("windows_information_protection_policies_id".into(), "WindowsInformationProtectionPoliciesIdApiClient".into()),
							]
						),
					]).build().unwrap(),


			// Device Management
			ResourceIdentity::DeviceManagement =>
				ResourceSettings::builder(path_name, ri)
					.imports(vec!["crate::device_management::*".into()])
					.api_client_links(vec![
						ApiClientLinkSettings(
							None, vec![
								ApiClientLink::Struct("device_configurations".into(), "DeviceConfigurationsApiClient".into()),
								ApiClientLink::StructId("device_configuration".into(), "DeviceConfigurationsIdApiClient".into()),
								ApiClientLink::Struct("device_enrollment_configurations".into(), "DeviceEnrollmentConfigurationsApiClient".into()),
								ApiClientLink::StructId(
									"device_enrollment_configuration".into(),
									"DeviceEnrollmentConfigurationsIdApiClient".into()
								),
								ApiClientLink::Struct("managed_devices".into(), "DeviceManagementManagedDevicesApiClient".into()),
								ApiClientLink::StructId("managed_device".into(), "DeviceManagementManagedDevicesIdApiClient".into()),
								ApiClientLink::Struct("role_definitions".into(), "RoleDefinitionsApiClient".into()),
								ApiClientLink::StructId("role_definition".into(), "RoleDefinitionsIdApiClient".into()),
								ApiClientLink::Struct("terms_and_conditions".into(), "TermsAndConditionsApiClient".into()),
								ApiClientLink::StructId("terms_and_condition".into(), "TermsAndConditionsIdApiClient".into()),
								ApiClientLink::Struct("troubleshooting_events".into(), "TroubleshootingEventsApiClient".into()),
								ApiClientLink::StructId("troubleshooting_event".into(), "TroubleshootingEventsIdApiClient".into()),
								ApiClientLink::Struct("reports".into(), "DeviceManagementReportsApiClient".into()),
								ApiClientLink::Struct("device_compliance_policy_setting_state_summaries".into(), "DeviceCompliancePolicySettingStateSummariesApiClient".into()),
								ApiClientLink::StructId("device_compliance_policy_setting_state_summaries_id".into(), "DeviceCompliancePolicySettingStateSummariesIdApiClient".into()),
								ApiClientLink::StructId("windows_autopilot_device_identities".into(), "WindowsAutopilotDeviceIdentitiesApiClient".into()),
								ApiClientLink::StructId("windows_autopilot_device_identities_id".into(), "WindowsAutopilotDeviceIdentitiesIdApiClient".into()),
							]
						),
					]).build().unwrap(),


			ResourceIdentity::Drives =>
				ResourceSettings {
					path_name: path_name.to_string(),
					ri,
					imports: vec![
						"crate::drives::{DrivesListApiClient, DrivesItemsApiClient, DrivesItemsIdApiClient}".into()
					],
					api_client_links: vec![
						ApiClientLinkSettings(Some("DrivesIdApiClient".into()), vec![
							ApiClientLink::StructId("list".into(), "DrivesListApiClient".into()),
							ApiClientLink::StructId("items".into(), "DrivesItemsApiClient".into()),
							ApiClientLink::StructId("item".into(), "DrivesItemsIdApiClient".into()),
						])
					],
				},
			ResourceIdentity::DrivesList =>
				ResourceSettings {
					path_name: path_name.to_string(),
					ri,
					imports: vec![
						"crate::drives::{DrivesListContentTypesApiClient, DrivesListContentTypesIdApiClient, DrivesItemsApiClient, DrivesItemsIdApiClient}".into()
					],
					api_client_links: vec![
						ApiClientLinkSettings(Some("DrivesListApiClient".into()), vec![
							ApiClientLink::Struct("content_types".into(), "DrivesListContentTypesApiClient".into()),
							ApiClientLink::StructId("content_type".into(), "DrivesListContentTypesIdApiClient".into()),
							ApiClientLink::Struct("items".into(), "DrivesItemsApiClient".into()),
							ApiClientLink::StructId("item".into(), "DrivesItemsIdApiClient".into()),
						])
					],
				},
			ResourceIdentity::Education => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::education::*".into()])
				.api_client_links(vec![
						ApiClientLinkSettings(None, vec![
							ApiClientLink::Struct("classes".into(), "EducationClassesApiClient".into()),
							ApiClientLink::StructId("class".into(), "EducationClassesIdApiClient".into()),
							ApiClientLink::Struct("schools".into(), "EducationSchoolsApiClient".into()),
							ApiClientLink::StructId("school".into(), "EducationSchoolsIdApiClient".into()),
							ApiClientLink::Struct("me".into(), "EducationMeApiClient".into()),
							ApiClientLink::Struct("users".into(), "EducationUsersApiClient".into()),
							ApiClientLink::StructId("user".into(), "EducationUsersIdApiClient".into()),
						])
					]).build().unwrap(),
			ResourceIdentity::EducationAssignments => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::education::*".into()])
				.api_client_links(vec![
						ApiClientLinkSettings(Some("EducationAssignmentsIdApiClient".into()), vec![
							ApiClientLink::Struct("submissions".into(), "EducationAssignmentsSubmissionsApiClient".into()),
							ApiClientLink::StructId("submission".into(), "EducationAssignmentsSubmissionsIdApiClient".into()),
						])
					]).build().unwrap(),
			ResourceIdentity::EducationMe => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::education::*".into()])
				.api_client_links(vec![
						ApiClientLinkSettings(Some("EducationMeApiClient".into()), vec![
							ApiClientLink::Struct("assignments".into(), "EducationAssignmentsApiClient".into()),
							ApiClientLink::StructId("assignment".into(), "EducationAssignmentsIdApiClient".into()),
						])
					]).build().unwrap(),
			ResourceIdentity::EducationSchools => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::education::*".into()])
				.api_client_links(vec![
						ApiClientLinkSettings(Some("EducationSchoolsIdApiClient".into()), vec![
							ApiClientLink::Struct("assignments".into(), "EducationAssignmentsApiClient".into()),
							ApiClientLink::StructId("assignment".into(), "EducationAssignmentsIdApiClient".into()),
						])
					]).build().unwrap(),
			ResourceIdentity::EducationUsers => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::education::*".into()])
				.api_client_links(vec![
						ApiClientLinkSettings(Some("EducationUsersIdApiClient".into()), vec![
							ApiClientLink::Struct("assignments".into(), "EducationAssignmentsApiClient".into()),
							ApiClientLink::StructId("assignment".into(), "EducationAssignmentsIdApiClient".into()),
						])
					]).build().unwrap(),
			ResourceIdentity::EducationClasses => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::education::*".into()])
				.api_client_links(vec![
						ApiClientLinkSettings(Some("EducationClassesIdApiClient".into()), vec![
							ApiClientLink::Struct("assignments".into(), "EducationAssignmentsApiClient".into()),
							ApiClientLink::StructId("assignment".into(), "EducationAssignmentsIdApiClient".into()),
						])
					]).build().unwrap(),
			ResourceIdentity::EntitlementManagement =>
				ResourceSettings {
					path_name: path_name.to_string(),
					ri,
					imports: vec![
						"crate::identity_governance::*".to_string(),
					],
					api_client_links: vec![
						ApiClientLinkSettings(
							None,
							vec![
								ApiClientLink::Struct("access_packages".into(), "AccessPackagesApiClient".into()),
								ApiClientLink::StructId(
									"access_package".into(),
									"AccessPackagesIdApiClient".into()
								),
								ApiClientLink::Struct(
									"access_package_assignment_approvals".into(),
									"AccessPackageAssignmentApprovalsApiClient".into()
								),
								ApiClientLink::StructId(
									"access_package_assignment_approval".into(),
									"AccessPackageAssignmentApprovalsIdApiClient".into(),
								),
								ApiClientLink::Struct(
									"assignment_policies".into(),
									"AssignmentPoliciesApiClient".into()
								),
								ApiClientLink::StructId(
									"assignment_policy".into(),
									"AssignmentPoliciesIdApiClient".into()
								),
								ApiClientLink::Struct(
									"assignments".into(),
									"EntitlementManagementAssignmentsApiClient".into()
								),
								ApiClientLink::StructId(
									"assignment".into(),
									"EntitlementManagementAssignmentsIdApiClient".into()
								),
								ApiClientLink::Struct(
									"catalogs".into(),
									"EntitlementManagementCatalogsApiClient".into(),
								),
								ApiClientLink::StructId(
									"catalog".into(),
									"EntitlementManagementCatalogsIdApiClient".into(),
								),
								ApiClientLink::Struct(
									"assignment_requests".into(),
									"AssignmentRequestsApiClient".into(),
								),
								ApiClientLink::StructId(
									"assignment_request".into(),
									"AssignmentRequestsIdApiClient".into(),
								),
								ApiClientLink::Struct(
									"connected_organizations".into(),
									"ConnectedOrganizationsApiClient".into(),
								),
								ApiClientLink::StructId(
									"connected_organization".into(),
									"ConnectedOrganizationsIdApiClient".into(),
								)
							]
						)
					],
				},
			ResourceIdentity::EntitlementManagementCatalogs =>
				ResourceSettings {
					path_name: path_name.to_string(),
					ri,
					imports: vec!["crate::identity_governance::{AccessPackagesApiClient, AccessPackagesIdApiClient}".into()],
					api_client_links: vec![
						ApiClientLinkSettings(
							Some("EntitlementManagementCatalogsIdApiClient".into()),
							vec![
								ApiClientLink::Struct("access_packages".into(), "AccessPackagesApiClient".into()),
								ApiClientLink::StructId(
									"access_package".into(),
									"AccessPackagesIdApiClient".into()
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
						"crate::identity_governance::{AccessReviewsApiClient, AccessPackagesApiClient, AccessPackagesIdApiClient, EntitlementManagementApiClient}".to_string()
					],
					api_client_links: vec![
						ApiClientLinkSettings(
							None,
							vec![
								ApiClientLink::Resource("access_reviews".into(), "AccessReviewsApiClient".into(), ResourceIdentity::AccessReviews),
								ApiClientLink::Resource(
									"entitlement_management".into(),
									"EntitlementManagementApiClient".into(),
									ResourceIdentity::EntitlementManagement
								)
							]
						)
					],
				},
			ResourceIdentity::Events => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("EventsIdApiClient".into()),
						vec![
							ApiClientLink::Struct("instances".into(), "EventsInstancesApiClient".into()),
							ApiClientLink::StructId("instance".into(), "EventsInstancesIdApiClient".into()),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::CalendarView => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("CalendarViewIdApiClient".into()),
						vec![
							ApiClientLink::Struct("instances".into(), "EventsInstancesApiClient".into()),
							ApiClientLink::StructId("instance".into(), "EventsInstancesIdApiClient".into()),
							ApiClientLink::Struct("attachments".into(), "UsersAttachmentsApiClient".into()),
							ApiClientLink::StructId("attachment".into(), "UsersAttachmentsIdApiClient".into()),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::Calendars => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into(), "crate::extended_properties::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("CalendarsIdApiClient".into()),
						vec![
							ApiClientLink::Struct("events".into(), "EventsApiClient".into()),
							ApiClientLink::StructId("event".into(), "EventsIdApiClient".into()),
							ApiClientLink::Struct("calendar_views".into(), "CalendarViewApiClient".into()),
							ApiClientLink::StructId("calendar_view".into(), "CalendarViewIdApiClient".into()),
							ApiClientLink::Struct("extended_properties".into(), "ExtendedPropertiesApiClient".into())
						]
					),
				]).build().unwrap(),
			ResourceIdentity::DefaultCalendar => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into(), "crate::extended_properties::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("DefaultCalendarApiClient".into()),
						vec![
							ApiClientLink::Struct("events".into(), "EventsApiClient".into()),
							ApiClientLink::StructId("event".into(), "EventsIdApiClient".into()),
							ApiClientLink::Struct("calendar_views".into(), "CalendarViewApiClient".into()),
							ApiClientLink::StructId("calendar_view".into(), "CalendarViewIdApiClient".into()),
							ApiClientLink::Struct("extended_properties".into(), "ExtendedPropertiesApiClient".into()),
						]
					),
				]).build().unwrap(),

			ResourceIdentity::Directory => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::directory::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						None, vec![
							ApiClientLink::Struct("deleted_items".into(), "DeletedItemsApiClient".into()),
							ApiClientLink::StructId("deleted_item".into(), "DeletedItemsIdApiClient".into()),
							ApiClientLink::Struct("administrative_units".into(), "AdministrativeUnitsApiClient".into()),
							ApiClientLink::StructId("administrative_unit".into(), "AdministrativeUnitsIdApiClient".into()),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::AdministrativeUnits => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::directory::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("AdministrativeUnitsIdApiClient".into()), vec![
							ApiClientLink::Struct("members".into(), "DirectoryMembersApiClient".into()),
							ApiClientLink::StructId("member".into(), "DirectoryMembersIdApiClient".into()),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::DirectoryRoles => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::directory::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("DirectoryRolesIdApiClient".into()), vec![
							ApiClientLink::Struct("members".into(), "DirectoryMembersApiClient".into()),
							ApiClientLink::StructId("member".into(), "DirectoryMembersIdApiClient".into()),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::CalendarGroups => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("CalendarGroupsIdApiClient".into()),
						vec![
							ApiClientLink::Struct("calendars".into(), "CalendarsApiClient".into()),
							ApiClientLink::StructId("calendar".into(), "CalendarsIdApiClient".into()),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::Groups => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::groups::*".into(), "crate::users::*".into(), "crate::sites::*".into(), "crate::planner::*".into(),
							  "crate::group_lifecycle_policies::*".into()])
				.api_client_links(vec![
						ApiClientLinkSettings(
						Some("GroupsIdApiClient".into()),
						vec![
							ApiClientLink::Struct("group_lifecycle_policies".into(), "GroupLifecyclePoliciesApiClient".into()),
							ApiClientLink::StructId("conversation".into(), "ConversationsIdApiClient".into()),
							ApiClientLink::Struct("conversations".into(), "ConversationsApiClient".into()),
							ApiClientLink::StructId("thread".into(), "ThreadsIdApiClient".into()),
							ApiClientLink::Struct("threads".into(), "ThreadsApiClient".into()),
							ApiClientLink::Struct("onenote".into(), "OnenoteApiClient".into()),
							ApiClientLink::Struct("member_of".into(), "MemberOfApiClient".into()),
							ApiClientLink::StructId("member_of_id".into(), "MemberOfIdApiClient".into()),
							ApiClientLink::Struct("events".into(), "EventsApiClient".into()),
							ApiClientLink::StructId("event".into(), "EventsIdApiClient".into()),
							ApiClientLink::Struct("calendar_views".into(), "CalendarViewApiClient".into()),
							ApiClientLink::StructId("calendar_view".into(), "CalendarViewIdApiClient".into()),
							ApiClientLink::Struct("default_calendar".into(), "DefaultCalendarApiClient".into()),
							ApiClientLink::Struct("planner".into(), "PlannerApiClient".into()),
							ApiClientLink::Struct("sites".into(), "SitesApiClient".into()),
							ApiClientLink::StructId("site".into(), "SitesIdApiClient".into()),
							ApiClientLink::Struct("groups_team".into(), "GroupsTeamApiClient".into()),
						]
					)
				]).build().unwrap(),
			ResourceIdentity::Conversations => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::groups::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("GroupsIdApiClient".into()),
						vec![
							ApiClientLink::Struct("group_lifecycle_policies".into(), "GroupLifecyclePoliciesApiClient".into()),
							ApiClientLink::StructId("conversation".into(), "GroupsConversationsIdApiClient".into()),
							ApiClientLink::Struct("conversations".into(), "GroupsConversationsApiClient".into()),
							ApiClientLink::StructId("thread".into(), "GroupsThreadsIdApiClient".into()),
							ApiClientLink::Struct("threads".into(), "GroupsThreadsApiClient".into())
						]
					)
				]).build().unwrap(),
			ResourceIdentity::Threads => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::groups::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("ThreadsIdApiClient".into()),
						vec![
							ApiClientLink::Struct("post".into(), "ThreadsPostsIdApiClient".into()),
							ApiClientLink::Struct("posts".into(), "ThreadsPostsApiClient".into())
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
				.imports(vec!["crate::teams::*".into(), "crate::chats::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("PrimaryChannelApiClient".into()),
						vec![
							ApiClientLink::Struct("shared_with_teams".into(), "SharedWithTeamsApiClient".into()),
							ApiClientLink::StructId("shared_with_team".into(), "SharedWithTeamsIdApiClient".into()),
							ApiClientLink::Struct("messages".into(), "ChatsMessagesApiClient".into()),
							ApiClientLink::StructId("message".into(), "ChatsMessagesIdApiClient".into()),
							ApiClientLink::Struct("members".into(), "TeamsMembersApiClient".into()),
							ApiClientLink::Struct("member".into(), "TeamsMembersIdApiClient".into()),
						]
					)
				]).build().unwrap(),
			ResourceIdentity::Teams => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into(), "crate::teams::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("TeamsIdApiClient".into()),
						vec![
							ApiClientLink::Struct("primary_channel".into(), "PrimaryChannelApiClient".into()),
							ApiClientLink::Struct("channels".into(), "ChannelsApiClient".into()),
							ApiClientLink::StructId("channel".into(), "ChannelsIdApiClient".into()),
							ApiClientLink::Struct("tags".into(), "TeamsTagsApiClient".into()),
							ApiClientLink::StructId("tag".into(), "TeamsTagsIdApiClient".into()),
							ApiClientLink::Struct("schedule".into(), "ScheduleApiClient".into()),
							ApiClientLink::Struct("members".into(), "TeamsMembersApiClient".into()),
							ApiClientLink::Struct("member".into(), "TeamsMembersIdApiClient".into()),
						]
					)
				]).build().unwrap(),
			ResourceIdentity::TeamsTags => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::teams::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("TeamsTagsIdApiClient".into()),
						vec![
							ApiClientLink::Struct("members".into(), "TeamsMembersApiClient".into()),
							ApiClientLink::Struct("member".into(), "TeamsMembersIdApiClient".into()),
						]
					)
				]).build().unwrap(),
			ResourceIdentity::GroupsTeam => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into(), "crate::teams::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("TeamApiClient".into()),
						vec![
							ApiClientLink::Struct("primary_channel".into(), "PrimaryChannelApiClient".into()),
							ApiClientLink::Struct("channels".into(), "ChannelsApiClient".into()),
							ApiClientLink::StructId("channel".into(), "ChannelsIdApiClient".into()),
							ApiClientLink::Struct("tags".into(), "TeamsTagsApiClient".into()),
							ApiClientLink::StructId("tag".into(), "TeamsTagsIdApiClient".into()),
							ApiClientLink::Struct("schedule".into(), "ScheduleApiClient".into()),
							ApiClientLink::Struct("members".into(), "TeamsMembersApiClient".into()),
							ApiClientLink::Struct("member".into(), "TeamsMembersIdApiClient".into()),
						]
					)
				]).build().unwrap(),
			ResourceIdentity::Todo => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						None,
						vec![
							ApiClientLink::Struct("lists".into(), "TodoListsApiClient".into()),
							ApiClientLink::StructId("list".into(), "TodoListsIdApiClient".into()),
						]
					)
				]).build().unwrap(),
			ResourceIdentity::TodoLists => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("TodoListsIdApiClient".into()),
						vec![
							ApiClientLink::Struct("tasks".into(), "TodoListsTasksApiClient".into()),
							ApiClientLink::StructId("task".into(), "TodoListsTasksIdApiClient".into()),
						]
					)
				]).build().unwrap(),
			ResourceIdentity::TodoListsTasks => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("TodoListsIdApiClient".into()),
						vec![
							ApiClientLink::Struct("tasks".into(), "TodoListsTasksApiClient".into()),
							ApiClientLink::StructId("task".into(), "TodoListsTasksIdApiClient".into()),
						]
					)
				]).build().unwrap(),
			ResourceIdentity::JoinedTeams => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into(), "crate::teams::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(
						Some("JoinedTeamsIdApiClient".into()),
						vec![
							ApiClientLink::Struct("primary_channel".into(), "PrimaryChannelApiClient".into()),
							ApiClientLink::Struct("channels".into(), "ChannelsApiClient".into()),
							ApiClientLink::StructId("channel".into(), "ChannelsIdApiClient".into()),
							ApiClientLink::Struct("tags".into(), "TeamsTagsApiClient".into()),
							ApiClientLink::StructId("tag".into(), "TeamsTagsIdApiClient".into()),
							ApiClientLink::Struct("schedule".into(), "ScheduleApiClient".into()),
							ApiClientLink::Struct("members".into(), "TeamsMembersApiClient".into()),
							ApiClientLink::Struct("member".into(), "TeamsMembersIdApiClient".into()),
						]
					)
				]).build().unwrap(),
			ResourceIdentity::MailFolders => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into(), "crate::extended_properties::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("MailFoldersIdApiClient".into()), vec![
						ApiClientLink::Struct("child_folders".into(), "ChildFoldersApiClient".into()),
						ApiClientLink::StructId("child_folder".into(), "ChildFoldersIdApiClient".into()),
						ApiClientLink::Struct("extended_properties".into(), "ExtendedPropertiesApiClient".into()),
						ApiClientLink::Struct("messages".into(), "UsersMessagesApiClient".into()),
						ApiClientLink::StructId("messages_id".into(), "UsersMessagesIdApiClient".into()),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::ChildFolders => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into(), "crate::extended_properties::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("ChildFoldersIdApiClient".into()), vec![
						ApiClientLink::Struct("messages".into(), "UsersMessagesApiClient".into()),
						ApiClientLink::StructId("messages_id".into(), "UsersMessagesIdApiClient".into()),
						ApiClientLink::Struct("extended_properties".into(), "ExtendedPropertiesApiClient".into()),
						ApiClientLink::StructId("child_folder".into(), "ChildFoldersIdApiClient".into()),
					])
				]).build()
				.unwrap(),
			ResourceIdentity::ContactFolders => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into(), "crate::extended_properties::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("ContactFoldersIdApiClient".into()), vec![
						ApiClientLink::Struct("contacts".into(), "ContactsApiClient".into()),
						ApiClientLink::StructId("contact".into(), "ContactsIdApiClient".into()),
						ApiClientLink::Struct("child_folders".into(), "ChildFoldersApiClient".into()),
						ApiClientLink::StructId("child_folder".into(), "ChildFoldersIdApiClient".into()),
						ApiClientLink::Struct("extended_properties".into(), "ExtendedPropertiesApiClient".into()),
					])
				]).build()
				.unwrap(),
			ResourceIdentity::Planner => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::planner::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("PlannerApiClient".into()), vec![
						ApiClientLink::Struct("tasks".into(), "PlannerTasksApiClient".into()),
						ApiClientLink::StructId("task".into(), "PlannerTasksIdApiClient".into()),
						ApiClientLink::Struct("buckets".into(), "BucketsApiClient".into()),
						ApiClientLink::StructId("bucket".into(), "BucketsIdApiClient".into()),
						ApiClientLink::Struct("plans".into(), "PlansApiClient".into()),
						ApiClientLink::StructId("plan".into(), "PlansIdApiClient".into()),
					])
				]).build()
				.unwrap(),
			ResourceIdentity::Buckets => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::planner::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("BucketsIdApiClient".into()), vec![
						ApiClientLink::Struct("tasks".into(), "PlannerTasksApiClient".into()),
						ApiClientLink::StructId("task".into(), "PlannerTasksIdApiClient".into()),
					])
				]).build()
				.unwrap(),
			ResourceIdentity::Plans => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::planner::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("PlansIdApiClient".into()), vec![
						ApiClientLink::Struct("tasks".into(), "PlannerTasksApiClient".into()),
						ApiClientLink::StructId("task".into(), "PlannerTasksIdApiClient".into()),
						ApiClientLink::Struct("plans".into(), "PlansApiClient".into()),
						ApiClientLink::StructId("plan".into(), "PlansIdApiClient".into()),
					])
				]).build()
				.unwrap(),
			ResourceIdentity::Onenote => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("OnenoteApiClient".into()), vec![
						ApiClientLink::Struct("pages".into(), "OnenotePagesApiClient".into()),
						ApiClientLink::StructId("page".into(), "OnenotePagesIdApiClient".into()),
						ApiClientLink::Struct("sections".into(), "OnenoteSectionsApiClient".into()),
						ApiClientLink::StructId("section".into(), "OnenoteSectionsIdApiClient".into()),
						ApiClientLink::Struct("section_groups".into(), "OnenoteSectionGroupsApiClient".into()),
						ApiClientLink::StructId("section_group".into(), "OnenoteSectionGroupsIdApiClient".into()),
						ApiClientLink::Struct("notebooks".into(), "OnenoteNotebooksApiClient".into()),
						ApiClientLink::StructId("notebook".into(), "OnenoteNotebooksIdApiClient".into()),

					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::OnenoteNotebooks => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("OnenoteNotebooksIdApiClient".into()), vec![
						ApiClientLink::Struct("sections".into(), "OnenoteSectionsApiClient".into()),
						ApiClientLink::StructId("section".into(), "OnenoteSectionsIdApiClient".into()),
						ApiClientLink::Struct("section_groups".into(), "OnenoteSectionGroupsApiClient".into()),
						ApiClientLink::StructId("section_group".into(), "OnenoteSectionGroupsIdApiClient".into()),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::OnenoteSections => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("OnenoteSectionsIdApiClient".into()), vec![
						ApiClientLink::Struct("sections".into(), "OnenoteSectionsApiClient".into()),
						ApiClientLink::StructId("section".into(), "OnenoteSectionsIdApiClient".into()),
						ApiClientLink::Struct("pages".into(), "OnenotePagesApiClient".into()),
						ApiClientLink::StructId("page".into(), "OnenotePagesIdApiClient".into()),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::OnenoteSectionGroups => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("OnenoteSectionGroupsIdApiClient".into()), vec![
						ApiClientLink::Struct("sections".into(), "OnenoteSectionsApiClient".into()),
						ApiClientLink::StructId("section".into(), "OnenoteSectionsIdApiClient".into()),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::OnenotePages => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into()])
				.build()
				.unwrap(),
			ResourceIdentity::Teamwork => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::teamwork::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("TeamworkApiClient".into()), vec![
						ApiClientLink::Struct("deleted_teams".into(), "DeletedTeamsApiClient".into()),
						ApiClientLink::StructId("deleted_team".into(), "DeletedTeamsIdApiClient".into()),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::DeletedTeams => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("DeletedTeamsIdApiClient".into()), vec![
						ApiClientLink::Struct("channels".into(), "ChannelsApiClient".into()),
						ApiClientLink::StructId("channel".into(), "ChannelsIdApiClient".into()),
					])
				])
				.build()
				.unwrap(),
			// resource_api_client!(ListsApiClient, ListsIdApiClient, ResourceIdentity::Lists);
			ResourceIdentity::Sites => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into(), "crate::sites::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("SitesIdApiClient".into()), vec![
						ApiClientLink::Struct("lists".into(), "ListsApiClient".into()),
						ApiClientLink::StructId("list".into(), "ListsIdApiClient".into()),
						ApiClientLink::Struct("onenote".into(), "OnenoteApiClient".into()),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::UsersMessages => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into()])
				.api_client_links(vec![
					ApiClientLinkSettings(Some("UsersMessagesIdApiClient".into()), vec![
						ApiClientLink::Struct("attachments".into(), "UsersAttachmentsApiClient".into()),
						ApiClientLink::StructId("attachment".into(), "UsersAttachmentsIdApiClient".into()),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::Users => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into(), "crate::chats::*".into(), "crate::agreement_acceptances::*".into(),
							  "crate::planner::*".into(), "crate::oauth2_permission_grants::*".into(), "crate::teams::*".into()])
				.api_client_links(get_users_api_client_links(ri))
				.build()
				.unwrap(),
			ResourceIdentity::Me => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into(), "crate::chats::*".into(), "crate::agreement_acceptances::*".into(),
							  "crate::planner::*".into(), "crate::oauth2_permission_grants::*".into(), "crate::teams::*".into()])
				.api_client_links(get_users_api_client_links(ri))
				.build()
				.unwrap(),
			_ => ResourceSettings::default(path_name, ri),
		}
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile, Eq, PartialEq, Hash)]
pub struct ResourceSettingsMap(pub BTreeMap<ResourceIdentity, ResourceSettings>);

fn get_method_macros_with_same_fn_name() {
    let set = OpenApi::get_metadata_method_macros(
        WriteConfiguration::second_level_builder(
            ResourceIdentity::Users,
            ResourceIdentity::ChatsMessages,
        )
        .trim_path_start("/users/{user-id}/chats/{chat-id}")
        .filter_path(get_me_child_filters(ResourceIdentity::ChatsMessages))
        .build()
        .unwrap(),
    );

    let mut same_method_name_map: HashMap<String, VecDeque<MethodMacro>> = HashMap::new();

    for method_macro in set.iter() {
        let mut v = VecDeque::new();
        v.push_back(method_macro.clone());
        same_method_name_map
            .entry(method_macro.fn_name.clone())
            .and_modify(|mut v| v.push_back(method_macro.clone()))
            .or_insert(v);
    }

    for (_fn_name, method_macro) in same_method_name_map.iter() {
        if method_macro.len() > 1 {
            dbg!(method_macro);
        }
    }
}

pub fn get_me_filter() -> Vec<String> {
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
    .iter()
    .map(|s| s.to_string())
    .collect()
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

pub fn get_groups_filter() -> Vec<String> {
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
    .iter()
    .map(|s| s.to_string())
    .collect()
}

fn get_me_child_filters(resource_identity: ResourceIdentity) -> Vec<String> {
    match resource_identity {
        ResourceIdentity::Calendars => vec![
            "events/{{id}}".into(),
            "calendarGroup".into(),
            "instances".into(),
            "calendarView".into(),
            "multiValueExtendedProperties".into(),
            "singleValueExtendedProperties".into(),
        ],
        ResourceIdentity::DefaultCalendar => vec![
            "calendars".into(),
            "events/{{id}}".into(),
            "calendarGroup".into(),
            "instances".into(),
            "calendarView".into(),
            "multiValueExtendedProperties".into(),
            "singleValueExtendedProperties".into(),
        ],
        ResourceIdentity::CalendarView => vec![
            "/calendars/{calendar-id}".into(),
            "calendarGroup".into(),
            "instances".into(),
            "/calendar/".into(),
            "events".into(),
            "multiValueExtendedProperties".into(),
            "singleValueExtendedProperties".into(),
        ],
        ResourceIdentity::CalendarGroups => vec![
            "instances".into(),
            "/calendars/{calendar-id}".into(),
            "/calendar/".into(),
            "events".into(),
            "multiValueExtendedProperties".into(),
            "singleValueExtendedProperties".into(),
            "calendarView".into(),
        ],
        // Channels doesUserHaveAccess is part of a broken path
        ResourceIdentity::Channels => vec![
            "messages".into(),
            "sharedWithTeams".into(),
            "doesUserHaveAccess".into(),
            "members".into(),
        ],
        ResourceIdentity::JoinedTeams => {
            vec![
                "primaryChannel".into(),
                "channels".into(),
                "tags".into(),
                "schedule".into(),
            ]
        }
        ResourceIdentity::Events => vec!["instances".into()],
        ResourceIdentity::MailFolders => vec!["childFolders".into(), "messages".into()],
        ResourceIdentity::Outlook => vec!["supportedTimeZones()".into()],
        ResourceIdentity::Todo => vec!["lists".into()],
        ResourceIdentity::TodoLists => vec!["tasks".into()],
        ResourceIdentity::Users => vec!["exportDeviceAndAppManagementData()".into()],
        ResourceIdentity::Planner => vec!["plans".into(), "tasks".into()],

        _ => vec![
            //"/multiValueExtendedProperties".into(),
           // "singleValueExtendedProperties".into(),
        ],
    }
}

pub fn map_child_write_config(ris: ResourceIdentity) -> WriteConfiguration {
    WriteConfiguration::second_level_builder(ResourceIdentity::Me, ris)
        .modifier_name("messages")
        .trim_path_start("/users/{user-id}")
        .filter_path(get_me_child_filters(ris))
        .build()
        .unwrap()
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
                    .filter_path(vec!["attachments".into()])
                    .build()
                    .unwrap()
            }

            ResourceIdentity::EventsInstances => {
                WriteConfiguration::second_level_builder(ResourceIdentity::Users, *ri)
                    .filter_path(vec!["attachments".into()])
                    .trim_path_start("/users/{user-id}/events/{event-id}")
                    .filter_path(get_me_child_filters(*ri))
                    .build()
                    .unwrap()
            }
            ResourceIdentity::DefaultCalendar => {
                WriteConfiguration::second_level_builder(ResourceIdentity::Users, *ri)
                    .trim_path_start("/users/{user-id}")
                    .path("/calendar/")
                    .filter_path(get_me_child_filters(*ri))
                    .build()
                    .unwrap()
            }
            ResourceIdentity::Onenote => {
                WriteConfiguration::second_level_builder(ResourceIdentity::Users, *ri)
                    .trim_path_start("/users/{user-id}")
                    .filter_path(vec![
                        "sectionGroups".into(),
                        "sections".into(),
                        "notebooks".into(),
                        "pages".into(),
                    ])
                    .build()
                    .unwrap()
            }
            ResourceIdentity::OnenoteSections => {
                WriteConfiguration::second_level_builder(ResourceIdentity::Users, *ri)
                    .trim_path_start("/users/{user-id}/onenote")
                    .filter_path(vec!["pages".into()])
                    .build()
                    .unwrap()
            }
            ResourceIdentity::OnenoteSectionGroups => {
                WriteConfiguration::second_level_builder(ResourceIdentity::Users, *ri)
                    .trim_path_start("/users/{user-id}/onenote")
                    .filter_path(vec!["sections".into(), "pages".into()])
                    .build()
                    .unwrap()
            }
            ResourceIdentity::OnenoteNotebooks => {
                WriteConfiguration::second_level_builder(ResourceIdentity::Users, *ri)
                    .trim_path_start("/users/{user-id}/onenote")
                    .filter_path(vec![
                        "sectionGroups".into(),
                        "sections".into(),
                        "pages".into(),
                    ])
                    .build()
                    .unwrap()
            }
            ResourceIdentity::OnenotePages => {
                WriteConfiguration::second_level_builder(ResourceIdentity::Users, *ri)
                    .trim_path_start("/users/{user-id}/onenote")
                    .build()
                    .unwrap()
            }
            ResourceIdentity::ContactFolders => {
                WriteConfiguration::second_level_builder(ResourceIdentity::Users, *ri)
                    .filter_path(vec![
                        "contacts".into(),
                        "childFolders".into(),
                        "multiValueExtendedProperties".into(),
                        "singleValueExtendedProperties".into(),
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
		// Teamwork
		ResourceIdentity::Teamwork => WriteConfiguration::builder(resource_identity)
			.filter_path(vec!["channels".into(), "deletedTeams".into()])
			.children(map_write_config(vec![ResourceIdentity::DeletedTeams]))
			.build()
			.unwrap(),
		ResourceIdentity::DeletedTeams => WriteConfiguration::second_level_builder(ResourceIdentity::Teamwork, resource_identity)
			.trim_path_start("/teamwork")
			.filter_path(vec!["channels".into()])
			.build()
			.unwrap(),


		// Onenote
		ResourceIdentity::Onenote => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}")
			.filter_path(vec!["sectionGroups".into(), "sections".into(), "notebooks".into(), "pages".into()])
			.build()
			.unwrap(),
		ResourceIdentity::OnenoteSections => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}/onenote")
			.filter_path(vec!["pages".into()])
			.build()
			.unwrap(),
		ResourceIdentity::OnenoteSectionGroups => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}/onenote")
			.filter_path(vec!["sections".into(), "pages".into()])
			.build()
			.unwrap(),
		ResourceIdentity::OnenoteNotebooks => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}/onenote")
			.filter_path(vec!["sectionGroups".into(), "sections".into(), "pages".into()])
			.build()
			.unwrap(),
		ResourceIdentity::OnenotePages => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}/onenote")
			.build()
			.unwrap(),


		ResourceIdentity::PlannerTasks => WriteConfiguration::second_level_builder(ResourceIdentity::Planner, resource_identity)
			.trim_path_start("/planner")
			.build()
			.unwrap(),
		ResourceIdentity::Plans => WriteConfiguration::second_level_builder(ResourceIdentity::Planner, resource_identity)
			.filter_path(vec!["tasks".into(), "buckets".into()])
			.trim_path_start("/planner")
			.build()
			.unwrap(),
		ResourceIdentity::Buckets => WriteConfiguration::second_level_builder(ResourceIdentity::Planner, resource_identity)
			.filter_path(vec!["tasks".into()])
			.trim_path_start("/planner")
			.build()
			.unwrap(),
		ResourceIdentity::Planner => WriteConfiguration::builder(resource_identity)
			.filter_path(vec!["plans".into(), "tasks".into(), "buckets".into()])
			.children(map_write_config(vec![ResourceIdentity::PlannerTasks, ResourceIdentity::Plans, ResourceIdentity::Buckets]))
			.build()
			.unwrap(),


		ResourceIdentity::Agreements => WriteConfiguration::builder(resource_identity)
			.filter_path(vec!["file".into(), "files".into()])
			.build()
			.unwrap(),
		ResourceIdentity::Applications => WriteConfiguration::builder(resource_identity)
			.filter_path(vec!["owners".into()])
			.build()
			.unwrap(),


		ResourceIdentity::AccessPackages |
		ResourceIdentity::AccessPackageAssignmentApprovals |
		ResourceIdentity::AssignmentPolicies |
		ResourceIdentity::AssignmentRequests => WriteConfiguration::second_level_builder(ResourceIdentity::IdentityGovernance, resource_identity)
			.trim_path_start("/identityGovernance/entitlementManagement")
			.build().unwrap(),
		ResourceIdentity::AppConsent |
		ResourceIdentity::AccessReviews => WriteConfiguration::second_level_builder(ResourceIdentity::IdentityGovernance, resource_identity)
			.filter_path(vec!["definitions".into(), "/multiValueExtendedProperties".into(), "singleValueExtendedProperties".into()])
			.trim_path_start("/identityGovernance".to_string())
			.build().unwrap(),
		ResourceIdentity::AccessReviewsDefinitions => WriteConfiguration::second_level_builder(ResourceIdentity::IdentityGovernance, resource_identity)
			.trim_path_start("/identityGovernance/accessReviews".to_string())
			.filter_path(vec!["instance".into()])
			.build().unwrap(),
		ResourceIdentity::AccessReviewsDefinitionsInstances => WriteConfiguration::second_level_builder(ResourceIdentity::IdentityGovernance, resource_identity)
			.trim_path_start("/identityGovernance/accessReviews/definitions/{accessReviewScheduleDefinition-id}")
			.build().unwrap(),
		ResourceIdentity::AccessReviewsDefinitionsInstancesStages => WriteConfiguration::second_level_builder(ResourceIdentity::IdentityGovernance, resource_identity)
			.trim_path_start("/identityGovernance/accessReviews/definitions/{accessReviewScheduleDefinition-id}/instances/{accessReviewInstance-id}")
			.build().unwrap(),



		ResourceIdentity::Communications => WriteConfiguration::builder(resource_identity)
			.children(map_write_config(vec![ResourceIdentity::Calls, ResourceIdentity::CallRecords, ResourceIdentity::CallRecordsSessions]))
			.build()
			.unwrap(),
		ResourceIdentity::Calls | ResourceIdentity::CallRecords =>  WriteConfiguration::second_level_builder(ResourceIdentity::Communications, resource_identity)
			.trim_path_start("/communications")
			.build()
			.unwrap(),
		ResourceIdentity::CallRecordsSessions =>  WriteConfiguration::second_level_builder(ResourceIdentity::Communications, resource_identity)
			.trim_path_start("/communications/callRecords/{callRecord-id}")
			.build()
			.unwrap(),


		ResourceIdentity::CertificateBasedAuthConfiguration => WriteConfiguration::from(resource_identity),


		ResourceIdentity::Chats => WriteConfiguration::builder(resource_identity)
			.filter_path(vec!["messages".into(), "members".into(), "tabs".into()])
			.children(map_write_config(vec![ResourceIdentity::ChatsMessages, ResourceIdentity::ChatsMessagesReplies]))
			.build()
			.unwrap(),
		ResourceIdentity::ChatsMessages => WriteConfiguration::second_level_builder(ResourceIdentity::Chats, resource_identity)
			.trim_path_start("/chats/{chat-id}")
			.filter_path(vec!["replies".into()])
			.build()
			.unwrap(),
		ResourceIdentity::ChatsMessagesReplies => WriteConfiguration::second_level_builder(ResourceIdentity::Chats, resource_identity)
			.trim_path_start("/chats/{chat-id}/messages/{chatMessage-id}")
			.build()
			.unwrap(),


		ResourceIdentity::ConnectedOrganizations => WriteConfiguration::second_level_builder(ResourceIdentity::IdentityGovernance, resource_identity)
			.trim_path_start("/identityGovernance/entitlementManagement")
			.filter_path(vec!["internalSponsors".into(), "externalSponsors".into()])
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
			.filter_path(vec!["userStateSummary".into(), "deviceStates".into()])
			.build()
			.unwrap(),
		ResourceIdentity::ManagedEBooksDeviceStates |
		ResourceIdentity::ManagedEBooksUserStateSummary  => WriteConfiguration::second_level_builder(ResourceIdentity::DeviceAppManagement, resource_identity)
			.trim_path_start("/deviceAppManagement/managedEBooks/{managedEBook-id}")
			.build()
			.unwrap(),
		ResourceIdentity::ManagedAppRegistrations => WriteConfiguration::second_level_builder(ResourceIdentity::DeviceAppManagement, resource_identity)
			.trim_path_start("/deviceAppManagement")
			.filter_path(vec!["appliedPolicies".into(), "intendedPolicies".into()])
			.build()
			.unwrap(),
		ResourceIdentity::ManagedAppRegistrationsIntendedPolicies |
		ResourceIdentity::ManagedAppRegistrationsAppliedPolicies  => WriteConfiguration::second_level_builder(ResourceIdentity::DeviceAppManagement, resource_identity)
			.trim_path_start("/deviceAppManagement/managedAppRegistrations/{managedAppRegistration-id}")
			.build()
			.unwrap(),
		ResourceIdentity::DeviceAppManagement => WriteConfiguration::builder(resource_identity)
			.filter_path(vec![
				"androidManagedAppProtections".into(),"defaultManagedAppProtections".into(),"iosManagedAppProtections".into(),"managedAppPolicies".into(),
				"managedAppRegistrations".into(),"managedAppStatuses".into(),"managedEBooks".into(),"mdmWindowsInformationProtectionPolicies".into(),"mobileAppCategories".into(),
				"mobileAppConfigurations".into(),"mobileApps".into(),"targetedManagedAppConfigurations".into(),"vppTokens".into(),"windowsInformationProtectionPolicies".into()
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
					"deviceEnrollmentConfigurations".into(), "deviceConfigurations".into(), "deviceCompliancePolicies".into(),
					"termsAndConditions".into(), "managedDevices".into(), "roleDefinitions".into(), "troubleshootingEvents".into(),
					"reports".into(), "deviceCompliancePolicySettingStateSummaries".into(), "windowsAutopilotDeviceIdentities".into()
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
				.build()
				.unwrap(),


		// Directory
		ResourceIdentity::DeletedItems | ResourceIdentity::AdministrativeUnits => WriteConfiguration::second_level_builder(ResourceIdentity::Directory, resource_identity)
			.trim_path_start("/directory")
			.filter_path(vec!["members".into()])
			.build()
			.unwrap(),
		ResourceIdentity::DirectoryMembers => WriteConfiguration::second_level_builder(ResourceIdentity::Directory, resource_identity)
			.trim_path_start("/directory/administrativeUnits/{administrativeUnit-id}")
			.build()
			.unwrap(),
		ResourceIdentity::Directory =>
			WriteConfiguration::builder(resource_identity)
				.filter_path(vec![
					"microsoft.graph.additionalAccess()".to_string(), "directoryObjects".into(), "directoryRoleTemplates".into(),
					"directoryRoles".into(), "administrativeUnits".into(), "deletedItems".into()
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
			.filter_path(vec!["members".to_string()])
			.build()
			.unwrap(),


		ResourceIdentity::DomainDnsRecords => WriteConfiguration::from(resource_identity),



		ResourceIdentity::EntitlementManagement => WriteConfiguration::second_level_builder(ResourceIdentity::Directory, resource_identity)
			.trim_path_start("/entitlementManagement")
			.filter_path(vec![
				"assignments".into(), "accessPackages".into(), "assignmentPolicies".into(), "assignmentRequests".into(),
				"catalogs".into(), "$count".into(), "accessPackageAssignmentApprovals".into(), "connectedOrganizations".into()
			])
			.build()
			.unwrap(),
		ResourceIdentity::EntitlementManagementAssignments |
		ResourceIdentity::EntitlementManagementCatalogs => WriteConfiguration::second_level_builder(ResourceIdentity::Directory, resource_identity)
			.trim_path_start("/identityGovernance/entitlementManagement")
			.filter_path(vec!["additionalAccess()".to_string(), "accessPackages/".to_string()])
			.build()
			.unwrap(),
		ResourceIdentity::IdentityGovernance =>
			WriteConfiguration::builder(resource_identity)
				.filter_path(vec!["entitlementManagement".into(), "accessReviews".into(), "appConsent".into()])
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
				ResourceIdentity::Lists, ResourceIdentity::SitesItems, ResourceIdentity::SitesContentTypes,
				ResourceIdentity::TermStores, ResourceIdentity::TermStore, ResourceIdentity::TermStoreGroups,
				ResourceIdentity::TermStoreSets, ResourceIdentity::TermStoreSetsChildren, ResourceIdentity::TermStoreSetsTerms,
				ResourceIdentity::TermStoreSetsParentGroup
			]))
			.filter_path(vec!["termStores".into(), "termStore".into(), "lists".into(),
							  "getActivitiesByInterval()".into(), "onenote".into(), "contentTypes".into()])
			.build()
			.unwrap(),
		ResourceIdentity::Lists => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.filter_path(vec!["items".into(), "contentTypes".into()])
			.trim_path_start("/sites/{site-id}")
			.build()
			.unwrap(),
		ResourceIdentity::SitesContentTypes=> WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.trim_path_start("/sites/{site-id}")
			.build()
			.unwrap(),
		ResourceIdentity::SitesItems => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.filter_path(vec!["contentTypes".into()])
			.trim_path_start("/sites/{site-id}")
			.build()
			.unwrap(),
		ResourceIdentity::TermStore => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.filter_path(vec!["termStores".into(), "sets".into(), "groups".into()])
			.trim_path_start("/sites/{site-id}")
			.build()
			.unwrap(),
		ResourceIdentity::TermStoreGroups => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.filter_path(vec!["sets".into()])
			.trim_path_start("/sites/{site-id}/termStore")
			.build()
			.unwrap(),
		ResourceIdentity::TermStoreSets => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.filter_path(vec!["children".into(), "parentGroup".into(), "terms".into(), "relations".into()])
			.trim_path_start("/sites/{site-id}/termStore")
			.build()
			.unwrap(),
		ResourceIdentity::TermStoreSetsChildren => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.filter_path(vec!["children/{term-id1}".into(), "relations/{relation-id}".into()])
			.trim_path_start("/sites/{site-id}/termStore/sets/{set-id}")
			.build()
			.unwrap(),
		ResourceIdentity::TermStoreSetsRelations => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.trim_path_start("/sites/{site-id}/termStore/sets/{set-id}")
			.build()
			.unwrap(),
		ResourceIdentity::TermStoreSetsTerms => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.filter_path(vec!["children".into(), "relations".into()])
			.trim_path_start("/sites/{site-id}/termStore/sets/{set-id}")
			.build()
			.unwrap(),
		ResourceIdentity::TermStoreSetsParentGroup => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.filter_path(vec!["sets/{set-id1}/children".into(), "sets/{set-id1}/terms".into(), "sets/{set-id1}/relations".into()])
			.trim_path_start("/sites/{site-id}/termStore/sets/{set-id}")
			.build()
			.unwrap(),
		ResourceIdentity::TermStores => WriteConfiguration::second_level_builder(ResourceIdentity::Sites, resource_identity)
			.filter_path(vec!["sets".into(), "groups".into()])
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
			.filter_path(vec!["sharedWithTeams".into(), "tabs".into(), "messages".into(), "members".into()])
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
			.filter_path(vec!["members".into()])
			.build()
			.unwrap(),


		ResourceIdentity::Calendars => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}")
			.path("/calendars/")
			.filter_path(vec![
				"calendarGroup".into(),
				"instances".into(),
				"calendarView".into(),
				"events".into(),
				"multiValueExtendedProperties".into(),
				"singleValueExtendedProperties".into(),
				"/attachments/".into(),
			])
			.replace_operation_map(resource_identity.exact_camel_case())
			.build().unwrap(),
		ResourceIdentity::DefaultCalendar => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}")
			.path("/calendar/")
			.filter_path(vec![
				"calendarGroup".into(), "instances".into(), "calendarView".into(), "events".into(), "multiValueExtendedProperties".into(),
				"singleValueExtendedProperties".into(), "/calendars/{calendar-id}".into(), "/attachments/".into(),
			])
			.build().unwrap(),
		ResourceIdentity::CalendarView => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}")
			.filter_path(vec![
				"calendarGroup".into(), "instances".into(), "/calendars/".into(), "/calendar/".into(), "events".into(),
				"multiValueExtendedProperties".into(), "singleValueExtendedProperties".into(), "/attachments/".into(),
			])
			.build().unwrap(),
		ResourceIdentity::CalendarGroups => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}")
			.path("/calendarGroups")
			.filter_path(vec![
				"calendarView".into(), "instances".into(), "/calendars/".into(), "/calendar/".into(), "events".into(),
				"multiValueExtendedProperties".into(), "singleValueExtendedProperties".into(), "/attachments/".into(),
			])
			.build().unwrap(),


		ResourceIdentity::Drives => WriteConfiguration::builder(resource_identity)
			.filter_path(vec!["items".into(), "list".into()])
			.children(map_write_config(vec![ResourceIdentity::DrivesList, ResourceIdentity::DrivesItems, ResourceIdentity::DrivesListContentTypes]))
			.build()
			.unwrap(),
		ResourceIdentity::DrivesList => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.trim_path_start("/drives/{drive-id}")
			.filter_path(vec!["items".into(), "contentTypes".into()])
			.build()
			.unwrap(),
		ResourceIdentity::DrivesListContentTypes => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.trim_path_start("/drives/{drive-id}/list")
			.build()
			.unwrap(),
		ResourceIdentity::DrivesItems => WriteConfiguration::second_level_builder(ResourceIdentity::Drives, resource_identity)
			.trim_path_start("/drives/{drive-id}")
			.filter_path(vec!["workbook".into(), "getActivitiesByInterval()".into()])
			.build()
			.unwrap(),


		// Service Principals
		ResourceIdentity::ServicePrincipals => WriteConfiguration::builder(resource_identity)
			.filter_path(vec!["owners".into(), "transitiveMemberOf".into(), "oauth2PermissionGrants".into(), "memberOf".into(), "createdObjects".into(), "ownedObjects".into()])
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
				ResourceIdentity::GroupsOwners, ResourceIdentity::GroupsTeam
			]))
			.build()
			.unwrap(),
		// Groups Team
		ResourceIdentity::GroupsTeam => WriteConfiguration::second_level_builder(
			ResourceIdentity::Groups,
			ResourceIdentity::GroupsTeam)
			.trim_path_start("/groups/{group-id}")
			.filter_path(vec!["primaryChannel".into(), "channels".into(), "tags".into(), "schedule".into(), "members".into()])
			.build().unwrap(),
		ResourceIdentity::Threads =>  WriteConfiguration::second_level_builder(
			ResourceIdentity::Groups,
			ResourceIdentity::Threads)
			.trim_path_start("/groups/{group-id}")
			.filter_path(vec![
				"posts".into(),
				"multiValueExtendedProperties".into(),
				"singleValueExtendedProperties".into(),
			])
			.build().unwrap(),
		ResourceIdentity::GroupsOwners =>  WriteConfiguration::second_level_builder(ResourceIdentity::Groups, resource_identity)
			.trim_path_start("/groups/{group-id}")
			.modifier_name("groupsOwners")
			.build().unwrap(),
		ResourceIdentity::Conversations => WriteConfiguration::second_level_builder(ResourceIdentity::Groups, resource_identity)
			.trim_path_start("/groups/{group-id}")
			.filter_path(vec![
				"conversations/{{RID}}/threads".into(),
				"posts".into(),
				"threads".into(),
				"multiValueExtendedProperties".into(),
				"singleValueExtendedProperties".into(),
			])
			.build().unwrap(),
		ResourceIdentity::ThreadsPosts => WriteConfiguration::second_level_builder(ResourceIdentity::Groups, resource_identity
		).filter_path(vec!["inReplyTo".into(), "/multiValueExtendedProperties".into(), "singleValueExtendedProperties".into()])
			.trim_path_start("/groups/{group-id}/threads/{conversationThread-id}")
			.build().unwrap(),


		ResourceIdentity::Identity => WriteConfiguration::builder(resource_identity)
			.replace_operation_map(resource_identity.exact_camel_case())
			.filter_path(vec!["identityProviders".into(), "identityGovernance".into(), "identityProtection".into()])
			.build()
			.unwrap(),


		// Education
		ResourceIdentity::EducationClasses |
		ResourceIdentity::EducationSchools |
		ResourceIdentity::EducationMe |
		ResourceIdentity::EducationUsers => WriteConfiguration::second_level_builder(ResourceIdentity::Education, resource_identity)
			.trim_path_start("/education")
			.filter_path(vec!["assignments".into()])
			.build()
			.unwrap(),
		ResourceIdentity::EducationAssignments => WriteConfiguration::second_level_builder(ResourceIdentity::Education, resource_identity)
			.trim_path_start("/education/me")
			.filter_path(vec!["submissions".into()])
			.build()
			.unwrap(),
		ResourceIdentity::EducationAssignmentsSubmissions => WriteConfiguration::second_level_builder(ResourceIdentity::Education, resource_identity)
			.trim_path_start("/education/me/assignments/{educationAssignment-id}")
			.build()
			.unwrap(),
		ResourceIdentity::Education => WriteConfiguration::builder(resource_identity)
			.filter_path(vec!["me".into(), "classes".into(), "schools".into(), "users".into()])
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

		// ApiClientLink::Struct("users_attachments".into(), "UsersAttachmentsApiClient".into()),
		// ApiClientLink::StructId("users_attachment".into(), "UsersAttachmentsIdApiClient".into()),
		// ResourceIdentity::UsersMessages
		// /users/{user-id}/events/{event-id}
		ResourceIdentity::EventsInstances=> WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.filter_path(vec!["attachments".into()])
			.trim_path_start("/users/{user-id}/events/{event-id}")
			.build()
			.unwrap(),
		ResourceIdentity::Events => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.filter_path(vec!["attachments".into(), "instances".into()])
			.trim_path_start("/users/{user-id}")
			.build()
			.unwrap(),
		ResourceIdentity::UsersMessages => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.filter_path(vec!["attachments".into()])
			.trim_path_start("/users/{user-id}")
			.build()
			.unwrap(),
		ResourceIdentity::UsersAttachments => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.trim_path_start("/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}")
			.build()
			.unwrap(),
		ResourceIdentity::ChildFolders => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.filter_path(vec!["contacts".into(), "messages".into(), "singleValueExtendedProperties".into(), "multiValueExtendedProperties".into()])
			.trim_path_start("/users/{user-id}/mailFolders/{mailFolder-id}")
			.build()
			.unwrap(),
		ResourceIdentity::ContactFolders => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.filter_path(vec!["childFolders".into(), "contacts".into(), "singleValueExtendedProperties".into(), "multiValueExtendedProperties".into()])
			.trim_path_start("/users/{user-id}")
			.build()
			.unwrap(),
		ResourceIdentity::MailFolders => WriteConfiguration::second_level_builder(ResourceIdentity::Users, resource_identity)
			.filter_path(vec!["childFolders".into(), "messages".into(), "singleValueExtendedProperties".into(), "multiValueExtendedProperties".into()])
			.trim_path_start("/users/{user-id}")
			.build()
			.unwrap(),
		_ => WriteConfiguration::builder(resource_identity)
			.build()
			.unwrap(),

	}
}
