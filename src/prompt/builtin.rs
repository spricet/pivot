use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BuiltinPrompt {
    #[validate(length(min = 1, max = 30))]
    pub prefix: String,
}

#[cfg(test)]
mod tests {
    use crate::prompt::builtin::BuiltinPrompt;
    use validator::Validate;

    #[test]
    fn test_validate_prefix_length() {
        let mut p = BuiltinPrompt {
            prefix: "123".to_string(),
        };
        assert!(p.validate().is_ok());
        p.prefix = "".to_string();
        assert!(p.validate().is_err());
    }
}
