mod actions;
mod client;
use std::{env, fs};
use std::process;

use actions::log::LogAction;
fn main() {
    fs::create_dir("./.logs").unwrap_or_else(|e| {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => {}
            _ => {
                eprintln!("couldn't initialize the app (error writing logs directory)");
            }
        }
    });
    let args = env::args().skip(1).collect::<Vec<_>>();
    // if command not provided
    if args.get(0).is_none() {
        not_enough_arg_err();
    }

    if args[0] == "query" {
        // if client's name not provided
        if args.get(1).is_none() {
            not_enough_arg_err();
        }
        actions::query(args[1].as_str());
        return;
    }
    if args[0] == "log" {
        // if the length of argument is less than 5
        if args.get(4).is_none() {
            not_enough_arg_err();
        }
        let log_action = LogAction::from_args(&args[2..]);
        actions::log(args[1].as_str(), log_action);
        return;
    }
    eprintln!("invalid arguments");
    process::exit(1);
}

fn not_enough_arg_err() {
    eprintln!("not enough arguments");
    process::exit(1);
}
