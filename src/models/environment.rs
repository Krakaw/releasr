use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Environment {
    pub name: String,
    pub version_url: Option<String>,
}
