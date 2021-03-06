#![allow(unused_variables, unused_mut, dead_code)]
//! This file was generated by async-google-apis. (https://github.com/dermesser/async-google-apis)
//!
//! (c) 2020 Lewin Bormann <lbo@spheniscida.de>
//!
//! ## Getting started
//!
//! **Tip**: Take a look at those types ending in `...Service`. These represent API resources
//! and contain methods to interact with an API. The remaining types are used by those methods
//! and can be explored starting from a method you want to use.
//!
//! I'd be happy if you let me know about your use case of this code.
//!
//! THIS FILE HAS BEEN GENERATED -- SAVE ANY MODIFICATIONS BEFORE REPLACING.

use async_google_apis_common::*;

/// Scopes of this API. Convertible to their string representation with `AsRef`.
#[derive(Debug, Clone, Copy)]
pub enum IntegrationTestScopes {
    /// See, edit, create, and delete all of your Google integrationtest files
    ///
    /// URL: https://example.borgac.net/auth/integrationtest
    Integrationtest,
    /// View and manage metadata of files in your Google integrationtest
    ///
    /// URL: https://example.borgac.net/auth/integrationtest.metadata
    IntegrationtestMetadata,
}

impl std::convert::AsRef<str> for IntegrationTestScopes {
    fn as_ref(&self) -> &'static str {
        match self {
            IntegrationTestScopes::Integrationtest => {
                "https://example.borgac.net/auth/integrationtest"
            }
            IntegrationTestScopes::IntegrationtestMetadata => {
                "https://example.borgac.net/auth/integrationtest.metadata"
            }
        }
    }
}

/// Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FileCapabilities {
    /// Whether the current user can add children to this folder. This is always false when the item is not a folder.
    #[serde(rename = "canAddChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_children: Option<bool>,
    /// Whether the current user can comment on this file.
    #[serde(rename = "canComment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_comment: Option<bool>,
}

/// The metadata for a file.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct File {
    /// A collection of arbitrary key-value pairs which are private to the requesting app. Entries with null values are cleared in update and copy requests.
    #[serde(rename = "appProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_properties: Option<HashMap<String, String>>,
    /// Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take.
    #[serde(rename = "capabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<FileCapabilities>,
}

/// A list of files.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FileList {
    /// The list of files. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched.
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<File>>,
    /// Whether the search process was incomplete. If true, then some search results may be missing, since all documents were not searched. This may occur when searching multiple drives with the "allDrives" corpora, but all corpora could not be searched. When this happens, it is suggested that clients narrow their query by choosing a different corpus such as "user" or "drive".
    #[serde(rename = "incompleteSearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_search: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "drive#fileList".
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The page token for the next page of files. This will be absent if the end of the files list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

///
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct IntegrationTestParams {
    /// Data format for the response.
    #[serde(rename = "alt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    /// Selector specifying which fields to include in a partial response.
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    /// API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// OAuth 2.0 token for the current user.
    #[serde(rename = "oauth_token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_token: Option<String>,
    /// Returns response with indentations and line breaks.
    #[serde(rename = "prettyPrint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pretty_print: Option<bool>,
    /// An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    #[serde(rename = "quotaUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_user: Option<String>,
    /// Deprecated. Please use quotaUser instead.
    #[serde(rename = "userIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ip: Option<String>,
}

/// Parameters for the `files.copy` method.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FilesCopyParams {
    /// General attributes applying to any API call
    #[serde(flatten)]
    pub integration_test_params: Option<IntegrationTestParams>,
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
}

impl std::fmt::Display for FilesCopyParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

/// Parameters for the `files.create` method.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FilesCreateParams {
    /// General attributes applying to any API call
    #[serde(flatten)]
    pub integration_test_params: Option<IntegrationTestParams>,
    /// Whether to use the uploaded content as indexable text.
    #[serde(rename = "useContentAsIndexableText")]
    pub use_content_as_indexable_text: Option<bool>,
}

