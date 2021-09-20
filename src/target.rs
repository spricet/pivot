use crate::block::Block;
use crate::pinit::PostInit;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

lazy_static! {
    static ref RE_TARGET_NAME: Regex = Regex::new(r"^[a-zA-Z0-9-_]+$").unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    #[validate(length(min = 1, max = 255))]
    #[validate(regex = "RE_TARGET_NAME")]
    pub name: String,

    #[validate]
    pub blocks: Vec<Block>,

    pub env: HashMap<String, String>,

    #[validate]
    pub post_init: Option<PostInit>,
}

#[cfg(test)]
mod tests {
    use crate::block::aws_profile::AwsProfileBlock;
    use crate::block::Block;
    use crate::pinit::PostInit;
    use crate::prompt::builtin::BuiltinPrompt;
    use crate::prompt::Prompt;
    use crate::target::Target;
    use validator::Validate;

    #[test]
    fn test_validation() {
        let mut t = Target {
            name: "test".to_string(),
            blocks: vec![],
            env: Default::default(),
            post_init: None,
        };
        assert!(t.validate().is_ok());

        t.name = "invalid !@#!@$ characters".to_string();
        assert!(t.validate().is_err());

        t.name = crate::test_util::gen_str(255, 'x');
        assert!(t.validate().is_ok());

        t.name = crate::test_util::gen_str(256, 'x');
        assert!(t.validate().is_err());

        t.name = "underbars_ok".to_string();
        assert!(t.validate().is_ok());

        t.name = "no spaces".to_string();
        assert!(t.validate().is_err());

        t.name = "asdf-ASDF".to_string();
        assert!(t.validate().is_ok());

        t.post_init = Some(PostInit {
            prompt: Prompt::Builtin(BuiltinPrompt {
                prefix: "".to_string(),
            }),
            start_dir: None,
        });
        assert!(t.validate().is_err());

        t.post_init = Some(PostInit {
            prompt: Prompt::Builtin(BuiltinPrompt {
                prefix: "asdf".to_string(),
            }),
            start_dir: None,
        });
        assert!(t.validate().is_ok());

        t.blocks = vec![Block::AwsProfile(AwsProfileBlock {
            profile: "$!@#".to_string(),
        })];
        assert!(t.validate().is_err());

        t.blocks = vec![Block::AwsProfile(AwsProfileBlock {
            profile: "profile".to_string(),
        })];
        assert!(t.validate().is_ok());
    }
}
