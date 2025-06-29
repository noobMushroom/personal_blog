#[derive(Clone)]
pub enum Roles {
    Admin,
    Guest,
    Unknown(String),
}

impl Roles {
    pub fn from(role: &str) -> Self {
        match role {
            "admin" => Roles::Admin,
            "guest" => Roles::Guest,
            unknown => Roles::Unknown(unknown.to_string()),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Roles::Admin => "admin".to_string(),
            Roles::Guest => "guest".to_string(),
            Roles::Unknown(unknown) => unknown.to_string(),
        }
    }
}
