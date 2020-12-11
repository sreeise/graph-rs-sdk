use crate::child_folders::ChildFoldersRequest;

impl<'a, Client> ChildFoldersRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn child_folder<S: AsRef<str>>(&self, id: S) -> ChildFoldersRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        ChildFoldersRequest::new(id.as_ref(), self.client)
    }
}
