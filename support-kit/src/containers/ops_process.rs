use crate::OpsProcessError;

#[derive(Clone, Debug)]
pub struct OpsProcess {
    pub command: String,
    pub args: Vec<String>,
}

impl OpsProcess {
    pub fn run(&self) -> Result<(), OpsProcessError> {
        std::process::Command::new(&self.command)
            .args(&self.args)
            .spawn()?
            .wait()?;

        Ok(())
    }

    pub fn command(&self) -> Vec<String> {
        let mut command = vec![self.command.clone()];
        command.extend(self.args.clone());
        command
    }
}

impl TryFrom<String> for OpsProcess {
    type Error = OpsProcessError;

    fn try_from(command: String) -> Result<Self, Self::Error> {
        let command: &str = command.as_ref();
        command.try_into()
    }
}

impl TryFrom<&str> for OpsProcess {
    type Error = OpsProcessError;

    fn try_from(command: &str) -> Result<Self, Self::Error> {
        let command_chunks: Vec<String> = command.split_whitespace().map(String::from).collect();

        if let Some((command, args)) = command_chunks.split_first() {
            Ok(Self {
                command: command.clone(),
                args: args.to_vec(),
            })
        } else {
            Err(OpsProcessError::MalformedCommand(command.to_string()))
        }
    }
}
