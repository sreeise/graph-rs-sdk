use crate::parser::ParserSettings;
use crate::settings::{ApiClientLink, ApiClientLinkSetting};
use from_as::*;
use graph_core::resource::ResourceIdentity;
use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Write};

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
    pub api_client_links: Vec<ApiClientLinkSetting>,
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
			ResourceIdentity::AccessPackages =>
				ResourceSettings {
					path_name: path_name.into(),
					ri,
					imports: vec!["crate::identity_governance::{AssignmentPoliciesApiClient, AssignmentPoliciesIdApiClient}".into()],
					api_client_links: vec![
						ApiClientLinkSetting(
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
						ApiClientLinkSetting(
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
						ApiClientLinkSetting(
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
						ApiClientLinkSetting(
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
						ApiClientLinkSetting(
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
					ApiClientLinkSetting(
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
					ApiClientLinkSetting(
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
					ApiClientLinkSetting(
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
						ApiClientLinkSetting(
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
					ApiClientLinkSetting(Some("ConnectedOrganizationsIdApiClient".into()), vec![
						ApiClientLink::Struct("external_sponsors".into(), "ConnectedOrganizationsExternalSponsorsApiClient".into()),
						ApiClientLink::Struct("internal_sponsors".into(), "ConnectedOrganizationsInternalSponsorsApiClient".into()),
					])
				])
				.build()
				.unwrap(),
			ResourceIdentity::DeviceAppManagement => ResourceSettings {
				path_name: path_name.to_string(),
				ri,
				imports: vec!["crate::device_app_management::{DeviceAppManagementAndroidManagedAppProtectionsIdApiClient, DeviceAppManagementAndroidManagedAppProtectionsApiClient}".into()],
				api_client_links: vec![ApiClientLinkSetting(None, vec![
					ApiClientLink::Struct("android_managed_app_protections".into(), "DeviceAppManagementAndroidManagedAppProtectionsApiClient".into()),
					ApiClientLink::StructId("android_managed_app_protection".into(), "DeviceAppManagementAndroidManagedAppProtectionsIdApiClient".into()),
				])],
			},
			ResourceIdentity::DeviceManagement =>
				ResourceSettings {
					path_name: path_name.into(),
					ri,
					imports: vec![
						"crate::device_management::{DeviceConfigurationsApiClient, DeviceConfigurationsIdApiClient, DeviceEnrollmentConfigurationsApiClient, DeviceEnrollmentConfigurationsIdApiClient, DeviceManagementManagedDevicesApiClient, DeviceManagementManagedDevicesIdApiClient, RoleDefinitionsApiClient, RoleDefinitionsIdApiClient, TermsAndConditionsApiClient, TermsAndConditionsIdApiClient}".into()
					],
					api_client_links: vec![
						ApiClientLinkSetting(
							None,
							vec![
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
								ApiClientLink::StructId("terms_and_condition".into(), "TermsAndConditionsIdApiClient".into())
							]
						)
					],
				},
			ResourceIdentity::Drives =>
				ResourceSettings {
					path_name: path_name.to_string(),
					ri,
					imports: vec![
						"crate::drives::{DrivesListApiClient, DrivesItemsApiClient, DrivesItemsIdApiClient}".into()
					],
					api_client_links: vec![
						ApiClientLinkSetting(Some("DrivesIdApiClient".into()), vec![
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
						ApiClientLinkSetting(Some("DrivesListApiClient".into()), vec![
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
						ApiClientLinkSetting(None, vec![
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
						ApiClientLinkSetting(Some("EducationAssignmentsIdApiClient".into()), vec![
							ApiClientLink::Struct("submissions".into(), "EducationAssignmentsSubmissionsApiClient".into()),
							ApiClientLink::StructId("submission".into(), "EducationAssignmentsSubmissionsIdApiClient".into()),
						])
					]).build().unwrap(),
			ResourceIdentity::EducationMe => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::education::*".into()])
				.api_client_links(vec![
						ApiClientLinkSetting(Some("EducationMeApiClient".into()),vec![
							ApiClientLink::Struct("assignments".into(), "EducationAssignmentsApiClient".into()),
							ApiClientLink::StructId("assignment".into(), "EducationAssignmentsIdApiClient".into()),
						])
					]).build().unwrap(),
			ResourceIdentity::EducationSchools => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::education::*".into()])
				.api_client_links(vec![
						ApiClientLinkSetting(Some("EducationSchoolsIdApiClient".into()), vec![
							ApiClientLink::Struct("assignments".into(), "EducationAssignmentsApiClient".into()),
							ApiClientLink::StructId("assignment".into(), "EducationAssignmentsIdApiClient".into()),
						])
					]).build().unwrap(),
			ResourceIdentity::EducationUsers => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::education::*".into()])
				.api_client_links(vec![
						ApiClientLinkSetting(Some("EducationUsersIdApiClient".into()),vec![
							ApiClientLink::Struct("assignments".into(), "EducationAssignmentsApiClient".into()),
							ApiClientLink::StructId("assignment".into(), "EducationAssignmentsIdApiClient".into()),
						])
					]).build().unwrap(),
			ResourceIdentity::EducationClasses => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::education::*".into()])
				.api_client_links(vec![
						ApiClientLinkSetting(Some("EducationClassesIdApiClient".into()), vec![
							ApiClientLink::Struct("assignments".into(), "EducationAssignmentsApiClient".into()),
							ApiClientLink::StructId("assignment".into(), "EducationAssignmentsIdApiClient".into()),
						])
					]).build().unwrap(),
			ResourceIdentity::EntitlementManagement =>
				ResourceSettings {
					path_name: path_name.to_string(),
					ri,
					imports: vec![
						"crate::identity_governance::{AccessPackagesApiClient, AccessPackagesIdApiClient, ConnectedOrganizationsApiClient, ConnectedOrganizationsIdApiClient, AccessPackageAssignmentApprovalsApiClient, AccessPackageAssignmentApprovalsIdApiClient}".to_string(),
						"crate::identity_governance::{AssignmentPoliciesApiClient, AssignmentPoliciesIdApiClient, AssignmentRequestsApiClient, AssignmentRequestsIdApiClient, EntitlementManagementAssignmentsApiClient, EntitlementManagementAssignmentsIdApiClient}".to_string(),
						"crate::identity_governance::{EntitlementManagementCatalogsApiClient, EntitlementManagementCatalogsIdApiClient}".to_string()
					],
					api_client_links: vec![
						ApiClientLinkSetting(
							None,
							vec![
								ApiClientLink::Resource("access_packages".into(), "AccessPackagesApiClient".into(), ResourceIdentity::AccessPackages),
								ApiClientLink::ResourceId(
									"access_package".into(),
									"AccessPackagesIdApiClient".into(),
									ResourceIdentity::AccessPackages
								),
								ApiClientLink::Resource(
									"access_package_assignment_approvals".into(),
									"AccessPackageAssignmentApprovalsApiClient".into(),
									ResourceIdentity::AccessPackageAssignmentApprovals
								),
								ApiClientLink::ResourceId(
									"access_package_assignment_approval".into(),
									"AccessPackageAssignmentApprovalsIdApiClient".into(),
									ResourceIdentity::AccessPackageAssignmentApprovals
								),
								ApiClientLink::Resource(
									"assignment_policies".into(),
									"AssignmentPoliciesApiClient".into(),
									ResourceIdentity::AssignmentPolicies
								),
								ApiClientLink::ResourceId(
									"assignment_policy".into(),
									"AssignmentPoliciesIdApiClient".into(),
									ResourceIdentity::AssignmentPolicies
								),
								ApiClientLink::Resource(
									"assignments".into(),
									"EntitlementManagementAssignmentsApiClient".into(),
									ResourceIdentity::EntitlementManagementAssignments
								),
								ApiClientLink::ResourceId(
									"assignment".into(),
									"EntitlementManagementAssignmentsIdApiClient".into(),
									ResourceIdentity::EntitlementManagementAssignments
								),
								ApiClientLink::Resource(
									"catalogs".into(),
									"EntitlementManagementCatalogsApiClient".into(),
									ResourceIdentity::EntitlementManagementCatalogs
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
						ApiClientLinkSetting(
							Some("EntitlementManagementCatalogsIdApiClient".into()),
							vec![
								ApiClientLink::Resource("access_packages".into(), "AccessPackagesApiClient".into(), ResourceIdentity::AccessPackages),
								ApiClientLink::ResourceId(
									"access_package".into(),
									"AccessPackagesIdApiClient".into(),
									ResourceIdentity::AccessPackages
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
						ApiClientLinkSetting(
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
			/*
			    api_client_link_id!(
        extended_properties,
        ResourceIdentity::ExtendedProperties,
        ExtendedPropertiesApiClient
    );
			 */
			ResourceIdentity::Events => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into()])
				.api_client_links(vec![
					ApiClientLinkSetting(
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
					ApiClientLinkSetting(
						Some("CalendarViewIdApiClient".into()),
						vec![
							ApiClientLink::Struct("instances".into(), "EventsInstancesApiClient".into()),
							ApiClientLink::StructId("instance".into(), "EventsInstancesIdApiClient".into()),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::Calendars => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into()])
				.api_client_links(vec![
					ApiClientLinkSetting(
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
				.imports(vec!["crate::users::*".into()])
				.api_client_links(vec![
					ApiClientLinkSetting(
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
			ResourceIdentity::CalendarGroups => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into()])
				.api_client_links(vec![
					ApiClientLinkSetting(
						Some("CalendarGroupsIdApiClient".into()),
						vec![
							ApiClientLink::Struct("calendars".into(), "CalendarsApiClient".into()),
							ApiClientLink::StructId("calendar".into(), "CalendarsIdApiClient".into()),
						]
					),
				]).build().unwrap(),
			ResourceIdentity::Groups => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::groups::*".into(), "crate::group_lifecycle_policies::*".into()])
				.api_client_links(vec![
						ApiClientLinkSetting(
						Some("GroupsIdApiClient".into()),
						vec![
							ApiClientLink::Struct("group_lifecycle_policies".into(), "GroupLifecyclePoliciesApiClient".into()),
							ApiClientLink::Struct("conversation".into(), "GroupsConversationsIdApiClient".into()),
							ApiClientLink::Struct("conversations".into(), "GroupsConversationsApiClient".into()),
							ApiClientLink::Struct("thread".into(), "GroupsThreadsIdApiClient".into()),
							ApiClientLink::Struct("threads".into(), "GroupsThreadsApiClient".into())
						]
					)
				]).build().unwrap(),
			ResourceIdentity::GroupsThreads => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::groups::*".into()])
				.api_client_links(vec![
					ApiClientLinkSetting(
						Some("GroupsThreadsIdApiClient".into()),
						vec![
							ApiClientLink::Struct("post".into(), "GroupsThreadsPostsIdApiClient".into()),
							ApiClientLink::Struct("posts".into(), "GroupsThreadsPostsApiClient".into())
						]
					)
				]).build().unwrap(),
			ResourceIdentity::Me =>
				ResourceSettings {
					path_name: path_name.to_string(),
					ri,
					imports: vec![],
					api_client_links: vec![],
				},
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
					ApiClientLinkSetting(
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
					ApiClientLinkSetting(
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
					ApiClientLinkSetting(
						Some("TeamsTagsIdApiClient".into()),
						vec![
							ApiClientLink::Struct("members".into(), "TeamsMembersApiClient".into()),
							ApiClientLink::Struct("member".into(), "TeamsMembersIdApiClient".into()),
						]
					)
				]).build().unwrap(),
			ResourceIdentity::Team => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into(), "crate::teams::*".into()])
				.api_client_links(vec![
					ApiClientLinkSetting(
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
					ApiClientLinkSetting(
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
					ApiClientLinkSetting(
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
					ApiClientLinkSetting(
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
			ResourceIdentity::Users => ResourceSettings::builder(path_name, ri)
				.imports(vec!["crate::users::*".into(), "crate::chats::*".into()])
				.api_client_links(vec![
					ApiClientLinkSetting(
						Some("UsersIdApiClient".into()),
						vec![
							ApiClientLink::Struct("chats".into(), "ChatsApiClient".into()),
							ApiClientLink::StructId("chat".into(), "ChatsIdApiClient".into()),

							// Users and Me
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
							ApiClientLink::Struct("app_role_assignments".into(), "AppRoleAssignmentsApiClient".into()),
							ApiClientLink::StructId("app_role_assignment".into(), "AppRoleAssignmentsIdApiClient".into()),
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
							ApiClientLink::Struct("oauth_2_permission_grants".into(), "Oauth2PermissionGrantsApiClient".into()),
							ApiClientLink::StructId("oauth_2_permission_grant".into(), "Oauth2PermissionGrantsIdApiClient".into()),
							ApiClientLink::Struct("followed_sites".into(), "FollowedSitesApiClient".into()),
							ApiClientLink::StructId("followed_site".into(), "FollowedSitesIdApiClient".into()),
							ApiClientLink::Struct("managed_app_registrations".into(), "ManagedAppRegistrationsApiClient".into()),
							ApiClientLink::StructId("managed_app_registration".into(), "ManagedAppRegistrationsIdApiClient".into()),
							ApiClientLink::Struct("scoped_role_member_of".into(), "ScopedRoleMemberOfApiClient".into()),
							ApiClientLink::StructId("scoped_role_member_of_id".into(), "ScopedRoleMemberOfIdApiClient".into()),
							ApiClientLink::Struct("teamwork".into(), "TeamworkApiClient".into()),
							ApiClientLink::Struct("inference_classification".into(), "InferenceClassificationApiClient".into()),
							ApiClientLink::Struct("mail_folders".into(), "MailFoldersApiClient".into()),
							ApiClientLink::StructId("mail_folder".into(), "MailFoldersIdApiClient".into()),
							ApiClientLink::Struct("activities".into(), "ActivitiesApiClient".into()),
							ApiClientLink::StructId("activity".into(), "ActivitiesIdApiClient".into()),
							ApiClientLink::Struct("device_management_troubleshooting_events".into(), "DeviceManagementTroubleshootingEventsApiClient".into()),
							ApiClientLink::StructId("device_management_troubleshooting_event".into(), "DeviceManagementTroubleshootingEventsIdApiClient".into()),
							ApiClientLink::Struct("extensions".into(), "ExtensionsApiClient".into()),
							ApiClientLink::StructId("extension".into(), "ExtensionsIdApiClient".into()),
							ApiClientLink::Struct("todo".into(), "TodoApiClient".into()),
							ApiClientLink::Struct("created_objects".into(), "CreatedObjectsApiClient".into()),
							ApiClientLink::StructId("created_object".into(), "CreatedObjectsIdApiClient".into()),
							ApiClientLink::Struct("transitive_member_of".into(), "TransitiveMemberOfApiClient".into()),
							ApiClientLink::StructId("transitive_member_of_id".into(), "TransitiveMemberOfIdApiClient".into()),
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
							ApiClientLink::Struct("registered_devices".into(), "RegisteredDevicesApiClient".into()),
							ApiClientLink::StructId("registered_device".into(), "RegisteredDevicesIdApiClient".into()),
							ApiClientLink::Struct("owned_devices".into(), "OwnedDevicesApiClient".into()),
							ApiClientLink::StructId("owned_device".into(), "OwnedDevicesIdApiClient".into()),
							ApiClientLink::Struct("owned_objects".into(), "OwnedObjectsApiClient".into()),
							ApiClientLink::StructId("owned_object".into(), "OwnedObjectsIdApiClient".into()),
						]
					)
				])
				.build()
				.unwrap(),
			_ => ResourceSettings::default(path_name, ri),
		}
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile, Eq, PartialEq, Hash)]
pub struct ResourceSettingsMap(pub BTreeMap<ResourceIdentity, ResourceSettings>);
