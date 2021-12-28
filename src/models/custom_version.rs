use semver::{Error, Version};
use serde::{Deserialize, Serialize};

use std::fmt::{Display, Formatter};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CustomVersion(pub Version);

impl CustomVersion {
    pub fn parse(s: &str) -> Result<Self, Error> {
        let s = s.strip_prefix('v').unwrap_or(s);
        let v = Version::parse(s)?;
        Ok(CustomVersion(v))
    }
}
impl Display for CustomVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0.major, self.0.minor, self.0.patch)
    }
}

impl From<CustomVersion> for i64 {
    fn from(custom_version: CustomVersion) -> Self {
        let version = custom_version.0;
        (version.major.pow(9) + version.minor.pow(6) + version.patch.pow(3)) as i64
    }
}
