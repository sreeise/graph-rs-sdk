// GENERATED CODE

use crate::api_default_imports::*;

api_client!(SettingsApiClient, ResourceIdentity::Settings);

impl SettingsApiClient {
    delete!(
        doc: "Delete navigation property settings for users",
        name: delete_settings,
        path: "/settings"
    );
    get!(
        doc: "Get settings from users",
        name: get_settings,
        path: "/settings"
    );
    patch!(
        doc: "Update the navigation property settings in users",
        name: update_settings,
        path: "/settings",
        body: true
    );
    delete!(
        doc: "Delete navigation property shiftPreferences for users",
        name: delete_shift_preferences,
        path: "/settings/shiftPreferences"
    );
    get!(
        doc: "Get shiftPreferences",
        name: get_shift_preferences,
        path: "/settings/shiftPreferences"
    );
    patch!(
        doc: "Update shiftPreferences",
        name: update_shift_preferences,
        path: "/settings/shiftPreferences",
        body: true
    );
}
