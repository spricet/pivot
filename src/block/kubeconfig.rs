use crate::block::error::Result;
use crate::block::{Block, BlockHandler};
use crate::switch::command::SwitcherCommand;
use serde::{Deserialize, Serialize};
use validator::Validate;

const KUBECONFIG_ENV: &str = "KUBECONFIG";

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct KubeconfigBlock {
    #[validate(length(min = 1, max = 300))]
    pub kubeconfig: String,
}

pub struct KubeconfigBlockHandler {}

impl KubeconfigBlockHandler {
    pub fn new() -> KubeconfigBlockHandler {
        KubeconfigBlockHandler {}
    }
}

impl Default for KubeconfigBlockHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockHandler for KubeconfigBlockHandler {
    fn handle(&self, block: &Block, cmd: &mut Box<dyn SwitcherCommand>) -> Result<()> {
        if let Block::Kubeconfig(kblock) = block {
            cmd.env(KUBECONFIG_ENV, &shellexpand::tilde(&kblock.kubeconfig));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::block::kubeconfig::{KubeconfigBlock, KubeconfigBlockHandler, KUBECONFIG_ENV};
    use crate::block::{Block, BlockHandler};
    use crate::switch::command::SwitcherCommand;
    use crate::test_util::MockSwitcherCommand;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use validator::Validate;

    #[test]
    fn test_validation() {
        let k = KubeconfigBlock {
            kubeconfig: "".to_string(),
        };
        assert!(k.validate().is_err());

        let k = KubeconfigBlock {
            kubeconfig: "asdf".to_string(),
        };
        assert!(k.validate().is_ok());

        let k = KubeconfigBlock {
            kubeconfig: crate::test_util::gen_str(300, 'x'),
        };
        assert!(k.validate().is_ok());

        let k = KubeconfigBlock {
            kubeconfig: crate::test_util::gen_str(301, 'x'),
        };
        assert!(k.validate().is_err());
    }

    #[test]
    fn test_kubeconfig_env_set() {
        let k = Block::Kubeconfig(KubeconfigBlock {
            kubeconfig: "src".to_string(),
        });
        let h = KubeconfigBlockHandler::default();

        let env: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
        let cmd = MockSwitcherCommand::new(Arc::clone(&env));
        let mut boxed = Box::new(cmd) as Box<dyn SwitcherCommand>;
        let res = h.handle(&k, &mut boxed);

        assert!(res.is_ok());
        {
            let e = env.lock().unwrap();
            assert_eq!(e.get(KUBECONFIG_ENV).unwrap().clone(), "src")
        }
    }
}
