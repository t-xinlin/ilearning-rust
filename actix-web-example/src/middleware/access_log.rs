//! For middleware documentation, see [`Logger`].

use std::{
    borrow::Cow,
    collections::HashSet,
    convert::TryFrom,
    env,
    fmt::{self, Display as _},
    future::Future,
    marker::PhantomData,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

use actix_service::{Service, Transform};
use actix_utils::future::{ready, Ready};
use bytes::Bytes;
use futures_core::ready;
use log::{debug, warn};
use pin_project_lite::pin_project;
use regex::{Regex, RegexSet};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use actix_web::{
    body::{BodySize, MessageBody},
    http::header::HeaderName,
    // service::{ServiceRequest, ServiceResponse},
    Error, HttpResponse, Result,
};

use actix_web::dev::{
    ServiceRequest,
    ServiceResponse,
};

/// Middleware for logging request and response summaries to the terminal.
///
/// This middleware uses the `log` crate to output information. Enable `log`'s output for the
/// "actix_web" scope using [`env_logger`](https://docs.rs/env_logger) or similar crate.
///
/// # Default Format
/// The [`default`](Logger::default) Logger uses the following format:
///
/// ```plain
/// %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
///
/// Example Output:
/// 127.0.0.1:54278 "GET /test HTTP/1.1" 404 20 "-" "HTTPie/2.2.0" 0.001074
/// ```
///
/// # Examples
/// ```
/// use actix_web::{middleware::Logger, App};
///
/// // access logs are printed with the INFO level so ensure it is enabled by default
/// env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
///
/// let app = App::new()
///     // .wrap(Logger::default())
///     .wrap(Logger::new("%a %{User-Agent}i"));
/// ```
///
/// # Format
/// Variable | Description
/// -------- | -----------
/// `%%` | The percent sign
/// `%a` | Peer IP address (or IP address of reverse proxy if used)
/// `%t` | Time when the request started processing (in RFC 3339 format)
/// `%r` | First line of request (Example: `GET /test HTTP/1.1`)
/// `%s` | Response status code
/// `%b` | Size of response in bytes, including HTTP headers
/// `%T` | Time taken to serve the request, in seconds to 6 decimal places
/// `%D` | Time taken to serve the request, in milliseconds
/// `%U` | Request URL
/// `%{r}a` | "Real IP" remote address **\***
/// `%{FOO}i` |  `request.headers["FOO"]`
/// `%{FOO}o` | `response.headers["FOO"]`
/// `%{FOO}e` | `env_var["FOO"]`
/// `%{FOO}xi` | [Custom request replacement](Logger::custom_request_replace) labelled "FOO"
///
/// # Security
/// **\*** "Real IP" remote address is calculated using
/// [`ConnectionInfo::realip_remote_addr()`](crate::dev::ConnectionInfo::realip_remote_addr())
///
/// If you use this value, ensure that all requests come from trusted hosts. Otherwise, it is
/// trivial for the remote client to falsify their source IP address.
#[derive(Debug)]
pub struct AccessLogging(Rc<Inner>);

#[derive(Debug, Clone)]
struct Inner {
    format: Format,
    exclude: HashSet<String>,
    exclude_regex: RegexSet,
    log_target: Cow<'static, str>,
}

impl AccessLogging {
    /// Create `Logger` middleware with the specified `format`.
    pub fn new(format: &str) -> AccessLogging {
        AccessLogging(Rc::new(Inner {
            format: Format::new(format),
            exclude: HashSet::new(),
            exclude_regex: RegexSet::empty(),
            log_target: Cow::Borrowed(module_path!()),
        }))
    }

    /// Ignore and do not log access info for specified path.
    pub fn exclude<T: Into<String>>(mut self, path: T) -> Self {
        Rc::get_mut(&mut self.0)
            .unwrap()
            .exclude
            .insert(path.into());
        self
    }

    /// Ignore and do not log access info for paths that match regex.
    pub fn exclude_regex<T: Into<String>>(mut self, path: T) -> Self {
        let inner = Rc::get_mut(&mut self.0).unwrap();
        let mut patterns = inner.exclude_regex.patterns().to_vec();
        patterns.push(path.into());
        let regex_set = RegexSet::new(patterns).unwrap();
        inner.exclude_regex = regex_set;
        self
    }

    /// Sets the logging target to `target`.
    ///
    /// By default, the log target is `module_path!()` of the log call location. In our case, that
    /// would be `actix_web::middleware::logger`.
    ///
    /// # Examples
    /// Using `.log_target("http_log")` would have this effect on request logs:
    /// ```diff
    /// - [2015-10-21T07:28:00Z INFO  actix_web::middleware::logger] 127.0.0.1 "GET / HTTP/1.1" 200 88 "-" "dmc/1.0" 0.001985
    /// + [2015-10-21T07:28:00Z INFO  http_log] 127.0.0.1 "GET / HTTP/1.1" 200 88 "-" "dmc/1.0" 0.001985
    ///                               ^^^^^^^^
    /// ```
    pub fn log_target(mut self, target: impl Into<Cow<'static, str>>) -> Self {
        let inner = Rc::get_mut(&mut self.0).unwrap();
        inner.log_target = target.into();
        self
    }

    /// Register a function that receives a ServiceRequest and returns a String for use in the
    /// log line. The label passed as the first argument should match a replacement substring in
    /// the logger format like `%{label}xi`.
    ///
    /// It is convention to print "-" to indicate no output instead of an empty string.
    ///
    /// # Examples
    /// ```
    /// # use actix_web::http::{header::HeaderValue};
    /// # use actix_web::middleware::Logger;
    /// # fn parse_jwt_id (_req: Option<&HeaderValue>) -> String { "jwt_uid".to_owned() }
    /// Logger::new("example %{JWT_ID}xi")
    ///     .custom_request_replace("JWT_ID", |req| parse_jwt_id(req.headers().get("Authorization")));
    /// ```
    pub fn custom_request_replace(
        mut self,
        label: &str,
        f: impl Fn(&ServiceRequest) -> String + 'static,
    ) -> Self {
        let inner = Rc::get_mut(&mut self.0).unwrap();

        let ft = inner.format.0.iter_mut().find(
            |ft| matches!(ft, FormatText::CustomRequest(unit_label, _) if label == unit_label),
        );

        if let Some(FormatText::CustomRequest(_, request_fn)) = ft {
            // replace into None or previously registered fn using same label
            request_fn.replace(CustomRequestFn {
                inner_fn: Rc::new(f),
            });
        } else {
            // non-printed request replacement function diagnostic
            debug!(
                "Attempted to register custom request logging function for nonexistent label: {}",
                label
            );
        }

        self
    }
}

impl Default for AccessLogging {
    /// Create `Logger` middleware with format:
    ///
    /// ```plain
    /// %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
    /// ```
    fn default() -> AccessLogging {
        AccessLogging(Rc::new(Inner {
            format: Format::default(),
            exclude: HashSet::new(),
            exclude_regex: RegexSet::empty(),
            log_target: Cow::Borrowed(module_path!()),
        }))
    }
}

impl<S, B> Transform<S, ServiceRequest> for AccessLogging
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        B: MessageBody,
{
    type Response = ServiceResponse<StreamLog<B>>;
    type Error = Error;
    type Transform = AccessLoggingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        for unit in &self.0.format.0 {
            // missing request replacement function diagnostic
            if let FormatText::CustomRequest(label, None) = unit {
                warn!(
                    "No custom request replacement function was registered for label \"{}\".",
                    label
                );
            }
        }

        ready(Ok(AccessLoggingMiddleware {
            service,
            inner: self.0.clone(),
        }))
    }
}

