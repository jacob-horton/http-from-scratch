use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Method {
    Get,
    Post,
}

impl FromStr for Method {
    type Err = ();

    fn from_str(input: &str) -> Result<Method, Self::Err> {
        match input {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            _ => Err(()),
        }
    }
}
