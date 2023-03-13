use inflector::Inflector;
use std::convert::AsRef;

/// Comprises both top level and second level resources.
/// These are not generated from OpenApi, except for top level resources,
/// and mostly consist of Apis that the project currently has generated.
#[remain::sorted]
#[derive(
    AsRefStr,
    Copy,
    Clone,
    Eq,
    PartialEq,
    EnumString,
    EnumIter,
    Debug,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
)]
#[strum(serialize_all = "camelCase")]
pub enum ResourceIdentity {
    AccessPackageAssignmentApprovals,
    AccessPackages,
    AccessReviews,
    AccessReviewsDefinitions,
    AccessReviewsDefinitionsInstances,
    AccessReviewsDefinitionsInstancesStages,
    Activities,
    Admin,
    AdministrativeUnits,
    AgreementAcceptances,
    Agreements,
    AllChannels,
    AndroidManagedAppProtections,
    AppCatalogs,
    AppConsent,
    Application,
    Applications,
    ApplicationTemplates,
    AppRoleAssignments,
    AssignmentPolicies,
    AssignmentRequests,
    Assignments,
    Attachments,
    AuditLogs,
    Authentication,
    AuthenticationMethodConfigurations,
    AuthenticationMethodsPolicy,
    Batch, // Specifically for $batch requests.
    Branding,
    Buckets,
    CalendarGroups,
    Calendars,
    CalendarView,
    CallRecords,
    CallRecordsSessions,
    Calls,
    CertificateBasedAuthConfiguration,
    Channels,
    Chats,
    ChatsAndChannelsMessages,
    ChatsMessages,
    ChatsMessagesReplies,
    ChildFolders,
    Communications,
    Compliance,
    ConnectedOrganizations,
    ConnectedOrganizationsExternalSponsors,
    ConnectedOrganizationsInternalSponsors,
    Connections,
    ContactFolders,
    Contacts,
    ContentTypes,
    Contracts,
    CreatedObjects,
    DataPolicyOperations,
    DefaultCalendar,
    DefaultManagedAppProtections,
    DeletedItems,
    DeletedTeams,
    DeviceAppManagement,
    DeviceCompliancePolicies,
    DeviceCompliancePolicySettingStateSummaries,
    DeviceConfigurations,
    DeviceEnrollmentConfigurations,
    DeviceManagement,
    DeviceManagementManagedDevices,
    DeviceManagementReports,
    Devices,
    Directory,
    DirectoryMembers,
    DirectoryObjects,
    DirectoryRoles,
    DirectoryRoleTemplates,
    DirectReports,
    DomainDnsRecords,
    Domains,
    Drive,
    Drives,
    DrivesItems,
    DrivesList,
    DrivesListContentTypes,
    Education,
    EducationAssignments,
    EducationAssignmentsSubmissions,
    EducationClasses,
    EducationMe,
    EducationSchools,
    EducationUsers,
    EntitlementManagement,
    EntitlementManagementAssignments,
    EntitlementManagementCatalogs,
    Events,
    EventsInstances,
    ExtendedProperties,
    Extensions,
    External,
    FollowedSites,
    GroupLifecyclePolicies,
    GroupSettings,
    GroupSettingTemplates,
    Groups,
    GroupsConversations,
    GroupsOwners,
    GroupsTeam,
    GroupsThreads,
    GroupsThreadsPosts,
    HistoryItems,
    Identity,
    IdentityGovernance,
    IdentityProtection,
    IdentityProviders,
    IncomingChannels,
    InferenceClassification,
    InformationProtection,
    Insights,
    Invitations,
    IosManagedAppProtections,
    Items,
    JoinedTeams,
    LicenseDetails,
    List,
    Lists,
    Localizations,
    MailFolders,
    ManagedAppPolicies,
    ManagedAppRegistrations,
    ManagedAppRegistrationsAppliedPolicies,
    ManagedAppRegistrationsIntendedPolicies,
    ManagedAppStatuses,
    ManagedEBooks,
    ManagedEBooksDeviceStates,
    ManagedEBooksUserStateSummary,
    MdmWindowsInformationProtectionPolicies,
    Me,
    MemberOf,
    MobileAppCategories,
    MobileAppConfigurations,
    MobileApps,
    Oauth2PermissionGrants,
    Onenote,
    OnenoteNotebooks,
    OnenotePages,
    OnenoteSectionGroups,
    OnenoteSections,
    OnlineMeetings,
    Organization,
    OrgContacts,
    Outlook,
    OwnedDevices,
    OwnedObjects,
    ParentNotebook,
    ParentSection,
    ParentSectionGroup,
    People,
    PermissionGrants,
    Photos,
    Places,
    Planner,
    PlannerTasks,
    Plans,
    Policies,
    Presence,
    PrimaryChannel,
    Print,
    Privacy,
    RegisteredDevices,
    Reports,
    RoleDefinitions,
    RoleManagement,
    Schedule,
    SchemaExtensions,
    ScopedRoleMemberOf,
    ScopedRoleMemberships,
    Search,
    Security,
    ServicePrincipals,
    ServicePrincipalsOwners,
    Settings,
    SharedWithTeams,
    Shares,
    Sites,
    Solutions,
    SubscribedSkus,
    Subscriptions,
    Tabs,
    TargetedManagedAppConfigurations,
    Tasks,
    Team,
    Teams,
    TeamsMembers,
    TeamsPrimaryChannelTabs,
    TeamsTags,
    TeamsTemplates,
    Teamwork,
    TermsAndConditions,
    Todo,
    TodoLists,
    TodoListsTasks,
    TransitiveMemberOf,
    TroubleshootingEvents,
    Users,
    UsersManagedDevices,
    UsersMessages,
    VppTokens,
    WindowsAutopilotDeviceIdentities,
    WindowsInformationProtectionPolicies,
    Workbooks,
}

