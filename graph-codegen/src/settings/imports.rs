use graph_core::resource::ResourceIdentity;

pub fn get_imports(resource_identity: ResourceIdentity) -> Vec<&'static str> {
    match resource_identity {
        ResourceIdentity::AccessPackages => vec!["crate::assignment_policies::{AssignmentPoliciesIdRequest, AssignmentPoliciesRequest}"],
        ResourceIdentity::AccessReviews => vec![
            "crate::access_review_definitions::{AccessReviewDefinitionsIdRequest, AccessReviewDefinitionsRequest}"
        ],
        ResourceIdentity::AccessReviewDefinitions => vec![
            "crate::definition_instances::{DefinitionInstancesIdRequest, DefinitionInstancesRequest}"
        ],
        ResourceIdentity::Admin => vec!["crate::service_announcement::ServiceAnnouncementRequest"],
        ResourceIdentity::AdministrativeUnits => vec!["crate::administrative_units_members::{AdministrativeUnitsMembersRequest, AdministrativeUnitsMembersIdRequest}"],
        ResourceIdentity::AuthenticationMethodsPolicy => vec![
            "crate::authentication_method_configurations::{AuthenticationMethodConfigurationsRequest, AuthenticationMethodConfigurationsIdRequest}"
        ],
        ResourceIdentity::Buckets => vec!["crate::tasks::{TasksRequest, TasksIdRequest}"],
        ResourceIdentity::Calendar | ResourceIdentity::Calendars => vec![
            "crate::calendar_view::{CalendarViewRequest, CalendarViewsRequest}",
            "crate::events::{EventsRequest, EventRequest}",
            "crate::extended_properties::ExtendedPropertiesRequest",
        ],
        ResourceIdentity::CalendarGroup | ResourceIdentity::CalendarGroups => vec![
            "crate::calendar::{CalendarRequest, CalendarsRequest}",
            "crate::events::{EventsRequest, EventRequest}",
        ],
        ResourceIdentity::CalendarView => vec![
            "crate::instances::{InstanceRequest, InstancesRequest}",
            "crate::calendar::CalendarRequest",
            "crate::extended_properties::ExtendedPropertiesRequest",
            "crate::attachments::{AttachmentRequest, AttachmentsRequest}",
        ],
        ResourceIdentity::Calls => vec!["crate::core::ResourceIdentity"],
        ResourceIdentity::CallRecords => vec!["crate::sessions::{SessionRequest, SessionsRequest}"],
        ResourceIdentity::Channels => vec![
            "crate::chats_channels_messages::{ChatsAndChannelsMessagesIdRequest, ChatsAndChannelsMessagesRequest}",
            "crate::tabs::{TabsIdRequest, TabsRequest}",
            "crate::members::{MembersIdRequest, MembersRequest}",
            "crate::shared_with_teams::{SharedWithTeamsIdRequest, SharedWithTeamsRequest}",
        ],
        ResourceIdentity::Communications => vec![
            "crate::call_records::{CallRecordRequest, CallRecordsRequest}",
            "crate::calls::{CallRequest, CallsRequest}",
        ],
        ResourceIdentity::Contacts => vec!["crate::extended_properties::ExtendedPropertiesRequest"],
        ResourceIdentity::ContactFolders => vec![
            "crate::child_folders::{ChildFolderRequest, ChildFoldersRequest}",
            "crate::contacts::{ContactRequest, ContactsRequest}",
            "crate::extended_properties::ExtendedPropertiesRequest",
        ],
        ResourceIdentity::Conversations => vec!["crate::threads::{ThreadRequest, ThreadsRequest}"],
        ResourceIdentity::ChildFolders => {
            vec!["crate::messages::{MessageRequest, MessagesRequest}"]
        }
        ResourceIdentity::DefinitionInstances => vec![
            "crate::definition_instance_stages::{DefinitionInstanceStagesRequest, DefinitionInstanceStagesIdRequest}"
        ],
        ResourceIdentity::Directory => vec![
            "crate::administrative_units::{AdministrativeUnitsRequest, AdministrativeUnitsIdRequest}",
            "crate::directory_deleted_items::{DirectoryDeletedItemsIdRequest, DirectoryDeletedItemsRequest}"
        ],
        ResourceIdentity::DirectoryRoles => vec!["crate::administrative_units_members::{AdministrativeUnitsMembersRequest, AdministrativeUnitsMembersIdRequest}"],
        ResourceIdentity::Drive | ResourceIdentity::Drives => vec![
            "std::path::Path",
            "crate::items::{ItemRequest, ItemsRequest}",
            "crate::lists::{ListRequest, ListsRequest}",
            "graph_http::types::DeltaPhantom",
        ],
        ResourceIdentity::Domains => vec!["crate::core::ResourceIdentity"],
        ResourceIdentity::Lists => vec![
            "crate::content_types::{ContentTypeRequest, ContentTypesRequest}",
            "crate::items::{ItemRequest, ItemsRequest}",
        ],
        ResourceIdentity::EntitlementManagement => vec![
            "crate::access_package_assignment_approvals::{AccessPackageAssignmentApprovalsIdRequest, AccessPackageAssignmentApprovalsRequest}",
            "crate::access_packages::{AccessPackagesIdRequest, AccessPackagesRequest}",
            "crate::assignment_policies::{AssignmentPoliciesIdRequest, AssignmentPoliciesRequest}",
            "crate::assignment_requests::{AssignmentRequestsIdRequest, AssignmentRequestsRequest}",
            "crate::assignments::{AssignmentsIdRequest, AssignmentsRequest}",
            "crate::connected_organizations::{ConnectedOrganizationsIdRequest, ConnectedOrganizationsRequest}"
        ],
        ResourceIdentity::Events => vec![
            "crate::calendar::CalendarRequest",
            "crate::instances::{InstanceRequest, InstancesRequest}",
            "crate::extended_properties::ExtendedPropertiesRequest",
            "crate::attachments::{AttachmentRequest, AttachmentsRequest}",
        ],
        ResourceIdentity::IdentityGovernance => vec![
            "crate::entitlement_management::EntitlementManagementRequest",
            "crate::terms_of_use::TermsOfUseRequest",
            "crate::app_consent::AppConsentRequest",
            "crate::access_reviews::AccessReviewsRequest"
        ],
        ResourceIdentity::Sites => vec![
            "crate::content_types::{ContentTypeRequest, ContentTypesRequest}",
            "crate::lists::{ListRequest, ListsRequest}",
            "crate::drive::DrivesRequest",
            "crate::onenote::OnenoteRequest",
        ],
        ResourceIdentity::Onenote => vec![
            "crate::notebooks::{NotebookRequest, NotebooksRequest}",
            "crate::pages::{PagesRequest, PageRequest}",
            "crate::sections::{SectionRequest, SectionsRequest}",
            "crate::section_groups::{SectionGroupRequest, SectionGroupsRequest}",
        ],
        ResourceIdentity::Pages => vec![
            "crate::parent_notebook::ParentNotebookRequest",
            "crate::parent_section::ParentSectionRequest",
            "graph_http::{BlockingDownload, AsyncDownload, BlockingHttpClient, AsyncHttpClient, \
             RequestClient}",
            "std::path::Path",
        ],
        ResourceIdentity::Planner => vec![
            "crate::plans::{PlansRequest, PlansIdRequest}",
            "crate::buckets::{BucketsRequest, BucketsIdRequest}",
            "crate::tasks::{TasksRequest, TasksIdRequest}",
        ],
        ResourceIdentity::Notebooks => vec![
            "crate::sections::SectionsRequest",
            "crate::section_groups::SectionGroupsRequest",
        ],
        ResourceIdentity::SectionGroups => vec!["crate::sections::SectionsRequest"],
        ResourceIdentity::Sections => vec![
            "crate::pages::PagesRequest",
            "crate::section_groups::SectionGroupsRequest",
            "crate::parent_notebook::ParentNotebookRequest",
            "crate::parent_section_group::ParentSectionGroupRequest",
        ],
        ResourceIdentity::ParentNotebook => vec![
            "crate::sections::SectionsRequest",
            "crate::section_groups::SectionGroupsRequest",
        ],
        ResourceIdentity::ParentSectionGroup => vec![
            "crate::sections::SectionsRequest",
            "crate::section_groups::SectionGroupsRequest",
            "crate::parent_notebook::ParentNotebookRequest",
        ],
        ResourceIdentity::ParentSection => vec![
            "crate::pages::PagesRequest",
            "crate::parent_section_group::ParentSectionGroupRequest",
            "crate::parent_notebook::ParentNotebookRequest",
        ],
        ResourceIdentity::Plans => vec![
            "crate::buckets::{BucketsRequest, BucketsIdRequest}",
            "crate::tasks::{TasksRequest, TasksIdRequest}",
        ],
        ResourceIdentity::Posts => vec![
            "crate::core::ResourceIdentity",
            "crate::extended_properties::ExtendedPropertiesRequest",
            "crate::attachments::{AttachmentRequest, AttachmentsRequest}",
        ],
        ResourceIdentity::PrimaryChannel => vec![
            "crate::chats_channels_messages::{ChatsAndChannelsMessagesIdRequest, ChatsAndChannelsMessagesRequest}",
            "crate::tabs::{TabsIdRequest, TabsRequest}",
            "crate::members::{MembersIdRequest, MembersRequest}",
            "crate::shared_with_teams::{SharedWithTeamsIdRequest, SharedWithTeamsRequest}",
        ],
        ResourceIdentity::ManagedDevices => vec!["crate::core::ResourceIdentity"],
        ResourceIdentity::MailFolders => vec![
            "crate::messages::{MessageRequest, MessagesRequest}",
            "crate::child_folders::{ChildFolderRequest, ChildFoldersRequest}",
            "crate::extended_properties::ExtendedPropertiesRequest",
        ],
        ResourceIdentity::Messages => vec![
            "crate::extended_properties::ExtendedPropertiesRequest",
            "crate::attachments::{AttachmentRequest, AttachmentsRequest}",
        ],
        ResourceIdentity::Me => vec![
            "crate::calendar_groups::{CalendarGroupRequest, CalendarGroupsRequest}",
            "crate::calendar_view::{CalendarViewRequest, CalendarViewsRequest}",
            "crate::calendar::{CalendarRequest, CalendarsRequest}",
            "crate::education::{MeRequest as EducationMeRequest}",
            "crate::events::{EventsRequest, EventRequest}",
            "crate::managed_devices::{ManagedDeviceRequest, ManagedDevicesRequest}",
            "crate::contact_folders::{ContactFolderRequest, ContactFoldersRequest}",
            "crate::messages::{MessageRequest, MessagesRequest}",
            "crate::mail_folders::{MailFolderRequest, MailFoldersRequest}",
            "crate::insights::InsightsRequest",
            "crate::inference_classification::InferenceClassificationRequest",
            "crate::activities::ActivitiesRequest",
            "crate::settings::SettingsRequest",
            "crate::outlook::OutlookRequest",
            "crate::drive::DrivesRequest",
            "crate::onenote::OnenoteRequest",
            "crate::contacts::{ContactRequest, ContactsRequest}",
            "crate::planner::PlannerRequest",
        ],
        ResourceIdentity::Users => vec![
            "crate::calendar_groups::{CalendarGroupRequest, CalendarGroupsRequest}",
            "crate::calendar_view::{CalendarViewRequest, CalendarViewsRequest}",
            "crate::calendar::{CalendarRequest, CalendarsRequest}",
            "crate::education::{UsersRequest as EducationUsersRequest}",
            "crate::events::{EventsRequest, EventRequest}",
            "crate::managed_devices::{ManagedDeviceRequest, ManagedDevicesRequest}",
            "crate::contact_folders::{ContactFolderRequest, ContactFoldersRequest}",
            "crate::messages::{MessageRequest, MessagesRequest}",
            "crate::mail_folders::{MailFolderRequest, MailFoldersRequest}",
            "crate::insights::InsightsRequest",
            "crate::inference_classification::InferenceClassificationRequest",
            "crate::activities::ActivitiesRequest",
            "crate::settings::SettingsRequest",
            "crate::outlook::OutlookRequest",
            "crate::drive::DrivesRequest",
            "crate::onenote::OnenoteRequest",
            "crate::contacts::{ContactRequest, ContactsRequest}",
            "crate::planner::PlannerRequest",
        ],
        ResourceIdentity::Groups => vec![
            "crate::calendar_groups::{CalendarGroupRequest, CalendarGroupsRequest}",
            "crate::calendar_view::{CalendarViewRequest, CalendarViewsRequest}",
            "crate::calendar::{CalendarRequest, CalendarsRequest}",
            "crate::events::{EventsRequest, EventRequest}",
            "crate::drive::DrivesRequest",
            "crate::onenote::OnenoteRequest",
            "crate::threads::{ThreadRequest, ThreadsRequest}",
            "crate::conversations::{ConversationRequest, ConversationsRequest}",
            "crate::planner::PlannerRequest",
        ],
        ResourceIdentity::ServiceAnnouncement => vec![
            "crate::messages::{MessageRequest, MessagesRequest}",
            "crate::health_overviews::{HealthOverviewsIdRequest, HealthOverviewsRequest}",
        ],
        ResourceIdentity::Teams => {
            vec![
                "crate::channels::{ChannelsIdRequest, ChannelsRequest}",
                "crate::members::{MembersIdRequest, MembersRequest}",
                "crate::primary_channel::PrimaryChannelRequest",
                "crate::schedule::ScheduleRequest"
            ]
        }
        ResourceIdentity::TermsOfUse => vec![
            "crate::agreement_acceptances::{AgreementAcceptancesIdRequest, AgreementAcceptancesRequest}",
            "crate::agreements::{AgreementsIdRequest, AgreementsRequest}"
        ],
        ResourceIdentity::Threads => vec!["crate::posts::{PostRequest, PostsRequest}"],
        _ => vec![],
    }
}
