use graph_rs::client::{Graph, Ident};
use graph_rs::http::RequestClient;
use graph_rs::url::GraphUrl;
use graph_rs::{GRAPH_URL, GRAPH_URL_BETA};
use url::Url;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SpecialFolder {
    Drive,
    DriveRoot,
    DriveRootChild,
    Delta,
    SharedWithMe,
    DriveRecent,
    DriveActivities,
    SpecialFolder,
    SpecialDocuments,
    SpecialDocumentsChild,
    SpecialPhotos,
    SpecialPhotosChild,
    SpecialCameraRoll,
    SpecialCameraRollChild,
    SpecialAppRoot,
    SpecialAppRootChild,
    SpecialMusic,
    SpecialMusicChild,
}

impl SpecialFolder {
    pub fn as_str(self) -> &'static str {
        match self {
            SpecialFolder::Drive => "drive",
            SpecialFolder::DriveRoot => "root",
            SpecialFolder::DriveRootChild => "root/children",
            SpecialFolder::Delta => "root/delta",
            SpecialFolder::SharedWithMe => "sharedWithMe",
            SpecialFolder::DriveRecent => "recent",
            SpecialFolder::DriveActivities => "activities",
            SpecialFolder::SpecialFolder => "special",
            SpecialFolder::SpecialDocuments => "special/documents",
            SpecialFolder::SpecialDocumentsChild => "special/documents/children",
            SpecialFolder::SpecialPhotos => "special/photos",
            SpecialFolder::SpecialPhotosChild => "special/photos/children",
            SpecialFolder::SpecialCameraRoll => "special/cameraroll",
            SpecialFolder::SpecialCameraRollChild => "special/cameraroll/children",
            SpecialFolder::SpecialAppRoot => "special/approot",
            SpecialFolder::SpecialAppRootChild => "special/approot/children",
            SpecialFolder::SpecialMusic => "special/music",
            SpecialFolder::SpecialMusicChild => "special/music/children",
        }
    }

    pub fn v1_url(self) -> String {
        format!("{}/me/{}", GRAPH_URL, self.as_str())
    }

    pub fn v1_url_with_id(self, id: &str) -> String {
        format!("{}/{}/{}", GRAPH_URL, id, self.as_str())
    }

    pub fn beta_url(self) -> String {
        format!("{}/me/{}", GRAPH_URL_BETA, self.as_str())
    }

    pub fn beta_url_with_id(self, id: &str) -> String {
        format!("{}/{}/{}", GRAPH_URL_BETA, id, self.as_str())
    }

    pub fn url(self, host: &str) -> String {
        format!("{}/{}", host, self.as_str())
    }

    pub fn with_ident(self, host: &str, ident: Ident) -> String {
        format!("{}/{}/{}", host, ident.as_ref(), self.as_str())
    }
}

impl From<SpecialFolder> for String {
    fn from(dep: SpecialFolder) -> Self {
        dep.v1_url()
    }
}

impl From<SpecialFolder> for GraphUrl {
    fn from(endpoint: SpecialFolder) -> Self {
        let url = format!("{}/{}", GRAPH_URL, endpoint.as_str());
        GraphUrl::from(Url::parse(&url).unwrap())
    }
}

impl AsRef<str> for SpecialFolder {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl ToString for SpecialFolder {
    fn to_string(&self) -> String {
        String::from(self.as_ref())
    }
}

pub fn assert_url_special<Client: RequestClient>(client: &Graph<Client>, endpoint: SpecialFolder) {
    client.url_ref(|url| {
        if client.ident().eq(&Ident::Me) {
            assert_eq!(
                format!("{}/me/drive/{}", GRAPH_URL, endpoint.as_str()),
                url.to_string(),
            )
        } else if client.ident().eq(&Ident::Drives) {
            assert_eq!(
                format!("{}/{}", GRAPH_URL, endpoint.as_str()),
                url.to_string()
            )
        } else {
            assert_eq!(
                format!("{}/drive/{}", GRAPH_URL, endpoint.as_str()),
                url.to_string()
            )
        }
    })
}

pub fn assert_url_id_equals<Client: RequestClient>(
    client: &Graph<Client>,
    item_id: &str,
    endpoint: SpecialFolder,
) {
    client.url_ref(|url| {
        if client.ident().eq(&Ident::Me) {
            assert_eq!(
                format!("{}/me/drive/{}", GRAPH_URL, endpoint.as_str()),
                url.to_string(),
            )
        } else if client.ident().eq(&Ident::Drives) {
            assert_eq!(
                format!(
                    "{}/{}/{}/{}",
                    GRAPH_URL,
                    client.ident().as_ref(),
                    item_id,
                    endpoint.as_str()
                ),
                url.to_string()
            )
        } else {
            assert_eq!(
                format!(
                    "{}/{}/{}/drive/{}",
                    GRAPH_URL,
                    client.ident().as_ref(),
                    item_id,
                    endpoint.as_str()
                ),
                url.to_string()
            )
        }
    })
}

pub fn assert_url_special_beta<Client: RequestClient>(
    client: &Graph<Client>,
    endpoint: SpecialFolder,
) {
    client.url_ref(|url| {
        if client.ident().eq(&Ident::Me) {
            if endpoint.eq(&SpecialFolder::Drive) {
                assert_eq!(
                    format!("{}/me/{}", GRAPH_URL_BETA, endpoint.as_str()),
                    url.to_string(),
                )
            } else {
                assert_eq!(
                    format!("{}/me/drive/{}", GRAPH_URL_BETA, endpoint.as_str()),
                    url.to_string(),
                )
            }
        } else if endpoint.eq(&SpecialFolder::Drive) {
            assert_eq!(
                format!("{}/{}", GRAPH_URL_BETA, endpoint.as_str()),
                url.to_string()
            )
        } else {
            assert_eq!(
                format!("{}/drive/{}", GRAPH_URL_BETA, endpoint.as_str()),
                url.to_string()
            )
        }
    })
}

pub fn assert_url_id_equals_beta<Client: RequestClient>(
    client: &Graph<Client>,
    item_id: &str,
    endpoint: SpecialFolder,
) {
    client.url_ref(|url| {
        if client.ident().eq(&Ident::Me) {
            assert_eq!(
                format!("{}/me/drive/{}", GRAPH_URL_BETA, endpoint.as_str()),
                url.to_string(),
            )
        } else if client.ident().eq(&Ident::Drives) {
            assert_eq!(
                format!(
                    "{}/{}/{}/{}",
                    GRAPH_URL_BETA,
                    client.ident().as_ref(),
                    item_id,
                    endpoint.as_str()
                ),
                url.to_string()
            )
        } else {
            assert_eq!(
                format!(
                    "{}/{}/{}/drive/{}",
                    GRAPH_URL_BETA,
                    client.ident().as_ref(),
                    item_id,
                    endpoint.as_str()
                ),
                url.to_string()
            )
        }
    })
}
