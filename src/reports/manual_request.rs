use crate::api_default_imports::*;
use crate::reports::ReportsApiClient;

impl ReportsApiClient {
    get!(
        doc: "List credentialUserRegistrationDetails",
        name: list_credential_user_registration_details,
        path: "/reports/credentialUserRegistrationDetails"
    );
}