/// Logger middleware service.
pub struct AccessLoggingMiddleware<S> {
    inner: Rc<Inner>,
    service: S,
}

impl<S, B> Service<ServiceRequest> for AccessLoggingMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        B: MessageBody,
{
    type Response = ServiceResponse<StreamLog<B>>;
    type Error = Error;
    type Future = LoggerResponse<S, B>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let excluded = self.inner.exclude.contains(req.path())
            || self.inner.exclude_regex.is_match(req.path());

        if excluded {
            LoggerResponse {
                fut: self.service.call(req),
                format: None,
                time: OffsetDateTime::now_utc(),
                log_target: Cow::Borrowed(""),
                _phantom: PhantomData,
            }
        } else {
            let now = OffsetDateTime::now_utc();
            let mut format = self.inner.format.clone();

            for unit in &mut format.0 {
                unit.render_request(now, &req);
            }

            LoggerResponse {
                fut: self.service.call(req),
                format: Some(format),
                time: now,
                log_target: self.inner.log_target.clone(),
                _phantom: PhantomData,
            }
        }
    }
}

pin_project! {
    pub struct LoggerResponse<S, B>
    where
        B: MessageBody,
        S: Service<ServiceRequest>,
    {
        #[pin]
        fut: S::Future,
        time: OffsetDateTime,
        format: Option<Format>,
        log_target: Cow<'static, str>,
        _phantom: PhantomData<B>,
    }
}

