#[derive(PartialEq)]
#[derive(Debug)]
pub enum DeploymentType {
    MajorUpgrade, MinorUpgrade, PatchUpgrade, NoChange, Downgrade
}

pub struct Version {
    major: i32,
    minor: i32,
    patch: i32,
}

impl Version {
    fn from_string(exp: &str) -> Version {
        let arr:Vec<&str> = exp.split('.').collect();
        if arr.len() != 3 {
            panic!("invalid expression");
        }
        Version {
            major: arr[0].parse().unwrap(),
            minor: arr[1].parse().unwrap(),
            patch: arr[2].parse().unwrap()
        }
    }

    fn compare(&self, other: &Version) -> DeploymentType {
        if self.major > other.major {
            DeploymentType::MajorUpgrade
        } else if self.major <= other.major && self.minor > other.minor {
            DeploymentType::MinorUpgrade
        } else if self.major <= other.major && self.minor <= other.minor && self.patch > other.patch {
            DeploymentType::PatchUpgrade
        } else if self.major == other.major && self.minor == other.minor && self.patch == other.patch {
            DeploymentType::NoChange
        } else {
            DeploymentType::Downgrade
        }
    }
}

fn main() {
    let current_version = Version::from_string("1.20.5");
    let previous_version = Version::from_string("6.20.5");
    println!("upgrade type: {:?}", current_version.compare(&previous_version));
}

#[cfg(test)]
mod tests {
    use crate::{DeploymentType, Version};

    #[test]
    fn parse_version() {
        let version = Version::from_string("1.4.9");
        assert_eq!(1, version.major);
        assert_eq!(4, version.minor);
        assert_eq!(9, version.patch);
    }

    #[test]
    fn major_upgrade() {
        let previous_version = Version::from_string("1.9.0");
        let current_version = Version::from_string("2.0.0");
        assert_eq!(current_version.compare(&previous_version), DeploymentType::MajorUpgrade);
    }

    #[test]
    fn minor_upgrade() {
        let previous_version = Version::from_string("1.4.0");
        let current_version = Version::from_string("1.9.0");
        assert_eq!(current_version.compare(&previous_version), DeploymentType::MinorUpgrade);
    }

    #[test]
    fn patch_upgrade() {
        let previous_version = Version::from_string("1.9.0");
        let current_version = Version::from_string("1.9.3");
        assert_eq!(current_version.compare(&previous_version), DeploymentType::PatchUpgrade);
    }

    #[test]
    fn no_change() {
        let previous_version = Version::from_string("1.9.0");
        let current_version = Version::from_string("1.9.0");
        assert_eq!(current_version.compare(&previous_version), DeploymentType::NoChange);
    }

    #[test]
    fn downgraded() {
        let previous_version = Version::from_string("1.9.0");
        let current_version = Version::from_string("1.8.9");
        let typee: DeploymentType = current_version.compare(&previous_version);
        println!(">>>> {:?}", typee);
        assert_eq!(current_version.compare(&previous_version), DeploymentType::Downgrade);
    }
}
