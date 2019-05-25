use crate::drive;
use crate::drive::drive_item::driveitem::DriveItem;
use crate::drive::driveaction::DownloadFormat;
use crate::drive::{DriveEvent, DriveResource, DriveVersion, ItemResult, ResourceBuilder};
use crate::fetch::Fetch;
use graph_error::GraphError;
use reqwest::{header, Client, RedirectPolicy, RequestBuilder, Response};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::{Path, PathBuf};
use transform_request::RequestError;

#[derive(Debug)]
pub struct ItemResponse {
    response: Response,
}

impl ItemResponse {
    pub fn new(response: Response) -> ItemResponse {
        ItemResponse { response }
    }

    pub fn status(&self) -> u16 {
        self.response.status().as_u16()
    }

    pub fn success(&self) -> bool {
        self.status() == 204
    }

    pub fn response(&self) -> &Response {
        &self.response
    }

    pub fn error(&self) -> Option<GraphError> {
        if GraphError::is_error(self.status()) {
            return Some(GraphError::try_from(self.status()).unwrap_or_default());
        }
        None
    }
}

pub trait Item {
    /// The token() method should return a bearer token to make
    /// authenticated calls to the OneDrive API.
    fn token(&self) -> &str;

    fn drive_version(&self) -> DriveVersion;

