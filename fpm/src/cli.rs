use clap::{command, Arg, ArgAction, Command};

fn cli() -> Command {
    command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands(vec![subcommand_new()])
}

fn subcommand_new() -> Command {
    Command::new("new").about("Create a New Project").args(&[
        Arg::new("name").short('n').long("name").help("Project Name"),
        Arg::new("desc").long("desc").help("Description of the project"),
        Arg::new("directory")
            .short('d')
            .long("directory")
            .help("The directory to place the project in. If nothing is provided a directory will be generated"),
        Arg::new("tags")
            .long("tag")
            .num_args(1..)
            .action(ArgAction::Append)
            .help("Tags for the project"),
        Arg::new("language")
            .short('l')
            .long("language")
            .help("Primary programming language used"),
        Arg::new("category")
            .short('c')
            .long("category")
            .help("Used to keep similar project types together. I.E. `work`, `thirdparty`, etc"),
        Arg::new("interactive")
            .short('i')
            .long("interactive")
            .action(ArgAction::SetTrue),
    ])
}

pub fn parse() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").cloned();
            let desc = sub_matches.get_one::<String>("desc").cloned();
            let tags = sub_matches
                .get_many::<String>("tags")
                .into_iter()
                .flatten()
                .cloned()
                .collect::<Vec<_>>();
            let language = sub_matches.get_one::<String>("language").cloned();
            let category = sub_matches.get_one::<String>("category").cloned();
            let interactive = sub_matches.get_flag("interactive");

            println!("Name: {name:?}");
            println!("Desc: {desc:?}");
            println!("Tags: {tags:?}");
            println!("Language: {language:?}");
            println!("Category: {category:?}");
            println!("Interactive: {interactive:?}");
        },
        _ => unreachable!(),
    }
}
