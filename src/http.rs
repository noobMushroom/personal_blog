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
         Location: /dashboard\r\n\
         Set-Cookie: session_id={}; Path=/; HttpOnly; SameSite=Strict\r\n\
         Content-Length: 0\r\n\
         \r\n",
        session_id
    )
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