    fn item<T>(&self, r: &mut Response) -> ItemResult<T>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        if GraphError::is_error(r.status().as_u16()) {
            return Err(RequestError::from(
                GraphError::try_from(r).unwrap_or_default(),
            ));
        }
        let item: T = r.json()?;
        Ok(item)
    }

    /// Converts response to a drive item. Checks for errors
    /// for errors returned from the OneDrive API and if found
    /// returns the error.
    fn drive_item(&self, r: &mut Response) -> ItemResult<DriveItem> {
        if GraphError::is_error(r.status().as_u16()) {
            return Err(RequestError::from(
                GraphError::try_from(r).unwrap_or_default(),
            ));
        }
        let drive_item = DriveItem::try_from(r)?;
        Ok(drive_item)
    }

    /// Converts serde_json Value to a drive item. Checks for errors
    /// for errors returned from the OneDrive API and if found
    /// returns the error.
    fn value(r: &mut Response) -> ItemResult<serde_json::Value> {
        if GraphError::is_error(r.status().as_u16()) {
            return Err(RequestError::from(
                GraphError::try_from(r).unwrap_or_default(),
            ));
        }
        let v: serde_json::Value = r.json()?;
        Ok(v)
    }

    fn client(&self) -> Result<Client, RequestError> {
        reqwest::Client::builder()
            .build()
            .map_err(RequestError::from)
    }

    /// A request builder for REST requests with the authorization header
    /// included.
    fn request_builder(&self, url: &str) -> ItemResult<RequestBuilder> {
        Ok(self.client()?.get(url).bearer_auth(self.token()))
    }

    fn get<T>(&self, url: &str) -> ItemResult<T>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        let mut response = self
            .client()?
            .get(url)
            .bearer_auth(self.token())
            .header(header::CONTENT_TYPE, "application/json")
            .send()?;

        self.item(&mut response)
    }

    /// Check-in a checkout DriveItem resource, which makes the version
    /// of the document available to others.
    ///
    /// # See
    /// [Check-in](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_checkin?view=odsp-graph-online)
    fn check_in(
        &self,
        drive_id: &str,
        item_id: &str,
        comment: &str,
        check_in_as: Option<&str>,
        resource: DriveResource,
    ) -> ItemResult<ItemResponse> {
        let mut builder = ResourceBuilder::new(self.drive_version());
        builder
            .drive_event(DriveEvent::CheckIn)
            .item_id(item_id)
            .drive_id(drive_id)
            .resource(resource);
        let url = builder.build()?;
        let mut map = HashMap::new();
        map.insert("comment", comment);
        if let Some(check_in) = check_in_as {
            map.insert("checkInAs", check_in);
        }

        let response = self
            .client()?
            .post(url.as_str())
            .bearer_auth(self.token())
            .json(&map)
            .header(header::CONTENT_TYPE, "application/json")
            .send()?;

        Ok(ItemResponse::new(response))
    }

    /// Check-out a driveItem resource to prevent others from editing the document,
    /// and your changes from being visible until the documented is checked-in.
    ///
    /// # See
    /// [Check-out](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_checkout?view=odsp-graph-online)
    fn check_out(
        &self,
        drive_id: &str,
        item_id: &str,
        resource: DriveResource,
    ) -> ItemResult<ItemResponse> {
        let mut builder = ResourceBuilder::new(self.drive_version());
        builder
            .drive_event(DriveEvent::CheckOut)
            .item_id(item_id)
            .drive_id(drive_id)
            .resource(resource);
        let url = builder.build()?;
        let response = self
            .client()?
            .post(url.as_str())
            .bearer_auth(self.token())
            .send()?;

        Ok(ItemResponse::new(response))
    }

    /// Delete a DriveItem by using its ID. Note that deleting items using this
    /// method will move the items to the recycle bin instead of permanently deleting the item.
    ///
    /// # Example
    /// ```rust,ignore
    /// use rust_onedrive::drive::{Drive, DriveVersion, ItemResponse, DriveResource};
    /// use rust_onedrive::drive::driveitem::DriveItem;
    /// use rust_onedrive::drive::value::Value;
    /// use rust_onedrive::drive::driveinfo::DriveInfo;
    ///
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    ///
    /// // Call the API. drive_root_child is the files in the users main documents folder.
    /// let mut drive_item: DriveItem = drive.drive_root_child().unwrap();
    ///
    /// // Find the file based on it's name.
    /// let value = drive_item.find_by_name("YOUR FILE NAME WITH EXTENSION SUCH AS: my_file.txt").unwrap();
    ///
    /// // Get the drive id from the DriveInfo metadata.
    /// let drive_info: DriveInfo = drive.drive().unwrap();
    /// let drive_id = drive_info.id().unwrap();
    ///
    /// // Get the item id from the files value metadata.
    /// let item_id = value.id().unwrap();
    ///
    /// // Delete the item based on the type of drive using DriveResource.
    /// // This can be one of:
    /// //    DriveResource::Drives,
    /// //    DriveResource::Groups,
    /// //    DriveResource::Sites,
    /// //    DriveResource::Users,
    /// //    DriveResource::Me,
    /// //
    /// // The DriveResource changes the URL being used to make the request.
    /// // For instance, given an item id of 1234 and a drive id of 1, the URL for
    /// // drives and users would look like:
    /// //
    /// // DriveResource::Drives => "https://graph.microsoft.com/v1.0/drives/1/items/1234/"
    /// // DriveResource::Users => https://graph.microsoft.com/v1.0/users/1/drive/items/1234/
    /// //
    /// // Note: DriveResource::Me does not use the drive_id, so while this is an option
    /// // it may be better to use the delete_by_value method which will make sure to use
    /// // the correct drive id. However, this method, always uses the DriveResource::Drives
    /// // URL. This may change in the future.
    /// let response: ItemResponse = drive
    ///      .delete(drive_id.as_str(), item_id.as_str(), DriveResource::Drives)
    ///      .unwrap();
    /// println!("{:#?}", response);
    /// println!("\nItem was deleted: {:#?}", response.success());
    /// ```
    ///
    /// # See
    /// [Delete a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_delete?view=odsp-graph-online)
    fn delete(
        &self,
        drive_id: &str,
        item_id: &str,
        resource: DriveResource,
    ) -> ItemResult<ItemResponse> {
        let mut builder = ResourceBuilder::new(self.drive_version());
        builder
            .drive_event(DriveEvent::Delete)
            .item_id(item_id)
            .drive_id(drive_id)
            .resource(resource);
        let url = builder.build()?;
        let response = self
            .client()?
            .delete(url.as_str())
            .bearer_auth(self.token())
            .send()?;

        Ok(ItemResponse::new(response))
    }

    /// Delete a DriveItem by the item's Value. This method will use the Me endpoint to make
    /// the request. Note that deleting items using this method will move the items to the
    /// recycle bin instead of permanently deleting the item.
    ///
    /// # Example
    /// ```rust,ignore
    /// use rust_onedrive::drive::{Drive, DriveVersion, ItemResponse};
    /// use rust_onedrive::drive::driveitem::DriveItem;
    /// use rust_onedrive::drive::value::Value;
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    ///
    /// let mut drive_item: DriveItem = drive.drive_recent().unwrap();
    /// let value: Value = drive_item.find_by_name("YOUR FILE NAME WITH EXTENSION SUCH AS: my_file.txt").unwrap();
    /// let item_response: ItemResponse = drive.delete_by_value(value).unwrap();
    /// println!("Item was deleted: {:#?}", item_response.success());
    /// ```
    ///
    /// # See
    /// [Delete a DriveItem](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_delete?view=odsp-graph-online)
    fn delete_by_value(&self, value: drive::value::Value) -> ItemResult<ItemResponse> {
        let url = DriveResource::Drives.drive_item_resource(
            self.drive_version(),
            value
                .parent_reference()
                .ok_or_else(|| RequestError::none_err("value parent_reference"))?
                .drive_id()
                .ok_or_else(|| RequestError::none_err("value parent_reference drive_id"))?
                .as_str(),
            value
                .id()
                .ok_or_else(|| RequestError::none_err("value item_id"))?
                .as_str(),
            DriveEvent::Delete,
        );
        let response = self
            .client()?
            .delete(url.as_str())
            .bearer_auth(self.token())
            .send()?;

        Ok(ItemResponse::new(response))
    }

    /// Download files from the OneDrive API.
    ///
    /// # Example
    /// ```rust,ignore
    /// use rust_onedrive::prelude::*;
    /// use std::path::PathBuf;
    ///
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// let mut drive_item: DriveItem = drive.drive_root_child().unwrap();
    /// let vec: Vec<Value> = drive_item.value().unwrap();
    ///
    /// let mut value = drive_item.find_by_name("rust.docx").unwrap();
    /// let path_buf: PathBuf = drive.download("/home/drive", &mut value).unwrap();
    /// println!("{:#?}", path_buf);
    /// ```
    ///
    /// Requires the directory to download to and the drive::Value to download. The Value
    /// struct stores a download URL that can be used. If the download url is None, then
    /// the item's id (also in the Value struct) is used to download the item.
    ///
    /// # See
    /// [Downloading Drive Items](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_get_content?view=odsp-graph-online)
    fn download<P: AsRef<Path>>(
        &self,
        directory: P,
        value: &mut drive::value::Value,
    ) -> ItemResult<PathBuf> {
        match value.microsoft_graph_download_url() {
            // First check for a download URL in the drive::Value itself, If found use this
            // to download the file.
            Some(download_url) => Ok(Fetch::file(directory, download_url.as_str(), self.token())?),
            // If there is no download URL, then a request to get a download URL
            // will be made. If successful, the request will be redirected to the URL
            // of the download.
            None => {
                let client = reqwest::Client::builder()
                    .redirect(RedirectPolicy::custom(|attempt| {
                        // There should be only 1 redirect to download a drive item.
                        if attempt.previous().len() > 1 {
                            return attempt.too_many_redirects();
                        }
                        attempt.stop()
                    }))
                    .build()
                    .map_err(RequestError::from)?;

                // In order for the OneDrive API to know which item we need, the request
                // must include the id for the item being downloaded: /{item-id}/content
                let item_id = match value.id() {
                    Some(t) => t,
                    None => return Err(RequestError::none_err("Missing item id or download URL")),
                };

                let url = DriveResource::Me.item_resource(
                    self.drive_version(),
                    item_id.as_str(),
                    DriveEvent::Download,
                );
                let res = client.get(url.as_str()).bearer_auth(self.token()).send()?;
                Ok(Fetch::file(directory, res.url().as_str(), self.token())?)
            },
        }
    }

    /// Download files from the OneDrive API in the format given. The format given
    /// must be one of:
    /// # Example
    /// ```rust,ignore
    /// DownloadFormat::GLB => "glb",
    /// DownloadFormat::HTML => "html",
    /// DownloadFormat::JPG => "jpg",
    /// DownloadFormat::PDF => "pdf",
    /// ```
    ///
    /// # Example
    /// ```rust,ignore
    /// use rust_onedrive::prelude::*;
    /// use std::path::PathBuf;
    ///
    /// let mut drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
    /// let mut drive_item: DriveItem = drive.drive_root_child().unwrap();
    /// let vec: Vec<Value> = drive_item.value().unwrap();
    ///
    /// let mut value = drive_item.find_by_name("rust.docx").unwrap();
    ///
    /// let path_buf: PathBuf = drive.download_format("/home/drive", &mut value, DownloadFormat::PDF).unwrap();
    /// println!("{:#?}", path_buf);
    /// ```
    ///
    /// Requires the directory to download, the drive::Value, and the format. The Value
    /// struct stores a download URL that can be used. If the download url is None, then
    /// the item's id (also in the Value struct) is used to download the item.
    ///
    /// # See
    /// [Download Formats](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_get_content_format?view=odsp-graph-online)
    fn download_format<P: AsRef<Path>>(
        &self,
        directory: P,
        value: &mut drive::value::Value,
        format: DownloadFormat,
    ) -> ItemResult<PathBuf> {
        // A formatted download always uses a redirect to get the item.
        let client = reqwest::Client::builder()
            .redirect(RedirectPolicy::custom(|attempt| {
                // There should be only 1 redirect to download a drive item.
                if attempt.previous().len() > 1 {
                    return attempt.too_many_redirects();
                }
                attempt.stop()
            }))
            .build()
            .map_err(RequestError::from)?;

        // In order for the OneDrive API to know which item we need, the request
        // must include the id for the item being downloaded: /{item-id}/content
        let item_id = match value.id() {
            Some(t) => t,
            None => return Err(RequestError::none_err("Missing item id or download URL")),
        };

        let mut url = DriveResource::Drives.item_resource(
            self.drive_version(),
            item_id.as_str(),
            DriveEvent::DownloadAndFormat,
        );
        url.push_str(format.as_ref());
        let res = client.get(url.as_str()).bearer_auth(self.token()).send()?;
        Ok(Fetch::file(directory, res.url().as_str(), self.token())?)
    }
}
