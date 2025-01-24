use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Status {
    Ok,
    BadRequest,
    NotFound,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Self::Ok => "200 OK",
            Self::BadRequest => "400 Bad Request",
            Self::NotFound => "404 Not Found",
        }
        .to_string()
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
