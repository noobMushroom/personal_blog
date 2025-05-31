use crate::error::HttpError;

enum Codes {
    Ok,
    NotFound,
}

pub fn get_route(req: &str) -> Result<&str, HttpError> {
    req.lines()
        .next()
        .ok_or_else(|| HttpError::UnexpectedRequest("route missing".into()))
        .and_then(|line| {
            line.split_whitespace()
                .nth(1)
                .ok_or_else(|| HttpError::UnexpectedRoute(line.to_string()))
        })
}

pub fn get_response(html: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: text/html\r\n\
         Access-Control-Allow-Origin: *\r\n\
          connection: Closed\r\n\
         Content-Length: \r\n\
         \r\n\
{}
         ",
        html
    )
}
