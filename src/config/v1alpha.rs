use crate::config::error::{ConfigError, Result};
use crate::target::Target;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

lazy_static! {
    static ref RE_API_VERSION: Regex = Regex::new(r"^v1$").unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[validate(regex = "RE_API_VERSION")]
    pub api_version: String,
    pub targets: Vec<Target>,
}

impl Config {
    pub fn get_target(&self, target_name: &str) -> Result<Target> {
        for target in self.targets.iter() {
            if target.name == target_name {
                return Ok(target.clone());
            }
        }
        Err(ConfigError::MissingTarget(target_name.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::config::v1alpha::Config;
    use crate::config::error::ConfigError;
    use validator::Validate;
    use crate::target::Target;

    #[test]
    fn test_validation() {
        let mut c = Config {
            api_version: "v1".to_string(),
            targets: vec![],
        };
        assert!(c.validate().is_ok());

        c.api_version = "".to_string();
        assert!(c.validate().is_err());

        c.api_version = "something else".to_string();
        assert!(c.validate().is_err());
    }

    #[test]
    fn test_get_target() {
        let target_name = "my-target";
        let c = Config {
            api_version: "v1".to_string(),
            targets: vec![Target{
                name: target_name.to_string(),
                blocks: vec![],
                env: Default::default(),
                post_init: None
            }],
        };
        assert!(c.get_target(target_name).is_ok());

        let res = c.get_target("bogus");
        assert!(res.is_err());
        if let Err(e) = res {
            assert!(matches!(e, ConfigError::MissingTarget(x) if x == "bogus"));
        }
    }
}
