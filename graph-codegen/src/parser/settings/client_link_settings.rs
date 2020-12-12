use crate::builder::ClientLinkSettings;
use graph_core::resource::ResourceIdentity;
use std::collections::{BTreeMap, BTreeSet};

pub fn get_client_link_settings(
    resource_identity: ResourceIdentity,
) -> BTreeMap<String, BTreeSet<ClientLinkSettings>> {
    let mut map = BTreeMap::new();
    match resource_identity {
        ResourceIdentity::Buckets => {
            let mut settings = ClientLinkSettings::new("tasks");
            settings
                .use_method_name("task")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("task");
            settings2
                .use_method_name("tasks")
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2]);
            map.insert("buckets".to_string(), set);
        },
        ResourceIdentity::Calendar | ResourceIdentity::Calendars => {
            let mut settings = ClientLinkSettings::new("events");
            settings
                .use_method_name("event")
                .with_extend_path_ident()
                .with_set_resource_identity()
                .with_id_param();

            let mut settings2 = ClientLinkSettings::new("event");
            settings2
                .use_method_name("events")
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings3 = ClientLinkSettings::new("calendars");
            settings3.as_id_method_link();

            let mut settings4 = ClientLinkSettings::new("calendarView");
            settings4
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings5 = ClientLinkSettings::new("calendarViews");
            settings5
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2, settings3, settings4, settings5]);
            map.insert("calendar".to_string(), set);

            let mut settings4 = ClientLinkSettings::new("calendarView");
            settings4
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings5 = ClientLinkSettings::new("calendarViews");
            settings5
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings6 = ClientLinkSettings::new("events");
            settings6
                .use_method_name("event")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings7 = ClientLinkSettings::new("event");
            settings7
                .use_method_name("events")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings8 = ClientLinkSettings::new("extendedProperties");
            settings8
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set2 = BTreeSet::new();
            set2.extend(vec![settings4, settings5, settings6, settings7, settings8]);
            map.insert("calendars".to_string(), set2);
        },
        ResourceIdentity::CalendarGroup | ResourceIdentity::CalendarGroups => {
            let mut settings = ClientLinkSettings::new("calendars");
            settings
                .use_method_name("calendar")
                .with_id_param()
                .with_extend_path_id()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("calendar");
            settings2
                .use_method_name("calendars")
                .with_extend_path_id()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings3 = ClientLinkSettings::new("events");
            settings3
                .use_method_name("event")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings4 = ClientLinkSettings::new("event");
            settings4
                .use_method_name("events")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2, settings3, settings4]);
            map.insert("calendarGroups".to_string(), set);

            let mut settings5 = ClientLinkSettings::new("events");
            settings5
                .use_method_name("event")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings6 = ClientLinkSettings::new("event");
            settings6
                .use_method_name("events")
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings7 = ClientLinkSettings::new("calendars");
            settings7
                .use_method_name("calendar")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings8 = ClientLinkSettings::new("calendar");
            settings8
                .use_method_name("calendars")
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings5, settings6, settings7, settings8]);
            map.insert("calendarGroup".to_string(), set);
        },
        ResourceIdentity::CalendarView | ResourceIdentity::CalendarViews => {
            let mut settings = ClientLinkSettings::new("instances");
            settings
                .use_method_name("instance")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity()
                .with_id_param();

            let mut settings2 = ClientLinkSettings::new("instance");
            settings2
                .use_method_name("instances")
                .with_extend_path_ident()
                .with_extend_path_id();

            let mut settings3 = ClientLinkSettings::new("calendar");
            settings3
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings4 = ClientLinkSettings::new("extendedProperties");
            settings4
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings5 = ClientLinkSettings::new("attachments");
            settings5
                .with_id_param()
                .use_method_name("attachment")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings6 = ClientLinkSettings::new("attachment");
            settings6
                .use_method_name("attachments")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![
                settings, settings2, settings3, settings4, settings5, settings6,
            ]);
            map.insert("calendarView".to_string(), set);
        },
        ResourceIdentity::Events => {
            let mut settings = ClientLinkSettings::new("calendar");
            settings
                .with_extend_path_id()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("instances");
            settings2
                .use_method_name("instance")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity()
                .with_id_param();

            let mut settings3 = ClientLinkSettings::new("instance");
            settings3
                .use_method_name("instances")
                .with_extend_path_ident()
                .with_extend_path_id();

            let mut settings4 = ClientLinkSettings::new("extendedProperties");
            settings4
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings5 = ClientLinkSettings::new("attachments");
            settings5
                .with_id_param()
                .use_method_name("attachment")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings6 = ClientLinkSettings::new("attachment");
            settings6
                .use_method_name("attachments")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![
                settings, settings2, settings3, settings4, settings5, settings6,
            ]);
            map.insert("events".to_string(), set);
        },
        ResourceIdentity::Conversations => {
            let mut settings = ClientLinkSettings::new("threads");
            settings
                .use_method_name("thread")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("thread");
            settings2
                .use_method_name("threads")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings3 = ClientLinkSettings::new("extendedProperties");
            settings3
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2, settings3]);
            map.insert("conversations".to_string(), set);
        },
        ResourceIdentity::ContactFolders => {
            let mut settings = ClientLinkSettings::new("extendedProperties");
            settings
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("childFolders");
            settings2
                .use_method_name("childFolder")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings3 = ClientLinkSettings::new("childFolder");
            settings3
                .use_method_name("childFolders")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings4 = ClientLinkSettings::new("contacts");
            settings4
                .use_method_name("contact")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings5 = ClientLinkSettings::new("contact");
            settings5
                .use_method_name("contacts")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2, settings3, settings4, settings5]);
            map.insert("contactFolders".to_string(), set);
        },
        ResourceIdentity::Contacts => {
            let mut settings = ClientLinkSettings::new("extendedProperties");
            settings
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings]);
            map.insert("contacts".to_string(), set);
        },
        ResourceIdentity::ChildFolders => {
            let mut settings = ClientLinkSettings::new("messages");
            settings
                .use_method_name("message")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("message");
            settings2
                .use_method_name("messages")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2]);
            map.insert("childFolders".to_string(), set);
        },
        ResourceIdentity::Drive | ResourceIdentity::Drives => {
            let mut settings = ClientLinkSettings::new("items");
            settings
                .use_method_name("item")
                .use_custom("self.transfer_identity();\n")
                .with_id_param()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("item");
            settings2
                .use_method_name("items")
                .use_custom("self.transfer_identity();\n");

            let mut settings3 = ClientLinkSettings::new("list");
            settings3
                .use_method_name("lists")
                .use_custom("self.transfer_identity();\n")
                .with_set_resource_identity();

            let mut settings4 = ClientLinkSettings::new("lists");
            settings4
                .use_method_name("list")
                .use_custom("self.transfer_identity();\n")
                .with_id_param()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2, settings3, settings4]);
            map.insert("drives".to_string(), set);

            let mut settings5 = ClientLinkSettings::new("item");
            settings5.use_method_name("items").with_extend_path_ident();

            let mut settings6 = ClientLinkSettings::new("list");
            settings6.use_method_name("lists").with_extend_path_ident();

            let mut set2 = BTreeSet::new();
            set2.extend(vec![settings5, settings6]);
            map.insert("drive".to_string(), set2);
        },
        ResourceIdentity::Lists => {
            let mut settings = ClientLinkSettings::new("items");
            settings
                .use_method_name("item")
                .with_id_param()
                .with_extend_path_id()
                .with_extend_path_ident();

            let mut settings2 = ClientLinkSettings::new("item");
            settings2
                .use_method_name("items")
                .with_extend_path_id()
                .with_extend_path_ident();

            let mut settings3 = ClientLinkSettings::new("contentTypes");
            settings3
                .use_method_name("contentType")
                .with_id_param()
                .with_extend_path_id()
                .with_extend_path_ident();

            let mut settings4 = ClientLinkSettings::new("contentType");
            settings4
                .use_method_name("contentTypes")
                .with_extend_path_id()
                .with_extend_path_ident();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2, settings3, settings4]);
            map.insert("lists".to_string(), set);
        },
        ResourceIdentity::Onenote => {
            let mut settings = ClientLinkSettings::new("pages");
            settings
                .use_method_name("page")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("page");
            settings2
                .use_method_name("pages")
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings3 = ClientLinkSettings::new("notebooks");
            settings3
                .use_method_name("notebook")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings4 = ClientLinkSettings::new("notebook");
            settings4
                .use_method_name("notebooks")
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings5 = ClientLinkSettings::new("sectionGroups");
            settings5
                .use_method_name("sectionGroup")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings6 = ClientLinkSettings::new("sectionGroup");
            settings6
                .use_method_name("sectionGroups")
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings7 = ClientLinkSettings::new("sections");
            settings7
                .use_method_name("section")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings8 = ClientLinkSettings::new("section");
            settings8
                .use_method_name("sections")
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![
                settings, settings2, settings3, settings4, settings5, settings6, settings7,
                settings8,
            ]);
            map.insert("onenote".to_string(), set);
        },
        ResourceIdentity::MailFolders => {
            let mut settings = ClientLinkSettings::new("extendedProperties");
            settings
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("messages");
            settings2
                .use_method_name("message")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings3 = ClientLinkSettings::new("message");
            settings3
                .use_method_name("messages")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings4 = ClientLinkSettings::new("childFolders");
            settings4
                .use_method_name("childFolder")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings5 = ClientLinkSettings::new("childFolder");
            settings5
                .use_method_name("childFolders")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2, settings3, settings4, settings5]);
            map.insert("mailFolders".to_string(), set);
        },
        ResourceIdentity::Messages => {
            let mut settings = ClientLinkSettings::new("extendedProperties");
            settings
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("attachments");
            settings2
                .with_id_param()
                .use_method_name("attachment")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings3 = ClientLinkSettings::new("attachment");
            settings3
                .use_method_name("attachments")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2, settings3]);
            map.insert("messages".to_string(), set);
        },
        ResourceIdentity::Notebooks => {
            let mut settings = ClientLinkSettings::new("sectionGroups");
            settings
                .use_method_name("sectionGroup")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("sections");
            settings2
                .use_method_name("section")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2]);
            map.insert("notebooks".to_string(), set);
        },
        ResourceIdentity::Sections => {
            let mut settings = ClientLinkSettings::new("pages");
            settings
                .use_method_name("page")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("parentNotebook");
            settings2
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings3 = ClientLinkSettings::new("parentSectionGroup");
            settings3
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2, settings3]);
            map.insert("sections".to_string(), set);
        },
        ResourceIdentity::Pages => {
            let mut settings = ClientLinkSettings::new("parentNotebook");
            settings
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("parentSection");
            settings2
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2]);
            map.insert("pages".to_string(), set);
        },
        ResourceIdentity::ParentSection => {
            let mut settings = ClientLinkSettings::new("pages");
            settings
                .use_method_name("page")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("parentSectionGroup");
            settings2
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings3 = ClientLinkSettings::new("parentNotebook");
            settings3
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2, settings3]);
            map.insert("parentSection".to_string(), set);
        },
        ResourceIdentity::SectionGroups => {
            let mut settings = ClientLinkSettings::new("sections");
            settings
                .use_method_name("section")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings]);
            map.insert("sectionGroups".to_string(), set);
        },
        ResourceIdentity::ParentNotebook => {
            let mut settings = ClientLinkSettings::new("sections");
            settings
                .use_method_name("section")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("sectionGroups");
            settings2
                .use_method_name("sectionGroup")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2]);
            map.insert("parentNotebook".to_string(), set);
        },
        ResourceIdentity::ParentSectionGroup => {
            let mut settings = ClientLinkSettings::new("sections");
            settings
                .use_method_name("section")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("sectionGroups");
            settings2
                .use_method_name("sectionGroup")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings3 = ClientLinkSettings::new("parentNotebook");
            settings3
                .with_extend_path_ident()
                .with_set_resource_identity();

            // Parent section group has requests that have the path /parentSectionGroup/parentSectionGroup
            // and these requests have the same method name, such as get_parent_section_group, which
            // is a conflict because parent section group already has that method name. So we remove
            // those requests entirely and provide a link to the same parent section group which
            // will extend the path correctly when calling these methods.
            let mut settings4 = ClientLinkSettings::new("parentSectionGroup");
            settings4.with_extend_path_ident();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2, settings3, settings4]);
            map.insert("parentSectionGroup".to_string(), set);
        },
        ResourceIdentity::Plans => {
            let mut settings = ClientLinkSettings::new("buckets");
            settings
                .use_method_name("bucket")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("bucket");
            settings2
                .use_method_name("buckets")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings3 = ClientLinkSettings::new("tasks");
            settings3
                .use_method_name("task")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings4 = ClientLinkSettings::new("task");
            settings4
                .use_method_name("tasks")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2, settings3, settings4]);
            map.insert("plans".to_string(), set);
        },
        ResourceIdentity::Posts => {
            let mut settings = ClientLinkSettings::new("extendedProperties");
            settings
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("attachments");
            settings2
                .with_id_param()
                .use_method_name("attachment")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings3 = ClientLinkSettings::new("attachment");
            settings3
                .use_method_name("attachments")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2, settings3]);
            map.insert("posts".to_string(), set);
        },
        ResourceIdentity::CallRecords => {
            let mut settings = ClientLinkSettings::new("sessions");
            settings
                .use_method_name("session")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("session");
            settings2
                .use_method_name("sessions")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2]);
            map.insert("callRecords".to_string(), set);
        },
        ResourceIdentity::Communications => {
            let mut settings = ClientLinkSettings::new("callRecords");
            settings
                .use_method_name("callRecord")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("callRecord");
            settings2
                .use_method_name("callRecords")
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings3 = ClientLinkSettings::new("calls");
            settings3
                .use_method_name("call")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings4 = ClientLinkSettings::new("call");
            settings4
                .use_method_name("calls")
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2, settings3, settings4]);
            map.insert("communications".to_string(), set);
        },
        ResourceIdentity::Planner => {
            let mut settings = ClientLinkSettings::new("plans");
            settings
                .use_method_name("plan")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("plan");
            settings2
                .use_method_name("plans")
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings3 = ClientLinkSettings::new("buckets");
            settings3
                .use_method_name("bucket")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings4 = ClientLinkSettings::new("bucket");
            settings4
                .use_method_name("buckets")
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings5 = ClientLinkSettings::new("tasks");
            settings5
                .use_method_name("task")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings6 = ClientLinkSettings::new("task");
            settings6
                .use_method_name("tasks")
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![
                settings, settings2, settings3, settings4, settings5, settings6,
            ]);
            map.insert("planner".to_string(), set);
        },
        ResourceIdentity::Me => {
            let mut settings = ClientLinkSettings::new("calendarGroups");
            settings
                .use_method_name("calendarGroup")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("calendarGroup");
            settings2
                .use_method_name("calendarGroups")
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings3 = ClientLinkSettings::new("calendars");
            settings3
                .use_method_name("calendar")
                .with_id_param()
                .with_set_resource_identity()
                .with_extend_path_ident();

            let mut settings4 = ClientLinkSettings::new("calendar");
            settings4
                .use_method_name("calendars")
                .with_set_resource_identity()
                .with_extend_path_ident();

            let mut settings5 = ClientLinkSettings::new("event");
            settings5
                .use_method_name("events")
                .with_set_resource_identity()
                .with_extend_path_ident();

            let mut settings6 = ClientLinkSettings::new("events");
            settings6
                .use_method_name("event")
                .with_id_param()
                .with_set_resource_identity()
                .with_extend_path_ident();

            let mut settings7 = ClientLinkSettings::new("calendarView");
            settings7
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings8 = ClientLinkSettings::new("calendarViews");
            settings8
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings9 = ClientLinkSettings::new("educationMe");
            settings9.use_method_name("education");

            let mut settings10 = ClientLinkSettings::new("insights");
            settings10
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings11 = ClientLinkSettings::new("managedDevices");
            settings11
                .use_method_name("managed_device")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings12 = ClientLinkSettings::new("managedDevice");
            settings12
                .use_method_name("managed_devices")
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings13 = ClientLinkSettings::new("inferenceClassification");
            settings13
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings14 = ClientLinkSettings::new("contactFolders");
            settings14
                .use_method_name("contact_folder")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings15 = ClientLinkSettings::new("contactFolder");
            settings15
                .use_method_name("contact_folders")
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings16 = ClientLinkSettings::new("activities");
            settings16
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings17 = ClientLinkSettings::new("settings");
            settings17
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings18 = ClientLinkSettings::new("outlook");
            settings18
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings19 = ClientLinkSettings::new("drives");
            settings19
                .use_method_name("drive")
                .with_extend_path_ident()
                .with_new_method_empty_id()
                .with_new_method_empty_id();

            let mut settings20 = ClientLinkSettings::new("onenote");
            settings20
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings21 = ClientLinkSettings::new("contacts");
            settings21
                .use_method_name("contact")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings22 = ClientLinkSettings::new("contact");
            settings22
                .use_method_name("contacts")
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings23 = ClientLinkSettings::new("mailFolders");
            settings23
                .use_method_name("mailFolder")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings24 = ClientLinkSettings::new("mailFolder");
            settings24
                .use_method_name("mailFolders")
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings25 = ClientLinkSettings::new("messages");
            settings25
                .use_method_name("message")
                .with_id_param()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings26 = ClientLinkSettings::new("message");
            settings26
                .use_method_name("messages")
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings27 = ClientLinkSettings::new("planner");
            settings27
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![
                settings, settings2, settings3, settings4, settings5, settings6, settings7,
                settings8, settings9, settings10, settings11, settings12, settings13, settings14,
                settings15, settings16, settings17, settings18, settings19, settings20, settings21,
                settings22, settings23, settings24, settings25, settings26, settings27,
            ]);
            map.insert("me".to_string(), set);
        },
        ResourceIdentity::Groups => {
            let mut settings = ClientLinkSettings::new("calendarGroups");
            settings
                .use_method_name("calendarGroup")
                .with_id_param()
                .with_extend_path_id()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("calendarGroup");
            settings2
                .use_method_name("calendarGroups")
                .with_extend_path_id()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings3 = ClientLinkSettings::new("calendars");
            settings3
                .use_method_name("calendar")
                .with_id_param()
                .with_set_resource_identity()
                .with_extend_path_id()
                .with_extend_path_ident();

            let mut settings4 = ClientLinkSettings::new("calendar");
            settings4
                .use_method_name("calendars")
                .with_set_resource_identity()
                .with_extend_path_id()
                .with_extend_path_ident();

            let mut settings5 = ClientLinkSettings::new("event");
            settings5
                .use_method_name("events")
                .with_set_resource_identity()
                .with_extend_path_id()
                .with_extend_path_ident();

            let mut settings6 = ClientLinkSettings::new("events");
            settings6
                .use_method_name("event")
                .with_id_param()
                .with_set_resource_identity()
                .with_extend_path_id()
                .with_extend_path_ident();

            let mut settings7 = ClientLinkSettings::new("calendarView");
            settings7
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings8 = ClientLinkSettings::new("calendarViews");
            settings8
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings9 = ClientLinkSettings::new("drives");
            settings9
                .use_method_name("drive")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id();

            let mut settings10 = ClientLinkSettings::new("onenote");
            settings10
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings11 = ClientLinkSettings::new("thread");
            settings11
                .use_method_name("threads")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings12 = ClientLinkSettings::new("threads");
            settings12
                .use_method_name("thread")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings13 = ClientLinkSettings::new("conversations");
            settings13
                .use_method_name("conversation")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings14 = ClientLinkSettings::new("conversation");
            settings14
                .use_method_name("conversations")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings15 = ClientLinkSettings::new("planner");
            settings15
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![
                settings, settings2, settings3, settings4, settings5, settings6, settings7,
                settings8, settings9, settings10, settings11, settings12, settings13, settings14,
                settings15,
            ]);
            map.insert("groups".to_string(), set);
        },
        ResourceIdentity::Sites => {
            let mut settings = ClientLinkSettings::new("contentTypes");
            settings
                .use_method_name("contentType")
                .with_id_param()
                .with_extend_path_id()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("contentType");
            settings2
                .use_method_name("contentTypes")
                .with_extend_path_id()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings3 = ClientLinkSettings::new("list");
            settings3
                .use_method_name("lists")
                .with_extend_path_id()
                .with_extend_path_ident();

            let mut settings4 = ClientLinkSettings::new("lists");
            settings4
                .use_method_name("list")
                .with_id_param()
                .with_extend_path_id()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings5 = ClientLinkSettings::new("drives");
            settings5
                .use_method_name("drive")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_new_method_empty_id();

            let mut settings6 = ClientLinkSettings::new("onenote");
            settings6
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![
                settings, settings2, settings3, settings4, settings5, settings6,
            ]);
            map.insert("sites".to_string(), set);
        },
        ResourceIdentity::Users => {
            let mut settings = ClientLinkSettings::new("calendarGroups");
            settings
                .use_method_name("calendarGroup")
                .with_id_param()
                .with_extend_path_id()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("calendarGroup");
            settings2
                .use_method_name("calendarGroups")
                .with_extend_path_id()
                .with_extend_path_ident()
                .with_set_resource_identity();

            let mut settings3 = ClientLinkSettings::new("calendars");
            settings3
                .use_method_name("calendar")
                .with_id_param()
                .with_set_resource_identity()
                .with_extend_path_id()
                .with_extend_path_ident();

            let mut settings4 = ClientLinkSettings::new("calendar");
            settings4
                .use_method_name("calendars")
                .with_set_resource_identity()
                .with_extend_path_id()
                .with_extend_path_ident();

            let mut settings5 = ClientLinkSettings::new("event");
            settings5
                .use_method_name("events")
                .with_set_resource_identity()
                .with_extend_path_id()
                .with_extend_path_ident();

            let mut settings6 = ClientLinkSettings::new("events");
            settings6
                .use_method_name("event")
                .with_id_param()
                .with_set_resource_identity()
                .with_extend_path_id()
                .with_extend_path_ident();

            let mut settings7 = ClientLinkSettings::new("calendarView");
            settings7
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings8 = ClientLinkSettings::new("calendarViews");
            settings8
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings9 = ClientLinkSettings::new("insights");
            settings9
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings10 = ClientLinkSettings::new("managedDevices");
            settings10
                .use_method_name("managed_device")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings11 = ClientLinkSettings::new("managedDevice");
            settings11
                .use_method_name("managed_devices")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings12 = ClientLinkSettings::new("inferenceClassification");
            settings12
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings13 = ClientLinkSettings::new("contactFolders");
            settings13
                .use_method_name("contact_folder")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings14 = ClientLinkSettings::new("contactFolder");
            settings14
                .use_method_name("contact_folders")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings15 = ClientLinkSettings::new("activities");
            settings15
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings16 = ClientLinkSettings::new("settings");
            settings16
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings17 = ClientLinkSettings::new("outlook");
            settings17
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings18 = ClientLinkSettings::new("drives");
            settings18
                .use_method_name("drive")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_new_method_empty_id();

            let mut settings19 = ClientLinkSettings::new("onenote");
            settings19
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings20 = ClientLinkSettings::new("messages");
            settings20
                .use_method_name("message")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings21 = ClientLinkSettings::new("message");
            settings21
                .use_method_name("messages")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings22 = ClientLinkSettings::new("contacts");
            settings22
                .use_method_name("contact")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings23 = ClientLinkSettings::new("contact");
            settings23
                .use_method_name("contacts")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings24 = ClientLinkSettings::new("mailFolders");
            settings24
                .use_method_name("mailFolder")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings25 = ClientLinkSettings::new("mailFolder");
            settings25
                .use_method_name("mailFolders")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings26 = ClientLinkSettings::new("planner");
            settings26
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![
                settings, settings2, settings3, settings4, settings5, settings6, settings7,
                settings8, settings9, settings10, settings11, settings12, settings13, settings14,
                settings15, settings16, settings17, settings18, settings19, settings20, settings21,
                settings22, settings23, settings24, settings25, settings26,
            ]);
            map.insert("users".to_string(), set);

            let mut user_setting = ClientLinkSettings::new("educationUsers");
            user_setting.use_method_name("education");

            let mut set = BTreeSet::new();
            set.extend(vec![user_setting]);
            map.insert("user".to_string(), set);
        },
        ResourceIdentity::Threads => {
            let mut settings = ClientLinkSettings::new("posts");
            settings
                .use_method_name("post")
                .with_id_param()
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut settings2 = ClientLinkSettings::new("post");
            settings2
                .use_method_name("posts")
                .with_extend_path_ident()
                .with_extend_path_id()
                .with_set_resource_identity();

            let mut set = BTreeSet::new();
            set.extend(vec![settings, settings2]);
            map.insert("threads".to_string(), set);
        },
        _ => {},
    }

    map
}
