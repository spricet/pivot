use crate::opt::PivotOpt;
use pivot::block::aws_profile::AwsProfileBlock;
use pivot::block::kubeconfig::KubeconfigBlock;
use pivot::block::Block;
use pivot::config::v1alpha::Config;
use pivot::pinit::PostInit;
use pivot::prompt::builtin::BuiltinPrompt;
use pivot::prompt::Prompt;
use pivot::switch::Switcher;
use pivot::target::Target;
use structopt::StructOpt;

mod opt;

fn main() {
    let opts: PivotOpt = PivotOpt::from_args();

    let cfg = Config {
        api_version: "v1".to_string(),
        targets: vec![Target {
            name: "test".to_string(),
            post_init: Some(PostInit {
                prompt: Prompt::Builtin(BuiltinPrompt {
                    prefix: "pivot".to_string(),
                }),
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
