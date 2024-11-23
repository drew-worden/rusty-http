pub enum HttpStatus {
    Ok,
    NotFound,
    BadRequest,
    InternalServerError,
}

pub enum ContentType {
    Text,
    Html,
    Json,
    Css,
    Js,
    Png,
    Jpg,
}

impl HttpStatus {
    pub fn as_code(&self) -> u16 {
        match self {
            HttpStatus::Ok => 200,
            HttpStatus::NotFound => 404,
            HttpStatus::BadRequest => 400,
            HttpStatus::InternalServerError => 500,
        }
    }

    pub fn as_text(&self) -> &'static str {
        match self {
            HttpStatus::Ok => "OK",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::BadRequest => "Bad Request",
            HttpStatus::InternalServerError => "Internal Server Error",
        }
    }
}

pub struct Response {
    status: HttpStatus,
    content_type: ContentType,
    body: String,
}

impl Response {
    pub fn new(status: HttpStatus, content_type: ContentType, body: String) -> Self {
        Response {
            status,
            content_type,
            body,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status.as_code(),
            self.status.as_text(),
            match self.content_type {
                ContentType::Text => "text/plain",
                ContentType::Html => "text/html",
                ContentType::Json => "application/json",
                ContentType::Css => "text/css",
                ContentType::Js => "application/javascript",
                ContentType::Png => "image/png",
                ContentType::Jpg => "image/jpeg" | "image/jpg",
            },
            self.body.len(),
            self.body
        )
    }
}

