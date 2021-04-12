use crate::prelude::*;
use nu_engine::WholeStreamCommand;
use nu_errors::ShellError;
use nu_protocol::{Signature, SyntaxShape};
use nu_source::Tagged;
use std::path::PathBuf;

pub struct Exec;

#[derive(Deserialize)]
pub struct ExecArgs {
    pub command: Tagged<PathBuf>,
    pub rest: Vec<Tagged<String>>,
}

impl WholeStreamCommand for Exec {
    fn name(&self) -> &str {
        "exec"
    }

    fn signature(&self) -> Signature {
        Signature::build("exec")
            .required("command", SyntaxShape::FilePath, "the command to execute")
            .rest(
                SyntaxShape::GlobPattern,
                "any additional arguments for command",
            )
    }

    fn usage(&self) -> &str {
        "Execute command."
    }

    fn run_with_actions(&self, args: CommandArgs) -> Result<ActionStream, ShellError> {
        exec(args)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Execute 'ps aux'",
                example: "exec ps aux",
                result: None,
            },
            Example {
                description: "Execute 'nautilus'",
                example: "exec nautilus",
                result: None,
            },
        ]
    }
}

#[cfg(unix)]
fn exec(args: CommandArgs) -> Result<ActionStream, ShellError> {
    use std::os::unix::process::CommandExt;
    use std::process::Command;

    let name = args.call_info.name_tag.clone();
    let (args, _): (ExecArgs, _) = args.process()?;

    let mut command = Command::new(args.command.item);
    for tagged_arg in args.rest {
        command.arg(tagged_arg.item);
    }

    let err = command.exec(); // this replaces our process, should not return

    Err(ShellError::labeled_error(
        "Error on exec",
        format!("{}", err),
        &name,
    ))
}

#[cfg(not(unix))]
fn exec(args: CommandArgs) -> Result<ActionStream, ShellError> {
    Err(ShellError::labeled_error(
        "Error on exec",
        "exec is not supported on your platform",
        &args.call_info.name_tag,
    ))
}
