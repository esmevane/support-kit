use clap::Parser;

fn main() {
    let args = support_kit::Args::parse();

    println!("{:?}", args);
    println!("{:?}", args.config());
}
