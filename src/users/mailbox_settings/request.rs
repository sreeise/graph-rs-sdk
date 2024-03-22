use crate::api_default_imports::*;

api_client!(MailboxSettingsApiClient, ResourceIdentity::MailboxSettings);

impl MailboxSettingsApiClient {
    get!(
        doc: "Get mailbox settings",
        name: get_mailbox_settings,
        path: "/mailboxSettings"
    );
    patch!(
        doc: "Update mailbox settings",
        name: update_mailbox_settings,
        path: "/mailboxSettings",
        body: true
    );
    get!(
        doc: "Get mailbox automatic replies settings",
        name: get_automatic_replies_setting,
        path: "/mailboxSettings/automaticRepliesSetting"
    );
    get!(
        doc: "Get mailbox date format settings",
        name: get_date_format,
        path: "/mailboxSettings/dateFormat"
    );
    get!(
        doc: "Get mailbox settings delegate meeting message delivery options",
        name: get_delegate_meeting_message_delivery_options,
        path: "/mailboxSettings/delegateMeetingMessageDeliveryOptions"
    );
    get!(
        doc: "Get mailbox settings language",
        name: get_language,
        path: "/mailboxSettings/language"
    );
    get!(
        doc: "Get mailbox settings time format",
        name: get_time_format,
        path: "/mailboxSettings/timeFormat"
    );
    get!(
        doc: "Get mailbox settings time zone",
        name: get_time_zone,
        path: "/mailboxSettings/timeZone"
    );
    get!(
        doc: "Get mailbox settings working hours",
        name: get_working_hours,
        path: "/mailboxSettings/workingHours"
    );
    get!(
        doc: "Get mailbox settings user purpose",
        name: get_user_purpose,
        path: "/mailboxSettings/userPurpose"
    );
}
