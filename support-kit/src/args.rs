use clap::Parser;

use crate::{Config, VerbosityLevel};

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbosity: u8,
}

impl Args {
    pub fn verbosity_level(&self) -> Option<VerbosityLevel> {
        let verbosity = self.verbosity;
        VerbosityLevel::from_repr(verbosity as usize)
    }

    pub fn as_config(self) -> Config {
        Config::builder()
            .maybe_verbosity(self.verbosity_level())
            .build()
    }
}

#[test]
fn setting_verbosity_with_args() -> Result<(), Box<dyn std::error::Error>> {
    let expectations = [
        ("app", VerbosityLevel::Off),
        ("app -v", VerbosityLevel::Error),
        ("app -vv", VerbosityLevel::Warn),
        ("app -vvv", VerbosityLevel::Info),
        ("app -vvvv", VerbosityLevel::Debug),
        ("app -vvvvv", VerbosityLevel::Trace),
    ];

    for (input, expected) in expectations.iter() {
        let args = Args::try_parse_from(input.split_whitespace())?;

        assert_eq!(
            args.as_config(),
            Config::builder().verbosity(*expected).build()
        );
    }
    Ok(())
}
