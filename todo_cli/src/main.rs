use clap::{Arg, Command};

fn main() {
    let matches = Command::new("To-Do List CLI")
        .version("1.0")
        .author("testing@gmail.com")
        .about("Manages your to-do list")
        .subcommand(
            Command::new("add")
                .about("Adds a new task")
                .arg(Arg::new("task").required(true).help("The task description")),
        )
        .subcommand(Command::new("list").about("Lists all tasks"))
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let task = sub_m.get_one::<String>("task").unwrap();
            println!("Adding task: {}", task);
        }
        Some(("list", _)) => {
            println!("Listing tasks...");
        }
        _ => println!("Use `add` or `list` commands."),
    }
}
