use crate::prompt::Prompt;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct PostInit {
    #[validate]
    pub prompt: Prompt,

    #[validate(length(min = 1, max = 500))]
    pub start_dir: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::pinit::PostInit;
    use crate::prompt::builtin::BuiltinPrompt;
    use crate::prompt::Prompt;
    use validator::Validate;

    #[test]
    fn test_validation() {
        let mut p = PostInit {
            prompt: Prompt::Builtin(BuiltinPrompt {
                prefix: "".to_string(),
            }), // bad prompt
            start_dir: None,
        };
        assert!(p.validate().is_err());

        p.prompt = Prompt::Builtin(BuiltinPrompt {
            prefix: "asdf".to_string(),
        });
        assert!(p.validate().is_ok());

        p.start_dir = Some("".to_string());
        assert!(p.validate().is_err());

        p.start_dir = Some("123".to_string());
        assert!(p.validate().is_ok());
    }
}
