use graph_core::resource::ResourceIdentity;

pub fn get_doc_comment_replace_filter(resource_identity: ResourceIdentity) -> Vec<String> {
    match resource_identity {
        ResourceIdentity::Buckets | ResourceIdentity::Plans | ResourceIdentity::Tasks => {
            vec![" in planner", " from planner", " for planner"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        }
        _ => vec![], /*
                     ResourceIdentity::Activities => {}
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