impl<S, B> Future for LoggerResponse<S, B>
    where
        B: MessageBody,
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    type Output = Result<ServiceResponse<StreamLog<B>>, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        let res = match ready!(this.fut.poll(cx)) {
            Ok(res) => res,
            Err(e) => return Poll::Ready(Err(e)),
        };

        if let Some(error) = res.response().error() {
            debug!("Error in response: {:?}", error);
        }

        if let Some(ref mut format) = this.format {
            for unit in &mut format.0 {
                unit.render_response(res.response());
            }
        }

        let time = *this.time;
        let format = this.format.take();
        let log_target = this.log_target.clone();

        Poll::Ready(Ok(res.map_body(move |_, body| StreamLog {
            body,
            time,
            format,
            size: 0,
            log_target,
        })))
    }
}

pin_project! {
    pub struct StreamLog<B> {
        #[pin]
        body: B,
        format: Option<Format>,
        size: usize,
        time: OffsetDateTime,
        log_target: Cow<'static, str>,
    }

    impl<B> PinnedDrop for StreamLog<B> {
        fn drop(this: Pin<&mut Self>) {
            if let Some(ref format) = this.format {
                let render = |fmt: &mut fmt::Formatter<'_>| {
                    for unit in &format.0 {
                        unit.render(fmt, this.size, this.time)?;
                    }
                    Ok(())
                };

                log::info!(
                    target: this.log_target.as_ref(),
                    "{}", FormatDisplay(&render)
                );
            }
        }
    }
}

impl<B: MessageBody> MessageBody for StreamLog<B> {
    type Error = B::Error;

    #[inline]
    fn size(&self) -> BodySize {
        self.body.size()
    }

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        let this = self.project();

        match ready!(this.body.poll_next(cx)) {
            Some(Ok(chunk)) => {
                *this.size += chunk.len();
                Poll::Ready(Some(Ok(chunk)))
            }
            Some(Err(err)) => Poll::Ready(Some(Err(err))),
            None => Poll::Ready(None),
        }
    }
}

/// A formatting style for the `Logger` consisting of multiple concatenated `FormatText` items.
#[derive(Debug, Clone)]
struct Format(Vec<FormatText>);

impl Default for Format {
    /// Return the default formatting style for the `Logger`:
    fn default() -> Format {
        Format::new(r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#)
    }
}

impl Format {
    /// Create a `Format` from a format string.
    ///
    /// Returns `None` if the format string syntax is incorrect.
    pub fn new(s: &str) -> Format {
        log::trace!("Access log format: {}", s);
        let fmt = Regex::new(r"%(\{([A-Za-z0-9\-_]+)\}([aioe]|xi)|[%atPrUsbTD]?)").unwrap();

        let mut idx = 0;
        let mut results = Vec::new();
        for cap in fmt.captures_iter(s) {
            let m = cap.get(0).unwrap();
            let pos = m.start();
            if idx != pos {
                results.push(FormatText::Str(s[idx..pos].to_owned()));
            }
            idx = m.end();

            if let Some(key) = cap.get(2) {
                results.push(match cap.get(3).unwrap().as_str() {
                    "a" => {
                        if key.as_str() == "r" {
                            FormatText::RealIpRemoteAddr
                        } else {
                            unreachable!()
                        }
                    }
                    "i" => {
                        FormatText::RequestHeader(HeaderName::try_from(key.as_str()).unwrap())
                    }
                    "o" => {
                        FormatText::ResponseHeader(HeaderName::try_from(key.as_str()).unwrap())
                    }
                    "e" => FormatText::EnvironHeader(key.as_str().to_owned()),
                    "xi" => FormatText::CustomRequest(key.as_str().to_owned(), None),
                    _ => unreachable!(),
                })
            } else {
                let m = cap.get(1).unwrap();
                results.push(match m.as_str() {
                    "%" => FormatText::Percent,
                    "a" => FormatText::RemoteAddr,
                    "t" => FormatText::RequestTime,
                    "r" => FormatText::RequestLine,
                    "s" => FormatText::ResponseStatus,
                    "b" => FormatText::ResponseSize,
                    "U" => FormatText::UrlPath,
                    "T" => FormatText::Time,
                    "D" => FormatText::TimeMillis,
                    _ => FormatText::Str(m.as_str().to_owned()),
                });
            }
        }
        if idx != s.len() {
            results.push(FormatText::Str(s[idx..].to_owned()));
        }

        Format(results)
    }
}

/// A string of text to be logged.
///
/// This is either one of the data fields supported by the `Logger`, or a custom `String`.
#[non_exhaustive]
#[derive(Debug, Clone)]
enum FormatText {
    Str(String),
    Percent,
    RequestLine,
    RequestTime,
    ResponseStatus,
    ResponseSize,
    Time,
    TimeMillis,
    RemoteAddr,
    RealIpRemoteAddr,
    UrlPath,
    RequestHeader(HeaderName),
    ResponseHeader(HeaderName),
    EnvironHeader(String),
    CustomRequest(String, Option<CustomRequestFn>),
}

#[derive(Clone)]
struct CustomRequestFn {
    inner_fn: Rc<dyn Fn(&ServiceRequest) -> String>,
}

impl CustomRequestFn {
    fn call(&self, req: &ServiceRequest) -> String {
        (self.inner_fn)(req)
    }
}

impl fmt::Debug for CustomRequestFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("custom_request_fn")
    }
}

