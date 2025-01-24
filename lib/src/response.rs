use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Status {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,

    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthInfo = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    ImUsed = 226,

    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    Unused = 306,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    ContentTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableContent = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,

    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthRequired = 511,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        let english = match self {
            Self::Continue => "Continue",
            Self::SwitchingProtocols => "Switching Protocols",
            Self::Processing => "Processing",
            Self::EarlyHints => "Early Hints",

            Self::Ok => "OK",
            Self::Created => "Created",
            Self::Accepted => "Accepted",
            Self::NonAuthInfo => "Non-Authoritative Information",
            Self::NoContent => "No Content",
            Self::ResetContent => "Reset Content",
            Self::PartialContent => "Partial Content",
            Self::MultiStatus => "Multi-Status",
            Self::AlreadyReported => "Already Reported",
            Self::ImUsed => "IM Used",

            Self::MultipleChoices => "Multiple Choices",
            Self::MovedPermanently => "Moved Permanently",
            Self::Found => "Found",
            Self::SeeOther => "See Other",
            Self::NotModified => "Not Modified",
            Self::UseProxy => "Use Proxy",
            Self::Unused => "Unused",
            Self::TemporaryRedirect => "Temporary Redirect",
            Self::PermanentRedirect => "Permanent Redirect",

            Self::BadRequest => "Bad Request",
            Self::Unauthorized => "Unauthorized",
            Self::PaymentRequired => "Payment Required",
            Self::Forbidden => "Forbidden",
            Self::NotFound => "Not Found",
            Self::MethodNotAllowed => "Method Not Allowed",
            Self::NotAcceptable => "Not Acceptable",
            Self::ProxyAuthRequired => "Proxy Authentication Required",
            Self::RequestTimeout => "Request Timeout",
            Self::Conflict => "Conflict",
            Self::Gone => "Gone",
            Self::LengthRequired => "Length Required",
            Self::PreconditionFailed => "Precondition Failed",
            Self::ContentTooLarge => "Content Too Large",
            Self::UriTooLong => "URI Too Long",
            Self::UnsupportedMediaType => "Unsupported Media Type",
            Self::RangeNotSatisfiable => "Range Not Satisfiable",
            Self::ExpectationFailed => "Expectation Failed",
            Self::ImATeapot => "I'm A Teapot",
            Self::MisdirectedRequest => "Misdirected Request",
            Self::UnprocessableContent => "Unprocessable Content",
            Self::Locked => "Locked",
            Self::FailedDependency => "Failed Dependency",
            Self::TooEarly => "Too Early",
            Self::UpgradeRequired => "Upgrade Required",
            Self::PreconditionRequired => "Precondition Required",
            Self::TooManyRequests => "Too Many Requests",
            Self::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            Self::UnavailableForLegalReasons => "Unavailable For Legal Reasons",

            Self::InternalServerError => "Internal Server Error",
            Self::NotImplemented => "Not Implemented",
            Self::BadGateway => "Bad Gateway",
            Self::ServiceUnavailable => "Service Unavailable",
            Self::GatewayTimeout => "Gateway Timeout",
            Self::HttpVersionNotSupported => "HTTP Version Not Supported",
            Self::VariantAlsoNegotiates => "Variant Also Negotiates",
            Self::InsufficientStorage => "Insufficient Storage",
            Self::LoopDetected => "Loop Detected",
            Self::NotExtended => "Not Extended",
            Self::NetworkAuthRequired => "Network Authentication Required",
        };

        return format!("{} {}", self.to_owned() as u32, english);
    }
}

#[derive(Debug, Clone)]
pub struct Response {
    pub version: String,
    pub status_code: Status,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.version);
        result.push_str(" ");
        result.push_str(&self.status_code.to_string());
        result.push_str("\r\n");

        for (k, v) in &self.headers {
            // This is calculatd and added later
            if k.to_lowercase() == "content-length" {
                continue;
            }

            result.push_str(&k.to_string());
            result.push_str(": ");
            result.push_str(&v.to_string());
            result.push_str("\r\n");
        }

        if let Some(body) = &self.body {
            result.push_str("Content-Lenth: ");
            result.push_str(&body.len().to_string());

            result.push_str("\r\n\r\n");
            result.push_str(body);
        } else {
            result.push_str("Content-Lenth: 0");
        }

        return result;
    }
}
