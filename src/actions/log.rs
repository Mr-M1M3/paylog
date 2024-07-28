use std::process;

pub enum LogAction {
    Took { amount: isize, date: String },
    Gave { amount: isize, date: String },
}
impl LogAction {
    // args is a vec mapping to: [action flag (took or gave), amount, date]
    pub fn from_args(args: &[String]) -> Self {
        if args.get(2).is_none() {
            eprintln!("not enough arguments");
            process::exit(1);
        }
        if (args[0] == "took") || (args[0] == "-t") {
            Self::Took {
                amount: args[1].parse::<isize>().expect("invalid number"),
                date: args[2].clone(),
            }
        } else if (args[0] == "gave") || (args[0] == "-g") {
            Self::Gave {
                amount: args[1].parse::<isize>().expect("invalid number"),
                date: args[2].clone(),
            }
        } else {
            eprintln!("invalid arguments");
            process::exit(1);
        }
    }
}