impl std::fmt::Display for FilesCreateParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref v) = self.use_content_as_indexable_text {
            write!(
                f,
                "&useContentAsIndexableText={}",
                percent_encode(format!("{}", v).as_bytes(), NON_ALPHANUMERIC).to_string()
            )?;
        }
        Ok(())
    }
}

/// Parameters for the `files.delete` method.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FilesDeleteParams {
    /// General attributes applying to any API call
    #[serde(flatten)]
    pub integration_test_params: Option<IntegrationTestParams>,
    /// Set to true to opt in to API behavior that aims for all items to have exactly one parent. This parameter will only take effect if the item is not in a shared drive. If an item's last parent is deleted but the item itself is not, the item will be placed under its owner's root.
    #[serde(rename = "enforceSingleParent")]
    pub enforce_single_parent: Option<bool>,
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// Whether the requesting application supports both My Drives and shared drives.
    #[serde(rename = "supportsAllDrives")]
    pub supports_all_drives: Option<bool>,
    /// Deprecated use supportsAllDrives instead.
    #[serde(rename = "supportsTeamDrives")]
    pub supports_team_drives: Option<bool>,
}

impl std::fmt::Display for FilesDeleteParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref v) = self.enforce_single_parent {
            write!(
                f,
                "&enforceSingleParent={}",
                percent_encode(format!("{}", v).as_bytes(), NON_ALPHANUMERIC).to_string()
            )?;
        }
        if let Some(ref v) = self.supports_all_drives {
            write!(
                f,
                "&supportsAllDrives={}",
                percent_encode(format!("{}", v).as_bytes(), NON_ALPHANUMERIC).to_string()
            )?;
        }
        if let Some(ref v) = self.supports_team_drives {
            write!(
                f,
                "&supportsTeamDrives={}",
                percent_encode(format!("{}", v).as_bytes(), NON_ALPHANUMERIC).to_string()
            )?;
        }
        Ok(())
    }
}

/// Parameters for the `files.emptyTrash` method.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FilesEmptyTrashParams {
    /// General attributes applying to any API call
    #[serde(flatten)]
    pub integration_test_params: Option<IntegrationTestParams>,
    /// Set to true to opt in to API behavior that aims for all items to have exactly one parent. This parameter will only take effect if the item is not in a shared drive. If an item's last parent is deleted but the item itself is not, the item will be placed under its owner's root.
    #[serde(rename = "enforceSingleParent")]
    pub enforce_single_parent: Option<bool>,
}

impl std::fmt::Display for FilesEmptyTrashParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref v) = self.enforce_single_parent {
            write!(
                f,
                "&enforceSingleParent={}",
                percent_encode(format!("{}", v).as_bytes(), NON_ALPHANUMERIC).to_string()
            )?;
        }
        Ok(())
    }
}

/// Parameters for the `files.export` method.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FilesExportParams {
    /// General attributes applying to any API call
    #[serde(flatten)]
    pub integration_test_params: Option<IntegrationTestParams>,
    /// The ID of the file.
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// The MIME type of the format requested for this export.
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}

impl std::fmt::Display for FilesExportParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "&mimeType={}",
            percent_encode(format!("{}", self.mime_type).as_bytes(), NON_ALPHANUMERIC).to_string()
        )?;
        Ok(())
    }
}

impl std::fmt::Display for IntegrationTestParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref v) = self.alt {
            write!(
                f,
                "&alt={}",
                percent_encode(format!("{}", v).as_bytes(), NON_ALPHANUMERIC).to_string()
            )?;
        }
        if let Some(ref v) = self.fields {
            write!(
                f,
                "&fields={}",
                percent_encode(format!("{}", v).as_bytes(), NON_ALPHANUMERIC).to_string()
            )?;
        }
        if let Some(ref v) = self.key {
            write!(
                f,
                "&key={}",
                percent_encode(format!("{}", v).as_bytes(), NON_ALPHANUMERIC).to_string()
            )?;
        }
        if let Some(ref v) = self.oauth_token {
            write!(
                f,
                "&oauth_token={}",
                percent_encode(format!("{}", v).as_bytes(), NON_ALPHANUMERIC).to_string()
            )?;
        }
        if let Some(ref v) = self.pretty_print {
            write!(
                f,
                "&prettyPrint={}",
                percent_encode(format!("{}", v).as_bytes(), NON_ALPHANUMERIC).to_string()
            )?;
        }
        if let Some(ref v) = self.quota_user {
            write!(
                f,
                "&quotaUser={}",
                percent_encode(format!("{}", v).as_bytes(), NON_ALPHANUMERIC).to_string()
            )?;
        }
        if let Some(ref v) = self.user_ip {
            write!(
                f,
                "&userIp={}",
                percent_encode(format!("{}", v).as_bytes(), NON_ALPHANUMERIC).to_string()
            )?;
        }
        Ok(())
    }
}

