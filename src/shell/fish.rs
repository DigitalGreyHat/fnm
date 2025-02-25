use super::shell::Shell;
use indoc::indoc;
use std::path::Path;

#[derive(Debug)]
pub struct Fish;

impl Shell for Fish {
    fn to_structopt_shell(&self) -> structopt::clap::Shell {
        structopt::clap::Shell::Fish
    }

    fn path(&self, path: &Path) -> String {
        format!("set -gx PATH {:?} $PATH;", path.to_str().unwrap())
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("set -gx {name} {value:?};", name = name, value = value)
    }

    fn use_on_cd(&self, _config: &crate::config::FnmConfig) -> String {
        indoc!(
            r#"
                function _fnm_autoload_hook --on-variable PWD --description 'Change Node version on directory change'
                    status --is-command-substitution; and return
                    if test -f .node-version -o -f .nvmrc
                        fnm use
                    end
                end

                _fnm_autoload_hook
            "#
        )
        .into()
    }
}
