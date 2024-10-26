use crate::ShellCommandError;

#[derive(Clone, Debug)]
pub struct ShellCommand {
    pub command: String,
    pub args: Vec<String>,
}

impl ShellCommand {
    #[tracing::instrument(skip(self), level = "trace")]
    pub fn run(&self) -> Result<(), ShellCommandError> {
        tracing::trace!(command = ?self.command_and_args(), "running shell command");
        std::process::Command::new(&self.command)
            .args(&self.args)
            .spawn()?
            .wait()?;

        Ok(())
    }

    pub fn command_and_args(&self) -> Vec<String> {
        let mut command = vec![self.command.clone()];
        command.extend(self.args.clone());
        command
    }
}

impl From<ShellCommand> for Vec<ShellCommand> {
    fn from(process: ShellCommand) -> Self {
        vec![process]
    }
}

impl TryFrom<String> for ShellCommand {
    type Error = ShellCommandError;

    fn try_from(command: String) -> Result<Self, Self::Error> {
        let command: &str = command.as_ref();
        command.try_into()
    }
}

impl TryFrom<&str> for ShellCommand {
    type Error = ShellCommandError;

    fn try_from(command: &str) -> Result<Self, Self::Error> {
        let command_chunks: Vec<String> = command.split_whitespace().map(String::from).collect();

        if let Some((command, args)) = command_chunks.split_first() {
            Ok(Self {
                command: command.clone(),
                args: args.to_vec(),
            })
        } else {
            Err(ShellCommandError::MalformedError(command.to_string()))
        }
    }
}

#[tracing::instrument(skip(shell_command), level = "trace")]
pub fn shell<T: Into<String>>(shell_command: T) -> crate::Result<ShellCommand> {
    let shell_command = shell_command.into();

    tracing::trace!(shell_command = ?shell_command, "converting to shell command");

    Ok(shell_command.try_into()?)
}
