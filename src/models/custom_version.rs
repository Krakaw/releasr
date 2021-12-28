use semver::{Error, Version};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CustomVersion(pub Version);

impl CustomVersion {
    pub fn parse(s: &str) -> Result<Self, Error> {
        let s = if s.starts_with("v") { &s[1..] } else { s };
        let v = Version::parse(s)?;
        Ok(CustomVersion(v))
    }
}
impl Display for CustomVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0.major, self.0.minor, self.0.patch)
    }
}

impl From<CustomVersion> for f64 {
    fn from(custom_version: CustomVersion) -> Self {
        let version = custom_version.0;
        let version_int =
            (version.major.pow(9) + version.minor.pow(6) + version.patch.pow(3)) as f64;
        version_int
    }
}
