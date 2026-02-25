use std::fmt;
use std::sync::Arc;
use std::task::{Context, Poll};

use axum::Router;
use http::{Method, Request};
use tower::{Layer, Service};

use crate::error::ExtensionError;
use crate::extension::middleware::MiddlewareHook;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AxumRequestContext {
    pub method: Method,
    pub path: String,
}

impl AxumRequestContext {
    fn from_request<B>(request: &Request<B>) -> Self {
        Self {
            method: request.method().clone(),
            path: request.uri().path().to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AxumMiddlewareError {
    pub reason: String,
}

impl From<ExtensionError> for AxumMiddlewareError {
    fn from(value: ExtensionError) -> Self {
        Self {
            reason: value.to_string(),
        }
    }
}

pub type AxumMiddlewareHook = Arc<dyn MiddlewareHook<AxumRequestContext>>;

#[derive(Clone)]
pub struct HttpSocketAxumLayer {
    hook: Option<AxumMiddlewareHook>,
    protected_prefixes: Vec<String>,
}

impl fmt::Debug for HttpSocketAxumLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpSocketAxumLayer")
            .field("has_hook", &self.hook.is_some())
            .field("protected_prefixes", &self.protected_prefixes)
            .finish()
    }
}

impl Default for HttpSocketAxumLayer {
    fn default() -> Self {
        Self {
            hook: None,
            protected_prefixes: vec!["/socket".to_string(), "/ws".to_string()],
        }
    }
}

impl HttpSocketAxumLayer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_hook(mut self, hook: AxumMiddlewareHook) -> Self {
        self.hook = Some(hook);
        self
    }

    pub fn protect_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.protected_prefixes.push(prefix.into());
        self
    }

    fn should_check(&self, path: &str) -> bool {
        self.protected_prefixes
            .iter()
            .any(|prefix| path.starts_with(prefix))
    }
}

impl<S> Layer<S> for HttpSocketAxumLayer {
    type Service = HttpSocketAxumService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        HttpSocketAxumService {
            inner,
            layer: self.clone(),
        }
    }
}

#[derive(Clone)]
pub struct HttpSocketAxumService<S> {
    inner: S,
    layer: HttpSocketAxumLayer,
}

impl<S, B> Service<Request<B>> for HttpSocketAxumService<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<B>) -> Self::Future {
        if self.layer.should_check(request.uri().path())
            && let Some(hook) = &self.layer.hook
        {
            let context = AxumRequestContext::from_request(&request);
            if let Err(error) = hook.before(&context) {
                request
                    .extensions_mut()
                    .insert(AxumMiddlewareError::from(error));
            }
        }

        self.inner.call(request)
    }
}

pub trait RouterHttpSocketExt<S> {
    fn with_http_socket(self, layer: HttpSocketAxumLayer) -> Self;
}

impl<S> RouterHttpSocketExt<S> for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn with_http_socket(self, layer: HttpSocketAxumLayer) -> Self {
        self.layer(layer)
    }
}
