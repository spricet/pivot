pub mod builtin;
pub mod poverride;

use crate::prompt::builtin::BuiltinPrompt;
use crate::prompt::poverride::OverridePrompt;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Prompt {
    Builtin(BuiltinPrompt),
    Override(OverridePrompt),
}

impl Validate for Prompt {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            Prompt::Builtin(b) => b.validate(),
            Prompt::Override(o) => o.validate(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prompt::builtin::BuiltinPrompt;
    use crate::prompt::poverride::OverridePrompt;
    use crate::prompt::Prompt;
    use validator::Validate;

    #[test]
    fn test_prompt_validators() {
        let p = Prompt::Builtin(BuiltinPrompt {
            prefix: "asdf".to_string(),
        });
        assert!(p.validate().is_ok());

        let p = Prompt::Builtin(BuiltinPrompt {
            prefix: "".to_string(),
        });
        assert!(p.validate().is_err());

        let p = Prompt::Override(OverridePrompt {
            prompt_override: "asdf".to_string(),
        });
        assert!(p.validate().is_ok());

        let p = Prompt::Override(OverridePrompt {
            prompt_override: "".to_string(),
        });
        assert!(p.validate().is_err());
    }
}
