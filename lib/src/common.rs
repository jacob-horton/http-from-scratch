use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Method {
    Get,
    Post,
    Delete,
    Put,
    Options,
}

impl FromStr for Method {
    type Err = ();

    fn from_str(input: &str) -> Result<Method, Self::Err> {
        match input {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "DELETE" => Ok(Method::Delete),
            "PUT" => Ok(Method::Put),
            "OPTIONS" => Ok(Method::Options),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub name: String,
    pub value: String,
}
