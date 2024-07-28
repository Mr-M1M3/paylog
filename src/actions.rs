pub mod log;
use log::LogAction;

use crate::client::Client;
use std::{fs, io, path::Path, process};

fn read_log(client_name: &str) -> io::Result<String> {
    let mut path = Path::new("./.logs").join(client_name);
    path.set_extension("paylog");
    fs::read_to_string(path)
}
pub fn query(client_name: &str) {
    let client_data = read_log(client_name);
    match client_data {
        Ok(d) => {
            let client = Client::from_string(client_name, d);
            println!("{}", client.display());
        }
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                eprintln!("no log found");
                process::exit(1);
            }
            _ => {
                eprintln!("error reading file");
                process::exit(1);
            }
        },
    }
}
pub fn log(name: &str, action: LogAction) {
    let client_data = read_log(name);
    let mut client: Client;
    match client_data {
        Ok(v) => {
            client = Client::from_string(name, v);
        }
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                client = Client::new(name);
            }
            _ => {
                eprintln!("error reading data");
                process::exit(1);
            }
        },
    }
    match action {
        LogAction::Took { amount, date } => {
            client.log(date.as_str(), amount);
        }
        LogAction::Gave { amount, date } => {
            client.log(date.as_str(), amount * -1);
        }
    }
    let mut path = Path::new("./.logs").join(name);
    path.set_extension("paylog");
    fs::write(path, client.export()).unwrap_or_else(|e| {
        eprintln!("{:?}", e);
        eprintln!("error writing file");
        process::exit(1);
    });
    println!("Done ✅ \nNew Entries:\n{}", client.display());
}
