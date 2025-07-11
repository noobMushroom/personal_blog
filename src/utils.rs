use url_encor::Encoder;

pub fn extract_from_string(body: &str, key: &str) -> Option<String> {
    body.split('&').find_map(|value| {
        let mut parts = value.splitn(2, '=');
        match (parts.next(), parts.next()) {
            (Some(k), Some(v)) if k == key => Some(v.to_string().url_decode()),
            _ => None,
        }
    })
}
