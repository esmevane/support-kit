use clap::Parser;

fn main() {
    let args = support_kit::Args::parse();
    let control = support_kit::SupportControl::load_configuration(&args).unwrap();

    println!("{:#?}", args);
    println!("{:#?}", control);
}
