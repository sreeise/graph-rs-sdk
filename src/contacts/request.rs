use crate::client::Graph;
use crate::http::{GraphResponse, IntoResponse};
use crate::types::{collection::Collection, content::Content, delta::DeltaRequest};
use graph_rs_types::entitytypes::{Contact, ContactFolder};
use handlebars::*;
use reqwest::Method;

register_client!(
    ContactsRequest,
    ct => "contacts",
    cf => "contactfolders",
);

impl<'a> ContactsRequest<'a> {
    get!( delta, DeltaRequest<Collection<Contact>> => "{{ct}}/delta" );
    get!( list, Collection<Contact> => "{{ct}}" );
    get!( | get, Contact => "{{ct}}/{{id}}" );
    post!( [ create, Contact => "{{ct}}" ] );
    patch!( [ | update, Contact => "{{ct}}/{{id}}" ] );
    delete!( | delete, GraphResponse<Content> => "{{ct}}/{{id}}" );

    pub fn contacts_folder(&'a self) -> ContactsFolderRequest<'a> {
        ContactsFolderRequest::new(self.client)
    }
}

register_client!(ContactsFolderRequest,);

impl<'a> ContactsFolderRequest<'a> {
    get!( delta, DeltaRequest<Collection<ContactFolder>> => "{{cf}}/delta" );
    get!( | get, ContactFolder => "{{cf}}/{{id}}" );
    get!( | list_child_folders, Collection<ContactFolder> => "{{cf}}/{{id}}/childFolders" );
    post!( [ | create_child_folder, ContactFolder => "{{cf}}/{{id}}/childFolders" ] );
    patch!( [ | update, ContactFolder => "{{cf}}/{{id}}" ] );
    delete!( | delete, GraphResponse<Content> => "{{cf}}/{{id}}" );

    pub fn contacts(&'a self) -> ContactsFolderContactsRequest<'a> {
        ContactsFolderContactsRequest::new(self.client)
    }
}

register_client!(ContactsFolderContactsRequest,);

impl<'a> ContactsFolderContactsRequest<'a> {
    get!( | delta, DeltaRequest<Collection<Contact>> => "{{cf}}/{{id}}/{{ct}}/delta" );
    get!( | list, Collection<Contact> => "{{cf}}/{{id}}/{{ct}}" );
    post!( [ | create, Contact => "{{cf}}/{{id}}" ] );
    delete!( || delete, GraphResponse<Content> => "{{cf}}/{{id}}/{{ct}}/{{id2}}" );
}
