// GENERATED CODE
use crate::api_default_imports::*;
use crate::extended_properties::*;
use crate::users::*;

resource_api_client!(
    MailFoldersApiClient,
    MailFoldersIdApiClient,
    ResourceIdentity::MailFolders
);

impl MailFoldersApiClient {
    post!(
        doc: "Create MailFolder",
        name: create_mail_folders,
        path: "/mailFolders",
        body: true
    );
    get!(
        doc: "List mailFolders",
        name: list_mail_folders,
        path: "/mailFolders"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_mail_folders_count,
        path: "/mailFolders/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/mailFolders/delta()"
    );
}
