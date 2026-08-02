use std::borrow::Cow;
use std::fmt::Display;
use std::time::Duration;

use bytes::Bytes;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::{Method, Request, Uri};
use serde::Serialize;
use serde::de::DeserializeOwned;
use url::Url;

use crate::error::{BaseUrlError, Body, BuildRequestError, Error, Result};
use crate::transport::{HttpTransport, Timeout};
use crate::types::invalid_value::InvalidValue;
use crate::types::token::Token;
use crate::types::try_into_value::TryIntoValue;
use crate::{ApiError, api};

#[derive(Debug, Clone)]
pub struct Client<T> {
    transport: T,
    base_url: Url,
    token: Option<Token>,
    timeout: Option<Timeout>,
}

impl<T> Client<T> {
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub fn transport(&self) -> &T {
        &self.transport
    }

    pub fn timeout(&self) -> Option<Duration> {
        self.timeout.map(Timeout::duration)
    }

    pub fn with_timeout(&self, timeout: Duration) -> Client<&T> {
        self.rebind(Some(Timeout::new(timeout)))
    }

    pub fn without_timeout(&self) -> Client<&T> {
        self.rebind(None)
    }

    fn rebind(&self, timeout: Option<Timeout>) -> Client<&T> {
        Client {
            transport: &self.transport,
            base_url: self.base_url.clone(),
            token: self.token.clone(),
            timeout,
        }
    }
}

impl<T: HttpTransport> Client<T> {
    pub fn new(transport: T, base_url: impl AsRef<str>) -> Result<Self> {
        Ok(ClientBuilder::new(base_url)?.build(transport))
    }

