use inflector::Inflector;
use std::convert::AsRef;

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
    Activities,
    AppCatalogs,
    Applications,
    Attachments,
    AuditLogs,
    Calendar,
    CalendarGroup,
    CalendarGroups,
    Calendars,
    CalendarView,
    CalendarViews,
    CertificateBasedAuthConfiguration,
    Communications,
    Contacts,
    ContactFolders,
    ContentTypes,
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
    Event,
    Events,
    GroupLifecyclePolicies,
    GroupSettingTemplates,
    GroupSettings,
    Groups,
    HistoryItems,
    Identity,
    InferenceClassification,
    InformationProtection,
    Insights,
    Instances,
    Invitations,
    Items,
    List,
    Lists,
    MailFolders,
    ManagedDevices,
    Me,
    Messages,
    Oauth2PermissionGrants,
    Organization,
    OnlineMeetings,
    Outlook,
    Places,
    Planner,
    Policies,
    Reports,
    SchemaExtensions,
    Security,
    ServicePrincipals,
    Settings,
    Shares,
    Sites,
    SubscribedSkus,
    Subscriptions,
    Teams,
    Teamwork,
    Users,
    Workbooks,
}

impl ToString for ResourceIdentity {
    fn to_string(&self) -> String {
        self.as_ref().to_camel_case()
    }
}

impl Default for ResourceIdentity {
    fn default() -> Self {
        ResourceIdentity::Me
    }
}

impl ResourceIdentity {
    pub fn enum_string(&self) -> String {
        format!("ResourceIdentity::{:#?}", self)
    }
}
