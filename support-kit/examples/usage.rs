pub fn main() {
    use clap::Parser;

    #[derive(Parser)]
    enum ServiceCommand {
        Install,
        Uninstall,
        Start,
        Stop,
        Restart,
    }

    #[derive(Parser)]
    struct ServiceArgs {
        #[clap(subcommand)]
        command: ServiceCommand,
    }

    #[derive(Parser)]
    enum Commands {
        Service(ServiceArgs),
    }

    #[derive(Parser)]
    struct SupportArgs {
        #[clap(subcommand)]
        command: Commands,
    }

    #[derive(Parser)]
    struct ConsumerArgs {
        #[clap(subcommand)]
        command: ConsumerCommands,
    }

    #[derive(Parser)]
    enum ConsumerCommands {
        Local(LocalArgs),
        Support(SupportArgs),
    }

    #[derive(Parser)]
    struct LocalArgs {
        #[clap(subcommand)]
        command: LocalCommand,
    }

    #[derive(Parser)]
    enum LocalCommand {
        Debug,
    }

    ConsumerArgs::parse();
}
