use semver::{Error, Version};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

use regex::Regex;
use std::fmt::{Display, Formatter};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CustomVersion(pub Version);

impl CustomVersion {
    pub fn parse(s: &str) -> Result<Self, Error> {
        let re = Regex::new(r"^[^\d+]*(?P<v>\d.*)$").unwrap();
        let s = re.replace_all(s, "$v");
        let v = Version::parse(s.as_ref())?;
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
        let modifier = 1000000 as f64;
        let major = version.major as f64 * modifier;
        let minor = version.minor as f64 / 1000f64 * modifier;
        let patch = version.patch as f64 / 1000000f64 * modifier;
        (major + minor + patch) as i64
    }
}

impl Ord for CustomVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        let version_int: i64 = self.clone().into();
        let other_int = other.clone().into();
        version_int.cmp(&other_int)
    }
}

impl Eq for CustomVersion {}

impl PartialEq for CustomVersion {
    fn eq(&self, other: &Self) -> bool {
        let version_int: i64 = self.clone().into();
        let other_int: i64 = other.clone().into();
        version_int == other_int
    }
}

impl PartialOrd for CustomVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let version_int: i64 = self.clone().into();
        Some(version_int.cmp(&other.clone().into()))
    }
}

#[cfg(test)]
mod tests {
    use crate::models::custom_version::CustomVersion;

    #[test]
    fn strip_prefixes() {
        let version = "v1.0.4";
        let version = CustomVersion::parse(version).unwrap();
        assert_eq!(version.to_string(), "1.0.4");

        let version = "v-garbage-1.0.4";
        let version = CustomVersion::parse(version).unwrap();
        assert_eq!(version.to_string(), "1.0.4");
    }

    #[test]
    fn compare_versions() {
        let v1 = CustomVersion::parse("v1.999.5").unwrap();
        let v2 = CustomVersion::parse("v2.0.0").unwrap();
        assert!(v2 > v1);

        let v1 = CustomVersion::parse("v1.999.5").unwrap();
        let v2 = CustomVersion::parse("garbage-v5.0.0").unwrap();
        assert!(v2 > v1);

        let v1 = CustomVersion::parse("v1.999.999").unwrap();
        let v2 = CustomVersion::parse("garbage-v2.0.0").unwrap();
        assert!(v2 > v1);

        let v1 = CustomVersion::parse("v0.0.1").unwrap();
        let v2 = CustomVersion::parse("garbage-v0.0.2").unwrap();
        assert!(v2 > v1);
    }
}