impl ToString for ResourceIdentity {
    fn to_string(&self) -> String {
        let device_am = ResourceIdentity::DeviceAppManagement.exact_camel_case();

        match self {
            ResourceIdentity::AccessPackages => "accessPackages".to_string(),
            ResourceIdentity::AccessReviewsDefinitions => "definitions".to_string(),
            ResourceIdentity::AccessReviewsDefinitionsInstances => "instances".to_string(),
            ResourceIdentity::AccessReviewsDefinitionsInstancesStages => "stages".to_string(),
            ResourceIdentity::DeviceManagementManagedDevices
            | ResourceIdentity::UsersManagedDevices => "managedDevices".to_string(),
            ResourceIdentity::DeviceManagementReports => "reports".into(),
            ResourceIdentity::EntitlementManagementAssignments => "assignments".to_string(),
            ResourceIdentity::EntitlementManagementCatalogs => "catalogs".to_string(),
            ResourceIdentity::PrimaryChannel => "primaryChannel".to_string(),
            ResourceIdentity::TeamsTags => "tags".to_string(),

            ResourceIdentity::DirectoryMembers | ResourceIdentity::TeamsMembers => {
                "members".to_string()
            }

            ResourceIdentity::SharedWithTeams => "sharedWithTeams".to_string(),
            ResourceIdentity::DrivesList => "list".to_string(),
            ResourceIdentity::DrivesItems => "items".to_string(),
            ResourceIdentity::DrivesListContentTypes => "contentTypes".to_string(),
            ResourceIdentity::GroupsConversations => "conversations".into(),
            ResourceIdentity::GroupsOwners | ResourceIdentity::ServicePrincipalsOwners => {
                "owners".into()
            }
            ResourceIdentity::GroupsTeam => "team".into(),
            ResourceIdentity::GroupsThreadsPosts => "posts".into(),
            ResourceIdentity::GroupsThreads => "threads".into(),
            ResourceIdentity::Activities => "activities".to_string(),
            ResourceIdentity::AgreementAcceptances => "agreementAcceptances".to_string(),
            ResourceIdentity::AppRoleAssignments => "appRoleAssignments".to_string(),
            ResourceIdentity::Authentication => "authentication".to_string(),
            ResourceIdentity::DefaultCalendar => "calendar".to_string(),
            ResourceIdentity::CalendarGroups => "calendarGroups".to_string(),
            ResourceIdentity::CalendarView => "calendarView".to_string(),
            ResourceIdentity::Calendars => "calendars".to_string(),
            ResourceIdentity::Chats => "chats".to_string(),
            ResourceIdentity::ChatsMessages => "messages".to_string(),
            ResourceIdentity::ChatsMessagesReplies => "replies".to_string(),
            ResourceIdentity::ContactFolders => "contactFolders".to_string(),
            ResourceIdentity::Contacts => "contacts".to_string(),
            ResourceIdentity::CreatedObjects => "createdObjects".to_string(),
            ResourceIdentity::DirectReports => "directReports".to_string(),
            ResourceIdentity::Drives => "drives".to_string(),
            ResourceIdentity::Events => "events".to_string(),
            ResourceIdentity::Extensions => "extensions".to_string(),
            ResourceIdentity::FollowedSites => "followedSites".to_string(),
            ResourceIdentity::InferenceClassification => "inferenceClassification".to_string(),
            ResourceIdentity::Insights => "insights".to_string(),
            ResourceIdentity::JoinedTeams => "joinedTeams".to_string(),
            ResourceIdentity::LicenseDetails => "licenseDetails".to_string(),
            ResourceIdentity::MailFolders => "mailFolders".to_string(),
            ResourceIdentity::ManagedAppRegistrations => "managedAppRegistrations".to_string(),
            ResourceIdentity::ManagedAppRegistrationsAppliedPolicies => {
                "appliedPolicies".to_string()
            }
            ResourceIdentity::ManagedAppRegistrationsIntendedPolicies => {
                "intendedPolicies".to_string()
            }
            ResourceIdentity::ManagedEBooksDeviceStates => "deviceStates".to_string(),
            ResourceIdentity::ManagedEBooksUserStateSummary => "userStateSummary".to_string(),
            ResourceIdentity::UsersManagedDevices => "managedDevices".to_string(),
            ResourceIdentity::MemberOf => "memberOf".to_string(),
            ResourceIdentity::Oauth2PermissionGrants => "oauth2PermissionGrants".to_string(),
            ResourceIdentity::Onenote => "onenote".to_string(),
            ResourceIdentity::OnlineMeetings => "onlineMeetings".to_string(),
            ResourceIdentity::Outlook => "outlook".to_string(),
            ResourceIdentity::OwnedDevices => "ownedDevices".to_string(),
            ResourceIdentity::OwnedObjects => "ownedObjects".to_string(),
            ResourceIdentity::People => "people".to_string(),
            ResourceIdentity::Photos => "photos".to_string(),
            ResourceIdentity::Planner => "planner".to_string(),
            ResourceIdentity::Presence => "presence".to_string(),
            ResourceIdentity::RegisteredDevices => "registeredDevices".to_string(),
            ResourceIdentity::ScopedRoleMemberOf => "scopedRoleMemberOf".to_string(),
            ResourceIdentity::Teamwork => "teamwork".to_string(),
            ResourceIdentity::Todo => "todo".to_string(),
            ResourceIdentity::TransitiveMemberOf => "transitiveMemberOf".to_string(),

            ResourceIdentity::ConnectedOrganizationsExternalSponsors => {
                "externalSponsors".to_string()
            }
            ResourceIdentity::ConnectedOrganizationsInternalSponsors => {
                "internalSponsors".to_string()
            }
            ResourceIdentity::CallRecordsSessions => "sessions".to_string(),
            ResourceIdentity::EducationAssignmentsSubmissions => "submissions".to_string(),
            ResourceIdentity::EducationAssignments => "assignments".to_string(),
            ResourceIdentity::EducationClasses => "classes".to_string(),
            ResourceIdentity::EducationMe => "me".to_string(),
            ResourceIdentity::EducationUsers => "users".to_string(),
            ResourceIdentity::EducationSchools => "schools".to_string(),
            ResourceIdentity::TodoLists => "lists".to_string(),
            ResourceIdentity::TodoListsTasks => "tasks".to_string(),
            ResourceIdentity::UsersMessages => "messages".into(),
            ResourceIdentity::EventsInstances => "instances".into(),
            ResourceIdentity::PlannerTasks => "tasks".into(),
            ResourceIdentity::OnenoteSections => "sections".into(),
            ResourceIdentity::OnenoteSectionGroups => "sectionGroups".into(),
            ResourceIdentity::OnenoteNotebooks => "notebooks".into(),
            ResourceIdentity::OnenotePages => "pages".into(),

            _ => self.as_ref().to_camel_case(),
        }
    }
}

