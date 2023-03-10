// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(PresenceApiClient, ResourceIdentity::Presence);

impl PresenceApiClient {
    delete!(
        doc: "Delete navigation property presence for users",
        name: delete_presence,
        path: "/presence"
    );
    get!(
        doc: "Get presence",
        name: get_presence,
        path: "/presence"
    );
    patch!(
        doc: "Update the navigation property presence in users",
        name: update_presence,
        path: "/presence",
        body: true
    );
    post!(
        doc: "Invoke action clearPresence",
        name: clear_presence,
        path: "/presence/clearPresence",
        body: true
    );
    post!(
        doc: "Invoke action clearUserPreferredPresence",
        name: clear_user_preferred_presence,
        path: "/presence/clearUserPreferredPresence"
    );
    post!(
        doc: "Invoke action setPresence",
        name: set_presence,
        path: "/presence/setPresence",
        body: true
    );
    post!(
        doc: "Invoke action setUserPreferredPresence",
        name: set_user_preferred_presence,
        path: "/presence/setUserPreferredPresence",
        body: true
    );
}
