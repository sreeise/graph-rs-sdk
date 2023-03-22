use crate::api_default_imports::*;

fn template(s: &str, last: &str) -> String {
    if s.starts_with(':') {
        format!("{{{{drive_root_path}}}}{{{{id}}}}/{}", last)
    } else {
        format!("{{{{drive_item}}}}/{{{{id}}}}/{}", last)
    }
}

/*
resource_api_client!(
    DefaultDrivesItemsIdApiClient,
    ResourceIdentity::DrivesItems
);

impl DefaultDrivesItemsIdApiClient {
    delete!(
        doc: "Delete navigation property items for drives",
        name: delete_items,
        path: "/drive/root{{RID}}"
    );
    get!(
        doc: "Get items from drives",
        name: get_items,
        path: "/drive/root{{RID}}"
    );
    patch!(
        doc: "Update the navigation property items in drives",
        name: update_items,
        path: "/root{{RID}}",
        body: true
    );
    delete!(
        doc: "Delete navigation property analytics for drives",
        name: delete_analytics,
        path: "/drive/items/{{RID}}/analytics"
    );
    get!(
        doc: "Get analytics from drives",
        name: get_analytics,
        path: "/drive/items/{{RID}}/analytics"
    );
    patch!(
        doc: "Update the navigation property analytics in drives",
        name: update_analytics,
        path: "/drive/items/{{RID}}/analytics",
        body: true
    );
    get!(
        doc: "Get itemAnalytics",
        name: get_all_time,
        path: "/drive/items/{{RID}}/analytics/allTime"
    );
    post!(
        doc: "Create new navigation property to itemActivityStats for drives",
        name: create_item_activity_stats,
        path: "/drive/items/{{RID}}/analytics/itemActivityStats",
        body: true
    );
    get!(
        doc: "Get itemActivityStats from drives",
        name: list_item_activity_stats,
        path: "/drive/items/{{RID}}/analytics/itemActivityStats"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_item_activity_stats_count,
        path: "/drive/items/{{RID}}/analytics/itemActivityStats/$count"
    );
    delete!(
        doc: "Delete navigation property itemActivityStats for drives",
        name: delete_item_activity_stats,
        path: "/drive/items/{{RID}}/analytics/itemActivityStats/{{id}}",
        params: item_activity_stat_id
    );
    get!(
        doc: "Get itemActivityStats from drives",
        name: get_item_activity_stats,
        path: "/drive/items/{{RID}}/analytics/itemActivityStats/{{id}}",
        params: item_activity_stat_id
    );
    patch!(
        doc: "Update the navigation property itemActivityStats in drives",
        name: update_item_activity_stats,
        path: "/drive/items/{{RID}}/analytics/itemActivityStats/{{id}}",
        body: true,
        params: item_activity_stat_id
    );
    post!(
        doc: "Create new navigation property to activities for drives",
        name: create_activities,
        path: "/drive/items/{{RID}}/analytics/itemActivityStats/{{id}}/activities",
        body: true,
        params: item_activity_stat_id
    );
    get!(
        doc: "Get activities from drives",
        name: list_activities,
        path: "/drive/items/{{RID}}/analytics/itemActivityStats/{{id}}/activities",
        params: item_activity_stat_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_activities_count,
        path: "/drive/items/{{RID}}/analytics/itemActivityStats/{{id}}/activities/$count",
        params: item_activity_stat_id
    );
    delete!(
        doc: "Delete navigation property activities for drives",
        name: delete_activities,
        path: "/drive/items/{{RID}}/analytics/itemActivityStats/{{id}}/activities/{{id2}}",
        params: item_activity_stat_id, item_activity_id
    );
    get!(
        doc: "Get activities from drives",
        name: get_activities,
        path: "/drive/items/{{RID}}/analytics/itemActivityStats/{{id}}/activities/{{id2}}",
        params: item_activity_stat_id, item_activity_id
    );
    patch!(
        doc: "Update the navigation property activities in drives",
        name: update_activities,
        path: "/drive/items/{{RID}}/analytics/itemActivityStats/{{id}}/activities/{{id2}}",
        body: true,
        params: item_activity_stat_id, item_activity_id
    );
    get!(
        doc: "Get driveItem from drives",
        name: get_drive_item,
        path: "/drive/items/{{RID}}/analytics/itemActivityStats/{{id}}/activities/{{id2}}/driveItem",
        params: item_activity_stat_id, item_activity_id
    );
    get!(
        doc: "Get content for the navigation property driveItem from drives",
        name: get_drive_item_content,
        path: "/drive/items/{{RID}}/analytics/itemActivityStats/{{id}}/activities/{{id2}}/driveItem/content",
        params: item_activity_stat_id, item_activity_id
    );
    put!(
        doc: "Update content for the navigation property driveItem in drives",
        name: update_drive_item_content,
        path: "/drive/items/{{RID}}/analytics/itemActivityStats/{{id}}/activities/{{id2}}/driveItem/content",
        body: true,
        params: item_activity_stat_id, item_activity_id
    );
    get!(
        doc: "Get lastSevenDays from drives",
        name: get_last_seven_days,
        path: "/drive/items/{{RID}}/analytics/lastSevenDays"
    );
    post!(
        doc: "Invoke action checkin",
        name: checkin,
        path: "/drive/items/{{RID}}/checkin",
        body: true
    );
    post!(
        doc: "Invoke action checkout",
        name: checkout,
        path: "/drive/items/{{RID}}/checkout"
    );
    post!(
        doc: "Create new navigation property to children for drives",
        name: create_children,
        path: "/drive/items/{{RID}}/children",
        body: true
    );
    get!(
        doc: "List children of a driveItem",
        name: list_children,
        path: "/drive/items/{{RID}}/children"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_children_count,
        path: "/drive/items/{{RID}}/children/$count"
    );
    get!(
        doc: "Get children from drives",
        name: get_children,
        path: "/drive/items/{{RID}}/children/{{id}}",
        params: drive_item_id_1
    );
    get!(
        doc: "Get content for the navigation property children from drives",
        name: get_children_content,
        path: "/drive/items/{{RID}}/children/{{id}}/content",
        params: drive_item_id_1
    );
    put!(
        doc: "Update content for the navigation property children in drives",
        name: update_children_content,
        path: "/drive/items/{{RID}}/children/{{id}}/content",
        body: true,
        params: drive_item_id_1
    );
    get!(
        doc: "Get content for the navigation property items from drives",
        name: get_items_content,
        path: "/drive/items/{{RID}}/content"
    );
    put!(
        doc: "Update content for the navigation property items in drives",
        name: update_items_content,
        path: "/drive/items/{{RID}}/content",
        body: true
    );
    post!(
        doc: "Invoke action copy",
        name: copy,
        path: "/drive/items/{{RID}}/copy",
        body: true
    );
    post!(
        doc: "Invoke action createLink",
        name: create_link,
        path: "/drive/items/{{RID}}/createLink",
        body: true
    );
    post!(
        doc: "Invoke action createUploadSession",
        name: create_upload_session,
        path: "/drive/items/{{RID}}/createUploadSession",
        body: true
    );
    get!(
        doc: "Invoke function delta",
        name: get_drive_item_delta,
        path: "/drive/items/{{RID}}/delta()"
    );
    get!(
        doc: "Invoke function delta",
        name: get_drive_item_delta_token,
        path: "/drive/items/{{RID}}/delta(token='{{id}}')",
        params: token
    );
    post!(
        doc: "Invoke action follow",
        name: follow,
        path: "/drive/items/{{RID}}/follow"
    );
    get!(
        doc: "Invoke function getActivitiesByInterval",
        name: get_drive_item_activities_by_interval,
        path: "/drive/items/{{RID}}/getActivitiesByInterval(startDateTime='{{id}}',endDateTime='{{id2}}',interval='{{id3}}')",
        params: start_date_time, end_date_time, interval
    );
    post!(
        doc: "Invoke action invite",
        name: invite,
        path: "/drive/items/{{RID}}/invite",
        body: true
    );
    get!(
        doc: "Get listItem from drives",
        name: get_list_item,
        path: "/drive/items/{{RID}}/listItem"
    );
    post!(
        doc: "Create new navigation property to permissions for drives",
        name: create_permissions,
        path: "/drive/items/{{RID}}/permissions",
        body: true
    );
    get!(
        doc: "List sharing permissions on a driveItem",
        name: list_permissions,
        path: "/drive/items/{{RID}}/permissions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_permissions_count,
        path: "/drive/items/{{RID}}/permissions/$count"
    );
    delete!(
        doc: "Delete navigation property permissions for drives",
        name: delete_permissions,
        path: "/drive/items/{{RID}}/permissions/{{id}}",
        params: permission_id
    );
    get!(
        doc: "Get permissions from drives",
        name: get_permissions,
        path: "/drive/items/{{RID}}/permissions/{{id}}",
        params: permission_id
    );
    patch!(
        doc: "Update the navigation property permissions in drives",
        name: update_permissions,
        path: "/drive/items/{{RID}}/permissions/{{id}}",
        body: true,
        params: permission_id
    );
    post!(
        doc: "Invoke action grant",
        name: grant,
        path: "/drive/items/{{RID}}/permissions/{{id}}/grant",
        body: true,
        params: permission_id
    );
    post!(
        doc: "Invoke action preview",
        name: preview,
        path: "/drive/items/{{RID}}/preview",
        body: true
    );
    post!(
        doc: "Invoke action restore",
        name: restore,
        path: "/drive/items/{{RID}}/restore",
        body: true
    );
    get!(
        doc: "Invoke function search",
        name: search,
        path: "/drive/items/{{RID}}/search(q='{{id}}')",
        params: q
    );
    post!(
        doc: "Create new navigation property to subscriptions for drives",
        name: create_subscriptions,
        path: "/drive/items/{{RID}}/subscriptions",
        body: true
    );
    get!(
        doc: "Get subscriptions from drives",
        name: list_subscriptions,
        path: "/drive/items/{{RID}}/subscriptions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_subscriptions_count,
        path: "/drive/items/{{RID}}/subscriptions/$count"
    );
    delete!(
        doc: "Delete navigation property subscriptions for drives",
        name: delete_subscriptions,
        path: "/drive/items/{{RID}}/subscriptions/{{id}}",
        params: subscription_id
    );
    get!(
        doc: "Get subscriptions from drives",
        name: get_subscriptions,
        path: "/drive/items/{{RID}}/subscriptions/{{id}}",
        params: subscription_id
    );
    patch!(
        doc: "Update the navigation property subscriptions in drives",
        name: update_subscriptions,
        path: "/drive/items/{{RID}}/subscriptions/{{id}}",
        body: true,
        params: subscription_id
    );
    post!(
        doc: "Invoke action reauthorize",
        name: reauthorize,
        path: "/drive/items/{{RID}}/subscriptions/{{id}}/reauthorize",
        params: subscription_id
    );
    post!(
        doc: "Create new navigation property to thumbnails for drives",
        name: create_thumbnails,
        path: "/drive/items/{{RID}}/thumbnails",
        body: true
    );
    get!(
        doc: "List thumbnails for a DriveItem",
        name: list_thumbnails,
        path: "/drive/items/{{RID}}/thumbnails"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_thumbnails_count,
        path: "/drive/items/{{RID}}/thumbnails/$count"
    );
    delete!(
        doc: "Delete navigation property thumbnails for drives",
        name: delete_thumbnails,
        path: "/drive/items/{{RID}}/thumbnails/{{id}}",
        params: thumbnail_set_id
    );
    get!(
        doc: "Get thumbnails from drives",
        name: get_thumbnails,
        path: "/drive/items/{{RID}}/thumbnails/{{id}}",
        params: thumbnail_set_id
    );
    patch!(
        doc: "Update the navigation property thumbnails in drives",
        name: update_thumbnails,
        path: "/drive/items/{{RID}}/thumbnails/{{id}}",
        body: true,
        params: thumbnail_set_id
    );
    post!(
        doc: "Invoke action unfollow",
        name: unfollow,
        path: "/drive/items/{{RID}}/unfollow"
    );
    post!(
        doc: "Invoke action validatePermission",
        name: validate_permission,
        path: "/drive/items/{{RID}}/validatePermission",
        body: true
    );
    post!(
        doc: "Create new navigation property to versions for drives",
        name: create_versions,
        path: "/drive/items/{{RID}}/versions",
        body: true
    );
    get!(
        doc: "List versions of a driveItem",
        name: list_versions,
        path: "/drive/items/{{RID}}/versions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_versions_count,
        path: "/drive/items/{{RID}}/versions/$count"
    );
    delete!(
        doc: "Delete navigation property versions for drives",
        name: delete_versions,
        path: "/drive/items/{{RID}}/versions/{{id}}",
        params: drive_item_version_id
    );
    get!(
        doc: "Get versions from drives",
        name: get_versions,
        path: "/drive/items/{{RID}}/versions/{{id}}",
        params: drive_item_version_id
    );
    patch!(
        doc: "Update the navigation property versions in drives",
        name: update_versions,
        path: "/drive/items/{{RID}}/versions/{{id}}",
        body: true,
        params: drive_item_version_id
    );
    get!(
        doc: "Get content for the navigation property versions from drives",
        name: get_versions_content,
        path: "/drive/items/{{RID}}/versions/{{id}}/content",
        params: drive_item_version_id
    );
    put!(
        doc: "Update content for the navigation property versions in drives",
        name: update_versions_content,
        path: "/drive/items/{{RID}}/versions/{{id}}/content",
        body: true,
        params: drive_item_version_id
    );
    post!(
        doc: "Invoke action restoreVersion",
        name: restore_version,
        path: "/drive/items/{{RID}}/versions/{{id}}/restoreVersion",
        params: drive_item_version_id
    );
}

 */