impl Default for ResourceIdentity {
    fn default() -> Self {
        ResourceIdentity::Me
    }
}

impl ResourceIdentity {
    pub fn enum_string(&self) -> String {
        format!("ResourceIdentity::{self:#?}")
    }

    pub fn to_path_start(&self) -> String {
        format!("/{}", self.to_string())
    }

    pub fn replace(&self, from: &str, to: &str) -> String {
        self.as_ref().replace(from, to)
    }

    pub fn exact_camel_case(&self) -> String {
        self.as_ref().to_camel_case()
    }

    pub fn exact_pascal_case(&self) -> String {
        self.as_ref().to_pascal_case()
    }

    pub fn exact_snake_case(&self) -> String {
        self.as_ref().to_snake_case()
    }
}

/// Top level resources are the names for the first or beginning part of a URI path.
/// These are generated from the OpenApi config.
#[derive(
    AsRefStr,
    Copy,
    Clone,
    Eq,
    PartialEq,
    EnumString,
    EnumIter,
    Debug,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
)]
#[strum(serialize_all = "camelCase")]
pub enum TopLevelResource {
    Admin,
    AgreementAcceptances,
    Agreements,
    AppCatalogs,
    ApplicationTemplates,
    Applications,
    AuditLogs,
    AuthenticationMethodConfigurations,
    AuthenticationMethodsPolicy,
    Branding,
    CertificateBasedAuthConfiguration,
    Chats,
    Communications,
    Compliance,
    Connections,
    Contacts,
    Contracts,
    DataPolicyOperations,
    DeviceAppManagement,
    DeviceManagement,
    Devices,
    Directory,
    DirectoryObjects,
    DirectoryRoleTemplates,
    DirectoryRoles,
    DomainDnsRecords,
    Domains,
    Drive,
    Drives,
    Education,
    External,
    GroupLifecyclePolicies,
    GroupSettingTemplates,
    GroupSettings,
    Groups,
    Identity,
    IdentityGovernance,
    IdentityProtection,
    IdentityProviders,
    InformationProtection,
    Invitations,
    Localizations,
    Me,
    Oauth2PermissionGrants,
    Organization,
    PermissionGrants,
    Places,
    Planner,
    Policies,
    Print,
    Privacy,
    Reports,
    RoleManagement,
    SchemaExtensions,
    ScopedRoleMemberships,
    Search,
    Security,
    ServicePrincipals,
    Shares,
    Sites,
    Solutions,
    SubscribedSkus,
    Subscriptions,
    Teams,
    TeamsTemplates,
    Teamwork,
    Users,
    Workbooks,
}

impl ToString for TopLevelResource {
    fn to_string(&self) -> String {
        self.as_ref().to_camel_case()
    }
}

impl Default for TopLevelResource {
    fn default() -> Self {
        TopLevelResource::Me
    }
}
