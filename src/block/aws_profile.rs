use crate::block::error::Result;
use crate::block::{Block, BlockHandler};
use crate::switch::command::SwitcherCommand;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

const AWS_PROFILE_ENV: &str = "AWS_PROFILE";

lazy_static! {
    static ref RE_AWS_PROFILE: Regex = Regex::new(r"^[a-zA-Z0-9-_]+$").unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AwsProfileBlock {
    #[validate(length(min = 1, max = 255))]
    #[validate(regex = "RE_AWS_PROFILE")]
    pub profile: String,
}

pub struct AwsProfileBlockHandler {}

impl AwsProfileBlockHandler {
    pub fn new() -> AwsProfileBlockHandler {
        AwsProfileBlockHandler {}
    }
}

impl Default for AwsProfileBlockHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockHandler for AwsProfileBlockHandler {
    fn handle(&self, block: &Block, cmd: &mut Box<dyn SwitcherCommand>) -> Result<()> {
        if let Block::AwsProfile(ablock) = block {
            log::debug!("setting AWS_PROFILE={}", &ablock.profile);
            cmd.env(AWS_PROFILE_ENV, &ablock.profile);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::block::aws_profile::{AwsProfileBlock, AwsProfileBlockHandler, AWS_PROFILE_ENV};
    use crate::block::{Block, BlockHandler};
    use crate::switch::command::SwitcherCommand;
    use crate::test_util::MockSwitcherCommand;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use validator::Validate;

    #[test]
    fn test_validation() {
        let b = AwsProfileBlock {
            profile: "".to_string(),
        };
        assert!(b.validate().is_err());

        let b = AwsProfileBlock {
            profile: "test".to_string(),
        };
        assert!(b.validate().is_ok());

        let b = AwsProfileBlock {
            profile: crate::test_util::gen_str(255, 'x'),
        };
        assert!(b.validate().is_ok());

        let b = AwsProfileBlock {
            profile: crate::test_util::gen_str(256, 'x'),
        };
        assert!(b.validate().is_err());

        let b = AwsProfileBlock {
            profile: "special$!@-124".to_string(),
        };
        assert!(b.validate().is_err());

        let b = AwsProfileBlock {
            profile: "spaces not allowed".to_string(),
        };
        assert!(b.validate().is_err());

        let b = AwsProfileBlock {
            profile: "abc123_ASD-124".to_string(),
        };
        assert!(b.validate().is_ok());
    }

    #[test]
    fn test_env_set() {
        let b = Block::AwsProfile(AwsProfileBlock {
            profile: "asdf".to_string(),
        });
        let h = AwsProfileBlockHandler::default();

        let env: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
        let cmd = MockSwitcherCommand::new(Arc::clone(&env));
        let mut boxed = Box::new(cmd) as Box<dyn SwitcherCommand>;
        let res = h.handle(&b, &mut boxed);

        assert!(res.is_ok());
        {
            let e = env.lock().unwrap();
            assert_eq!(e.get(AWS_PROFILE_ENV).unwrap().clone(), "asdf")
        }
    }
}
