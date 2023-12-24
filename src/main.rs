use clap::{Command, Arg, ArgAction};


fn main () {
    let matches = Command::new("alta")
        .about("A CLI for managing self-hosted products developed by altadena")
        .version("0.1.0")
        .author("lyricalsoul")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("install")
                .about("Install a product")
                .arg(
                    Arg::new("product")
                        .long("product")
                        .short('p')
                        .help("The product to install")
                        .num_args(1..)
                        .action(ArgAction::Set)
}