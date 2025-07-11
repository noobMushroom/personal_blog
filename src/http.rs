use crate::error::AppError;
use crate::html::get_html_template;

enum Codes {
    Ok,
    NotFound,
}

// Returns response for successful login
pub fn get_successful_login(session_id: &str) -> String {
    format!(
        "HTTP/1.1 302 Found\r\n\
         Location: /\r\n\
         Set-Cookie: session_id={}; Path=/; HttpOnly; SameSite=Strict\r\n\
         Content-Length: 0\r\n\
          Connection: close\r\n\
         \r\n",
        session_id
    )
}

pub fn get_page_not_found(http: &str) -> String {
    format!(
        "HTTP/1.1 404 Not Found\r\n\
        Content-Type: text/html; charset=utf-8\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\r\n\
        {}",
        http.len(),
        http
    )
}

pub fn get_successful_article_add<'a>() -> &'a str {
    "HTTP/1.1 302 Found\r\n\
         Location: /\r\n\
         Content-Length: 0\r\n\
         Connection: closed\r\n\
         \r\n"
}
pub async fn get_failed_login_with_body() -> Result<String, AppError> {
    let html = get_html_template("failed_login.html").await?;
    Ok(format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: text/html\r\n\
         Content-Length: {}\r\n\
         \r\n\
         {}",
        html.len(),
        html
    ))
}

pub fn get_response(html: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: text/html\r\n\
         Access-Control-Allow-Origin: *\r\n\
         connection: Closed\r\n\
         Content-Length: {} \r\n\
         \r\n\
{}
         ",
        html.len(),
        html
    )
}

pub fn redirect_to_login<'a>() -> &'a str {
    "HTTP/1.1 302 Found\r\n\
         Location: /login\r\n\
         Content-Length: 0\r\n\
         \r\n"
}