    pub fn account(&self) -> api::Account<'_, T> {
        api::Account::new(self)
    }

    pub fn assets(&self) -> api::Assets<'_, T> {
        api::Assets::new(self)
    }

    pub fn audio(&self) -> api::Audio<'_, T> {
        api::Audio::new(self)
    }

    pub fn ble(&self) -> api::Ble<'_, T> {
        api::Ble::new(self)
    }

    pub fn busy(&self) -> api::Busy<'_, T> {
        api::Busy::new(self)
    }

    pub fn display(&self) -> api::Display<'_, T> {
        api::Display::new(self)
    }

    pub fn input(&self) -> api::Input<'_, T> {
        api::Input::new(self)
    }

    pub fn settings(&self) -> api::Settings<'_, T> {
        api::Settings::new(self)
    }

    pub fn smart_home(&self) -> api::SmartHome<'_, T> {
        api::SmartHome::new(self)
    }

    pub fn storage(&self) -> api::Storage<'_, T> {
        api::Storage::new(self)
    }

    pub fn system(&self) -> api::System<'_, T> {
        api::System::new(self)
    }

    pub fn time(&self) -> api::Time<'_, T> {
        api::Time::new(self)
    }

    pub fn updater(&self) -> api::Updater<'_, T> {
        api::Updater::new(self)
    }

    pub fn wifi(&self) -> api::Wifi<'_, T> {
        api::Wifi::new(self)
    }

    pub(crate) async fn json<R: DeserializeOwned>(&self, request: Call) -> Result<R> {
        let method = request.method.clone();
        let path = request.path.to_string();

        let body = self.send(request).await?;

        serde_json::from_slice(&body).map_err(|source| Error::Decode {
            method,
            path,
            body: Body::new(&body),
            source,
        })
    }

    pub(crate) async fn bytes(&self, request: Call) -> Result<Bytes> {
        self.send(request).await
    }

    pub(crate) async fn ok(&self, request: Call) -> Result<()> {
        let method = request.method.clone();
        let path = request.path.to_string();

        let body = self.send(request).await?;

        if body.is_empty() {
            return Ok(());
        }

        serde_json::from_slice::<SuccessResponse>(&body)
            .map(|_| ())
            .map_err(|source| Error::Decode {
                method,
                path,
                body: Body::new(&body),
                source,
            })
    }

    async fn send(&self, request: Call) -> Result<Bytes> {
        let method = request.method.clone();
        let path = request.path.to_string();

        let request = self.build(request).map_err(|source| Error::BuildRequest {
            method: method.clone(),
            path: path.clone(),
            source,
        })?;

        let response =
            self.transport
                .execute(request)
                .await
                .map_err(|source| Error::Transport {
                    method: method.clone(),
                    path: path.clone(),
                    source,
                })?;

        let status = response.status();
        let body = response.into_body();

        if status.is_success() {
            return Ok(body);
        }

        Err(match serde_json::from_slice::<ApiError>(&body) {
            Ok(error) => Error::Api {
                method,
                path,
                status,
                error,
            },
            Err(_) => Error::UnexpectedStatus {
                method,
                path,
                status,
                body: Body::new(&body),
            },
        })
    }

    fn build(&self, request: Call) -> Result<Request<Bytes>, BuildRequestError> {
        let mut url = self.base_url.join(request.path.trim_start_matches('/'))?;

        if !request.query.is_empty() {
            let mut pairs = url.query_pairs_mut();

            for (name, value) in &request.query {
                pairs.append_pair(name, value);
            }
        }

        let mut builder = Request::builder()
            .method(request.method)
            .uri(Uri::try_from(url.as_str())?);

        if let Some(timeout) = self.timeout {
            builder = builder.extension(timeout);
        }

        if let Some(token) = &self.token {
            builder = builder.header(AUTHORIZATION, format!("Bearer {}", token.as_str()));
        }

        match request.body {
            Some(body) => builder
                .header(CONTENT_TYPE, body.content_type)
                .body(body.bytes)
                .map_err(Into::into),
            None => builder.body(Bytes::new()).map_err(Into::into),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientBuilder {
    base_url: Url,
    token: Option<Token>,
    timeout: Option<Timeout>,
}

impl ClientBuilder {
    pub fn new(base_url: impl AsRef<str>) -> Result<Self> {
        let base_url = base_url.as_ref();
        let parsed = Self::parse_base_url(base_url).map_err(|source| Error::BaseUrl {
            url: base_url.to_owned(),
            source,
        })?;
        Ok(Self {
            base_url: parsed,
            token: None,
            timeout: None,
        })
    }

    pub fn token(mut self, token: impl TryIntoValue<Token>) -> Result<Self, InvalidValue> {
        self.token = Some(token.try_into_value()?);
        Ok(self)
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(Timeout::new(timeout));
        self
    }

    pub fn build<T: HttpTransport>(self, transport: T) -> Client<T> {
        Client {
            transport,
            base_url: self.base_url,
            token: self.token,
            timeout: self.timeout,
        }
    }

    fn parse_base_url(base_url: &str) -> Result<Url, BaseUrlError> {
        let mut url = Url::parse(base_url)?;

        if !matches!(url.scheme(), "http" | "https") {
            return Err(BaseUrlError::UnsupportedScheme(url.scheme().to_owned()));
        }

        if !url.has_host() {
            return Err(BaseUrlError::MissingHost);
        }

        if url.query().is_some() || url.fragment().is_some() {
            return Err(BaseUrlError::HasQueryOrFragment);
        }

        if !url.path().ends_with('/') {
            let path = format!("{}/", url.path());
            url.set_path(&path);
        }

        Ok(url)
    }
}

#[cfg(feature = "reqwest")]
impl ClientBuilder {
    pub fn build_reqwest(self) -> Client<crate::transport::ReqwestHttpTransport> {
        self.build(crate::transport::ReqwestHttpTransport::new())
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct SuccessResponse {
    // TODO, currently only using this in the tests via json
    #[allow(unused)]
    result: String,
}

pub(crate) struct Call {
    method: Method,
    path: Cow<'static, str>,
    query: Vec<(&'static str, String)>,
    body: Option<RequestBody>,
}

struct RequestBody {
    content_type: &'static str,
    bytes: Bytes,
}

impl Call {
    pub(crate) fn get(path: impl Into<Cow<'static, str>>) -> Self {
        Self::new(Method::GET, path)
    }

    pub(crate) fn post(path: impl Into<Cow<'static, str>>) -> Self {
        Self::new(Method::POST, path)
    }

    pub(crate) fn put(path: impl Into<Cow<'static, str>>) -> Self {
        Self::new(Method::PUT, path)
    }

    pub(crate) fn delete(path: impl Into<Cow<'static, str>>) -> Self {
        Self::new(Method::DELETE, path)
    }

    fn new(method: Method, path: impl Into<Cow<'static, str>>) -> Self {
        Self {
            method,
            path: path.into(),
            query: Vec::new(),
            body: None,
        }
    }

    pub(crate) fn query(mut self, name: &'static str, value: impl Display) -> Self {
        self.query.push((name, value.to_string()));
        self
    }

    pub(crate) fn maybe_query(self, name: &'static str, value: Option<impl Display>) -> Self {
        match value {
            Some(value) => self.query(name, value),
            None => self,
        }
    }

    pub(crate) fn json<B: Serialize + ?Sized>(mut self, body: &B) -> Result<Self> {
        let bytes = serde_json::to_vec(body).map_err(|source| Error::BuildRequest {
            method: self.method.clone(),
            path: self.path.to_string(),
            source: BuildRequestError::Body(source),
        })?;
        self.body = Some(RequestBody {
            content_type: "application/json",
            bytes: Bytes::from(bytes),
        });
        Ok(self)
    }

    pub(crate) fn octet_stream(mut self, bytes: impl Into<Bytes>) -> Self {
        self.body = Some(RequestBody {
            content_type: "application/octet-stream",
            bytes: bytes.into(),
        });
        self
    }
}
