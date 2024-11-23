use std::str::FromStr;

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub protocal: String
}

#[derive(Debug, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    UNKOWN
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "HEAD" => Ok(HttpMethod::HEAD),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            _ => Ok(HttpMethod::UNKOWN)
        }
    }
}

pub fn parse_request(buffer: &[u8]) -> Option<Request> {
    let request = String::from_utf8_lossy(buffer);
    let mut lines = request.lines();
    let first_line = lines.next()?;

    let mut parts = first_line.split_whitespace();
    let method = parts.next()?.parse::<HttpMethod>().unwrap_or(HttpMethod::UNKNOWN);
    let path = String::from(parts.next()?);
    let protocal = String::from(parts.next()?);

    Some(Request {
        method,
        path,
        protocal
    })
}