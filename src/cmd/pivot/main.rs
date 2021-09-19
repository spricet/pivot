use crate::opt::PivotOpt;
use pivot::config::aws::AwsProfileBlock;
use pivot::config::kubernetes::KubeconfigBlock;
use pivot::config::prompt::BuiltinPrompt;
use pivot::config::{Block, Config, Prompt, Target, PostInit};
use pivot::switch::Switcher;
use structopt::StructOpt;

mod opt;

fn main() {
    let opts: PivotOpt = PivotOpt::from_args();

    let cfg = Config {
        api_version: "0.1.0".to_string(),
        targets: vec![Target {
            name: "test".to_string(),
            post_init: Some(PostInit{
                prompt: Prompt::Builtin(BuiltinPrompt { prefix: "pivot".to_string() }),
                // start_dir: None,
                start_dir: Some("~/Development".to_string()),
            }),
            blocks: vec![
                Block::AwsProfile(AwsProfileBlock {
                    profile: "my-profile".to_string(),
                }),
                Block::Kubeconfig(KubeconfigBlock {
                    kubeconfig: "~/.kube/my-config".to_string(),
                }),
            ],
            env: Default::default(),
        }],
    };
    let switcher = Switcher::new(cfg);
    match switcher.switch(&opts.target) {
        Ok(_) => {}
        Err(e) => println!("command failed: {:?}", e),
    }
}
