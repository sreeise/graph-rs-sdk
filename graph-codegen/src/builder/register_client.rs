use bytes::Bytes;
use graph_core::resource::ResourceIdentity;
use inflector::Inflector;

#[derive(Debug, Clone, Copy)]
pub enum RegisterClient {
    BaseClient,
    IdentClient,
}

impl RegisterClient {
    pub fn format(self, name: &str) -> Bytes {
        Bytes::copy_from_slice(self.register_client(name.as_ref()).as_bytes())
    }

    fn register_client(&self, client_name: &str) -> String {
        let ends_with = client_name.ends_with("Request");
        let client_pascal_casing = client_name.to_pascal_case();
        match self {
            RegisterClient::BaseClient => {
                if ends_with {
                    format!("register_client!({},);\n", client_pascal_casing)
                } else {
                    format!("register_client!({}Request,);\n", client_pascal_casing)
                }
            }
            RegisterClient::IdentClient => {
                if ends_with {
                    format!("register_client!({}, ());\n", client_pascal_casing)
                } else {
                    format!("register_client!({}Request, ());\n", client_pascal_casing)
                }
            }
        }
    }

    pub fn from_resource_identity(resource_identity: ResourceIdentity) -> String {
        match resource_identity {
            ResourceIdentity::Activities => {
                RegisterClient::BaseClient.register_client(&resource_identity.to_string())
            }
            _ => RegisterClient::BaseClient.register_client(&resource_identity.to_string()),
            /*
            ResourceIdentity::AppCatalogs => {}
            ResourceIdentity::Application => {}
            ResourceIdentity::Applications => {}
            ResourceIdentity::Attachments => {}
            ResourceIdentity::AuditLogs => {}
            ResourceIdentity::Buckets => {}
            ResourceIdentity::Calendar => {}
            ResourceIdentity::CalendarGroup => {}
            ResourceIdentity::CalendarGroups => {}
            ResourceIdentity::CalendarView => {}
            ResourceIdentity::CalendarViews => {}
            ResourceIdentity::Calendars => {}
            ResourceIdentity::CallRecord => {}
            ResourceIdentity::CallRecords => {}
            ResourceIdentity::Calls => {}
            ResourceIdentity::CertificateBasedAuthConfiguration => {}
            ResourceIdentity::ChildFolders => {}
            ResourceIdentity::Communications => {}
            ResourceIdentity::ContactFolders => {}
            ResourceIdentity::Contacts => {}
            ResourceIdentity::ContentTypes => {}
            ResourceIdentity::Contracts => {}
            ResourceIdentity::Conversations => {}
            ResourceIdentity::DataPolicyOperations => {}
            ResourceIdentity::DeviceAppManagement => {}
            ResourceIdentity::DeviceManagement => {}
            ResourceIdentity::Devices => {}
            ResourceIdentity::Directory => {}
            ResourceIdentity::DirectoryObjects => {}
            ResourceIdentity::DirectoryRoleTemplates => {}
            ResourceIdentity::DirectoryRoles => {}
            ResourceIdentity::DomainDnsRecords => {}
            ResourceIdentity::Domains => {}
            ResourceIdentity::Drive => {}
            ResourceIdentity::Drives => {}
            ResourceIdentity::Education => {}
            ResourceIdentity::Event => {}
            ResourceIdentity::Events => {}
            ResourceIdentity::ExtendedProperties => {}
            ResourceIdentity::GroupLifecyclePolicies => {}
            ResourceIdentity::GroupSettingTemplates => {}
            ResourceIdentity::GroupSettings => {}
            ResourceIdentity::Groups => {}
            ResourceIdentity::HistoryItems => {}
            ResourceIdentity::Identity => {}
            ResourceIdentity::InferenceClassification => {}
            ResourceIdentity::InformationProtection => {}
            ResourceIdentity::Insights => {}
            ResourceIdentity::Instances => {}
            ResourceIdentity::Invitations => {}
            ResourceIdentity::Items => {}
            ResourceIdentity::List => {}
            ResourceIdentity::Lists => {}
            ResourceIdentity::MailFolders => {}
            ResourceIdentity::ManagedDevices => {}
            ResourceIdentity::Me => {}
            ResourceIdentity::Messages => {}
            ResourceIdentity::Notebooks => {}
            ResourceIdentity::Oauth2PermissionGrants => {}
            ResourceIdentity::Onenote => {}
            ResourceIdentity::OnlineMeetings => {}
            ResourceIdentity::OrgContact => {}
            ResourceIdentity::Organization => {}
            ResourceIdentity::Outlook => {}
            ResourceIdentity::Pages => {}
            ResourceIdentity::ParentNotebook => {}
            ResourceIdentity::ParentSection => {}
            ResourceIdentity::ParentSectionGroup => {}
            ResourceIdentity::Places => {}
            ResourceIdentity::Planner => {}
            ResourceIdentity::Plans => {}
            ResourceIdentity::Policies => {}
            ResourceIdentity::Posts => {}
            ResourceIdentity::Reports => {}
            ResourceIdentity::SchemaExtensions => {}
            ResourceIdentity::SectionGroups => {}
            ResourceIdentity::Sections => {}
            ResourceIdentity::Security => {}
            ResourceIdentity::ServicePrincipals => {}
            ResourceIdentity::Sessions => {}
            ResourceIdentity::Settings => {}
            ResourceIdentity::Shares => {}
            ResourceIdentity::Sites => {}
            ResourceIdentity::SubscribedSkus => {}
            ResourceIdentity::Subscriptions => {}
            ResourceIdentity::Tasks => {}
            ResourceIdentity::Teams => {}
            ResourceIdentity::Teamwork => {}
            ResourceIdentity::Threads => {}
            ResourceIdentity::Users => {}
            ResourceIdentity::Workbooks => {}
             */
        }
    }
}