/// The Integration_test Files service represents the Files resource.
pub struct FilesService {
    client: TlsClient,
    authenticator: Box<dyn 'static + std::ops::Deref<Target = Authenticator>>,
    scopes: Vec<String>,

    base_url: String,
    root_url: String,
}

impl FilesService {
    /// Create a new FilesService object. The easiest way to call this is wrapping the Authenticator
    /// into an `Rc`: `new(client.clone(), Rc::new(authenticator))`.
    /// This way, one authenticator can be shared among several services.
    pub fn new<A: 'static + std::ops::Deref<Target = Authenticator>>(
        client: TlsClient,
        auth: A,
    ) -> FilesService {
        FilesService {
            client: client,
            authenticator: Box::new(auth),
            scopes: vec![],
            base_url: "https://example.borgac.net/integrationAPI".into(),
            root_url: "https://example.borgac.net/".into(),
        }
    }

    /// Provide the base URL of this API. The returned URL is guaranteed to end with a '/'.
    fn base_url(&self) -> String {
        if self.base_url.ends_with("/") {
            return self.base_url.clone();
        }
        return self.base_url.clone() + "/";
    }
    /// Provide the root URL of this API. The returned URL is guaranteed to end with a '/'.
    fn root_url(&self) -> String {
        if self.root_url.ends_with("/") {
            return self.root_url.clone();
        }
        return self.root_url.clone();
    }
    /// Returns appropriate URLs for relative and absolute paths.
    fn format_path(&self, path: &str) -> String {
        if path.starts_with("/") {
            return self.root_url().trim_end_matches("/").to_string() + path;
        } else {
            return self.base_url() + path;
        }
    }

    #[cfg(test)]
    /// Override API URLs. `base` is the base path relative to which (relative) method paths are interpreted,
    /// whereas `root` is the URL relative to which absolute paths are interpreted.
    pub fn set_urls(&mut self, base: String, root: String) {
        self.base_url = base;
        self.root_url = root;
    }

    /// Explicitly select which scopes should be requested for authorization. Otherwise,
    /// a possibly too large scope will be requested.
    ///
    /// It is most convenient to supply a vec or slice of Integration_testScopes enum values.
    pub fn set_scopes<S: AsRef<str>, T: AsRef<[S]>>(&mut self, scopes: T) {
        self.scopes = scopes
            .as_ref()
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect();
    }

    /// Creates a copy of a file and applies any requested updates with patch semantics. Folders cannot be copied.
    pub async fn copy(&mut self, params: &FilesCopyParams, req: &File) -> Result<File> {
        let rel_path = format!(
            "files/{fileId}/copy",
            fileId = percent_encode(params.file_id.as_bytes(), NON_ALPHANUMERIC)
        );
        let path = self.format_path(rel_path.as_str());
        println!("{}", path);

        let mut headers = vec![];
        let tok;
        if self.scopes.is_empty() {
            let scopes = &["https://www.googleapis.com/auth/drive.photos.readonly".to_string()];
            tok = self.authenticator.token(scopes).await?;
        } else {
            tok = self.authenticator.token(&self.scopes).await?;
        }
        headers.push((
            hyper::header::AUTHORIZATION,
            format!("Bearer {token}", token = tok.as_str()),
        ));

        let mut url_params = format!("?{params}", params = params);
        if let Some(ref api_params) = &params.integration_test_params {
            url_params.push_str(&format!("{}", api_params));
        }

        let full_uri = path + &url_params;

        let opt_request: Option<&EmptyRequest> = None;
        let opt_request = Some(req);
        do_request(&self.client, &full_uri, &headers, "POST", opt_request).await
    }

    /// Creates a new file.
    pub async fn create(&mut self, params: &FilesCreateParams, req: &File) -> Result<File> {
        let rel_path = format!("files",);
        let path = self.format_path(rel_path.as_str());

        let mut headers = vec![];
        let tok;
        if self.scopes.is_empty() {
            let scopes = &["https://www.googleapis.com/auth/drive.file".to_string()];
            tok = self.authenticator.token(scopes).await?;
        } else {
            tok = self.authenticator.token(&self.scopes).await?;
        }
        headers.push((
            hyper::header::AUTHORIZATION,
            format!("Bearer {token}", token = tok.as_str()),
        ));

        let mut url_params = format!("?{params}", params = params);
        if let Some(ref api_params) = &params.integration_test_params {
            url_params.push_str(&format!("{}", api_params));
        }

        let full_uri = path + &url_params;

        let opt_request: Option<&EmptyRequest> = None;
        let opt_request = Some(req);
        do_request(&self.client, &full_uri, &headers, "POST", opt_request).await
    }

    /// Creates a new file.
    ///
    /// This method is a variant of `create()`, taking data for upload. It performs a multipart upload.
    pub async fn create_upload(
        &mut self,
        params: &FilesCreateParams,
        req: &File,
        data: hyper::body::Bytes,
    ) -> Result<File> {
        let rel_path = format!("/upload/drive/v3/files",);
        let path = self.format_path(rel_path.as_str());

        let mut headers = vec![];
        let tok;
        if self.scopes.is_empty() {
            let scopes = &["https://www.googleapis.com/auth/drive.file".to_string()];
            tok = self.authenticator.token(scopes).await?;
        } else {
            tok = self.authenticator.token(&self.scopes).await?;
        }
        headers.push((
            hyper::header::AUTHORIZATION,
            format!("Bearer {token}", token = tok.as_str()),
        ));

        let mut url_params = format!("?uploadType=multipart{params}", params = params);

        if let Some(ref api_params) = &params.integration_test_params {
            url_params.push_str(&format!("{}", api_params));
        }

        let full_uri = path + &url_params;
        let opt_request: Option<&EmptyRequest> = None;
        let opt_request = Some(req);

        do_upload_multipart(&self.client, &full_uri, &headers, "POST", opt_request, data).await
    }

    /// Creates a new file.
    ///
    /// This method is a variant of `create()`, taking data for upload.
    /// It returns a `ResumableUpload` upload manager which you can use to stream larger amounts
    /// of data to the API. The result of this call will be returned by the `ResumableUpload` method
    /// you choose for the upload.
    pub async fn create_resumable_upload<'client>(
        &'client mut self,
        params: &FilesCreateParams,
        req: &File,
    ) -> Result<ResumableUpload<'client, File>> {
        let rel_path = format!("/resumable/upload/drive/v3/files",);
        let path = self.format_path(rel_path.as_str());

        let mut headers = vec![];
        let tok;
        if self.scopes.is_empty() {
            let scopes = &["https://www.googleapis.com/auth/drive.file".to_string()];
            tok = self.authenticator.token(scopes).await?;
        } else {
            tok = self.authenticator.token(&self.scopes).await?;
        }
        headers.push((
            hyper::header::AUTHORIZATION,
            format!("Bearer {token}", token = tok.as_str()),
        ));

        let mut url_params = format!("?uploadType=resumable{params}", params = params);
        if let Some(ref api_params) = &params.integration_test_params {
            url_params.push_str(&format!("{}", api_params));
        }

        let full_uri = path + &url_params;

        let opt_request: Option<&EmptyRequest> = None;
        let opt_request = Some(req);
        let (_resp, headers): (EmptyResponse, hyper::HeaderMap) =
            do_request_with_headers(&self.client, &full_uri, &headers, "POST", opt_request).await?;
        if let Some(dest) = headers.get(hyper::header::LOCATION) {
            use std::convert::TryFrom;
            Ok(ResumableUpload::new(
                hyper::Uri::try_from(dest.to_str()?)?,
                &self.client,
                5 * 1024 * 1024,
            ))
        } else {
            Err(Error::from(ApiError::RedirectError(format!(
                "Resumable upload response didn't contain Location: {:?}",
                headers
            )))
            .context(format!("{:?}", headers)))?
        }
    }

    /// Permanently deletes a file owned by the user without moving it to the trash. If the file belongs to a shared drive the user must be an organizer on the parent. If the target is a folder, all descendants owned by the user are also deleted.
    pub async fn delete(&mut self, params: &FilesDeleteParams) -> Result<()> {
        let rel_path = format!(
            "files/{fileId}",
            fileId = percent_encode(params.file_id.as_bytes(), NON_ALPHANUMERIC)
        );
        let path = self.format_path(rel_path.as_str());

        let mut headers = vec![];
        let tok;
        if self.scopes.is_empty() {
            let scopes = &["https://www.googleapis.com/auth/drive.file".to_string()];
            tok = self.authenticator.token(scopes).await?;
        } else {
            tok = self.authenticator.token(&self.scopes).await?;
        }
        headers.push((
            hyper::header::AUTHORIZATION,
            format!("Bearer {token}", token = tok.as_str()),
        ));

        let mut url_params = format!("?{params}", params = params);
        if let Some(ref api_params) = &params.integration_test_params {
            url_params.push_str(&format!("{}", api_params));
        }

        let full_uri = path + &url_params;

        let opt_request: Option<&EmptyRequest> = None;
        do_request(&self.client, &full_uri, &headers, "DELETE", opt_request).await
    }

    /// Permanently deletes all of the user's trashed files.
    pub async fn empty_trash(&mut self, params: &FilesEmptyTrashParams) -> Result<()> {
        let rel_path = format!("files/trash",);
        let path = self.format_path(rel_path.as_str());

        let mut headers = vec![];
        let tok;
        if self.scopes.is_empty() {
            let scopes = &["https://www.googleapis.com/auth/drive".to_string()];
            tok = self.authenticator.token(scopes).await?;
        } else {
            tok = self.authenticator.token(&self.scopes).await?;
        }
        headers.push((
            hyper::header::AUTHORIZATION,
            format!("Bearer {token}", token = tok.as_str()),
        ));

        let mut url_params = format!("?{params}", params = params);
        if let Some(ref api_params) = &params.integration_test_params {
            url_params.push_str(&format!("{}", api_params));
        }

        let full_uri = path + &url_params;

        let opt_request: Option<&EmptyRequest> = None;
        do_request(&self.client, &full_uri, &headers, "DELETE", opt_request).await
    }

    /// Exports a Google Doc to the requested MIME type and returns the exported content. Please note that the exported content is limited to 10MB.
    ///
    /// This method potentially downloads data. See documentation of `Download`.
    pub async fn export<'a>(
        &'a mut self,
        params: &FilesExportParams,
    ) -> Result<Download<'a, EmptyRequest, ()>> {
        let rel_path = format!(
            "files/{fileId}/export",
            fileId = percent_encode(params.file_id.as_bytes(), NON_ALPHANUMERIC)
        );
        let path = self.format_path(rel_path.as_str());

        let mut headers = vec![];
        let tok;
        if self.scopes.is_empty() {
            let scopes = &["https://www.googleapis.com/auth/drive.readonly".to_string()];
            tok = self.authenticator.token(scopes).await?;
        } else {
            tok = self.authenticator.token(&self.scopes).await?;
        }
        headers.push((
            hyper::header::AUTHORIZATION,
            format!("Bearer {token}", token = tok.as_str()),
        ));

        let mut url_params = format!("?{params}", params = params);
        if let Some(ref api_params) = &params.integration_test_params {
            url_params.push_str(&format!("{}", api_params));
        }

        let full_uri = path + &url_params;
        let opt_request: Option<&EmptyRequest> = None;

        do_download(&self.client, &full_uri, headers, "GET".into(), opt_request).await
    }
}