/*
// Methods that require explicit implementation.
impl DriveApiClient

{
    pub(crate) fn transfer_identity(&self) {
        let ident = self.client.ident();
        if ident.eq(&ResourceIdentity::Users)
            || ident.eq(&ResourceIdentity::Me)
            || ident.eq(&ResourceIdentity::Sites)
            || ident.eq(&ResourceIdentity::Groups)
        {
            self.client
                .request
                .extend_path(&[ResourceIdentity::Drive.as_ref()]);
        }
    }

    pub fn check_out_item<S: AsRef<str>>(&'a self, id: S) -> IntoResponse<'a, NoContent, Client> {
        render_path!(
            self.client,
            template(id.as_ref(), "checkout").as_str(),
            &json!({ "id": id.as_ref() })
        );
        let client = self.client.request();
        client.set_method(Method::POST);
        client.header(CONTENT_LENGTH, HeaderValue::from(0));
        IntoResponse::new(&self.client.request)
    }

    pub fn preview<S: AsRef<str>, B: serde::Serialize>(
        &'a self,
        id: S,
        body: Option<&B>,
    ) -> IntoResponse<'a, serde_json::Value, Client> {
        if let Some(body) = body {
            let body = serde_json::to_string(body);
            if let Ok(body) = body {
                let client = self.client.request();
                client.set_method(Method::POST);
                client.set_body(body);
            } else if let Err(e) = body {
                return IntoResponse::new_error(self.client.request(), GraphFailure::from(e));
            }
        } else {
            let client = self.client.request();
            client.set_method(Method::POST);
            client.header(CONTENT_LENGTH, HeaderValue::from(0));
        }
        render_path!(
            self.client,
            template(id.as_ref(), "preview").as_str(),
            &json!({ "id": id.as_ref() })
        );
        IntoResponse::new(&self.client.request)
    }

    pub fn upload_new<S: AsRef<str>, P: AsRef<Path>>(
        &'a self,
        id: S,
        file: P,
    ) -> IntoResponse<'a, serde_json::Value, Client> {
        if id.as_ref().starts_with(':') {
            if let Err(err) = self
                .client
                .request()
                .set_body_with_file(file.as_ref().to_path_buf())
            {
                return IntoResponse::new_error(self.client.request(), err);
            }

            self.client.request().set_method(Method::PUT);
            render_path!(
                self.client,
                template(id.as_ref(), "content").as_str(),
                &json!({"id": id.as_ref() })
            );
        } else {
            let name = file.as_ref().file_name();
            if name.is_none() {
                return IntoResponse::new_error(
                    self.client.request(),
                    GraphFailure::invalid("file_name"),
                );
            }
            let name = name.unwrap().to_str();
            if name.is_none() {
                return IntoResponse::new_error(
                    self.client.request(),
                    GraphFailure::internal(GraphRsError::FileNameInvalidUTF8),
                );
            }
            render_path!(
                self.client,
                "{{drive_item}}/{{id}}:/{{file_name}}:/content",
                &json!({
                    "id": id.as_ref(),
                    "file_name": name.unwrap(),
                })
            );

            if let Err(e) = self
                .client
                .request()
                .set_body_with_file(file.as_ref().to_path_buf())
            {
                return IntoResponse::new_error(self.client.request(), e);
            }
            self.client.request().set_method(Method::PUT);
        }
        IntoResponse::new(&self.client.request)
    }

    pub fn create_upload_session<
        S: AsRef<str>,
        P: AsRef<Path> + Send + Sync,
        B: serde::Serialize,
    >(
        &'a self,
        id: S,
        file: P,
        body: &B,
    ) -> IntoResponse<'a, UploadSessionClient<Client>, Client> {
        let body = serde_json::to_string(body);
        if let Ok(body) = body {
            let client = self.client.request();
            client.set_method(Method::POST);
            client.set_upload_session(file.as_ref().to_path_buf());
            client.set_body(body);
        } else if let Err(e) = body {
            return IntoResponse::new_error(self.client.request(), GraphFailure::from(e));
        }
        render_path!(
            self.client,
            template(id.as_ref(), "createUploadSession").as_str(),
            &json!({ "id": id.as_ref() })
        );
        IntoResponse::new(&self.client.request)
    }
}

impl<'a> DrivesRequest<'a, BlockingHttpClient> {
    pub fn download<S: AsRef<str>, P: AsRef<Path>>(
        &'a self,
        id: S,
        directory: P,
    ) -> BlockingDownload {
        render_path!(
            self.client,
            template(id.as_ref(), "content").as_str(),
            &json!({ "id": id.as_ref() })
        );
        self.client
            .request()
            .set_request(vec![
                RequestAttribute::Method(Method::GET),
                RequestAttribute::Download(directory.as_ref().to_path_buf()),
                RequestAttribute::RequestType(RequestType::Redirect),
            ])
            .unwrap();
        self.client.request().download()
    }
}

impl<'a> DrivesRequest<'a, AsyncHttpClient> {
    pub fn download<S: AsRef<str>, P: AsRef<Path>>(&'a self, id: S, directory: P) -> AsyncDownload {
        render_path!(
            self.client,
            template(id.as_ref(), "content").as_str(),
            &json!({ "id": id.as_ref() })
        );
        self.client
            .request()
            .set_request(vec![
                RequestAttribute::Method(Method::GET),
                RequestAttribute::Download(directory.as_ref().to_path_buf()),
                RequestAttribute::RequestType(RequestType::Redirect),
            ])
            .unwrap();
        futures::executor::block_on(self.client.request().download())
    }
}

 */