impl FormatText {
    fn render(
        &self,
        fmt: &mut fmt::Formatter<'_>,
        size: usize,
        entry_time: OffsetDateTime,
    ) -> Result<(), fmt::Error> {
        match self {
            FormatText::Str(ref string) => fmt.write_str(string),
            FormatText::Percent => "%".fmt(fmt),
            FormatText::ResponseSize => size.fmt(fmt),
            FormatText::Time => {
                let rt = OffsetDateTime::now_utc() - entry_time;
                let rt = rt.as_seconds_f64();
                fmt.write_fmt(format_args!("{:.6}", rt))
            }
            FormatText::TimeMillis => {
                let rt = OffsetDateTime::now_utc() - entry_time;
                let rt = (rt.whole_nanoseconds() as f64) / 1_000_000.0;
                fmt.write_fmt(format_args!("{:.6}", rt))
            }
            FormatText::EnvironHeader(ref name) => {
                if let Ok(val) = env::var(name) {
                    fmt.write_fmt(format_args!("{}", val))
                } else {
                    "-".fmt(fmt)
                }
            }
            _ => Ok(()),
        }
    }

    fn render_response<B>(&mut self, res: &HttpResponse<B>) {
        match self {
            FormatText::ResponseStatus => {
                *self = FormatText::Str(format!("{}", res.status().as_u16()))
            }
            FormatText::ResponseHeader(ref name) => {
                let s = if let Some(val) = res.headers().get(name) {
                    if let Ok(s) = val.to_str() {
                        s
                    } else {
                        "-"
                    }
                } else {
                    "-"
                };
                *self = FormatText::Str(s.to_string())
            }
            _ => {}
        }
    }

    fn render_request(&mut self, now: OffsetDateTime, req: &ServiceRequest) {
        match self {
            FormatText::RequestLine => {
                *self = if req.query_string().is_empty() {
                    FormatText::Str(format!(
                        "{} {} {:?}",
                        req.method(),
                        req.path(),
                        req.version()
                    ))
                } else {
                    FormatText::Str(format!(
                        "{} {}?{} {:?}",
                        req.method(),
                        req.path(),
                        req.query_string(),
                        req.version()
                    ))
                };
            }
            FormatText::UrlPath => *self = FormatText::Str(req.path().to_string()),
            FormatText::RequestTime => *self = FormatText::Str(now.format(&Rfc3339).unwrap()),
            FormatText::RequestHeader(ref name) => {
                let s = if let Some(val) = req.headers().get(name) {
                    if let Ok(s) = val.to_str() {
                        s
                    } else {
                        "-"
                    }
                } else {
                    "-"
                };
                *self = FormatText::Str(s.to_string());
            }
            FormatText::RemoteAddr => {
                let s = if let Some(peer) = req.connection_info().peer_addr() {
                    FormatText::Str((*peer).to_string())
                } else {
                    FormatText::Str("-".to_string())
                };
                *self = s;
            }
            FormatText::RealIpRemoteAddr => {
                let s = if let Some(remote) = req.connection_info().realip_remote_addr() {
                    FormatText::Str(remote.to_string())
                } else {
                    FormatText::Str("-".to_string())
                };
                *self = s;
            }
            FormatText::CustomRequest(_, request_fn) => {
                let s = match request_fn {
                    Some(f) => FormatText::Str(f.call(req)),
                    None => FormatText::Str("-".to_owned()),
                };

                *self = s;
            }
            _ => {}
        }
    }
}

/// Converter to get a String from something that writes to a Formatter.
pub(crate) struct FormatDisplay<'a>(
    &'a dyn Fn(&mut fmt::Formatter<'_>) -> Result<(), fmt::Error>,
);

impl<'a> fmt::Display for FormatDisplay<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        (self.0)(fmt)
    }
}

#[cfg(test)]
mod tests {
    async fn test_logger() {

    }
}
