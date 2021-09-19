use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OverridePrompt {
    #[serde(rename = "override")]
    #[validate(length(min = 1, max = 200))]
    pub prompt_override: String,
}

#[cfg(test)]
mod tests {
    use crate::prompt::poverride::OverridePrompt;
    use validator::Validate;

    #[test]
    fn test_validate_prefix_length() {
        let mut o = OverridePrompt {
            prompt_override: "asdf$".to_string(),
        };
        assert!(o.validate().is_ok());

        o.prompt_override = "".to_string();
        assert!(o.validate().is_err());

        o.prompt_override = "a".to_string();
        assert!(o.validate().is_ok());

        o.prompt_override = "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec qu".to_string();
        assert!(o.validate().is_ok());

        o.prompt_override = "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec que".to_string();
        assert!(o.validate().is_err());
    }
}
