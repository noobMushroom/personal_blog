pub fn parse_body(req: &str) -> Option<String> {
    req.split("\r\n\r\n").nth(1).map(|s| s.to_string())
}
